#[cfg(feature = "events")]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RequestMessage<'a, P: serde::Serialize + 'a> {
    api: &'a str,
    method: &'a str,
    params: P,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<&'a str>,
}

#[cfg(feature = "events")]
impl<'a, P: serde::Serialize + 'a> RequestMessage<'a, P> {
    pub(crate) fn new(api: &'a str, method: &'a str, params: P, api_key: Option<&'a str>) -> Self {
        Self {
            api,
            method,
            params,
            api_key,
        }
    }
}

// Message protocol:
// 1. u8 specifying number of additional data segments
// 2. Varint specifying length of JSON data
// 3. One varint for each data segment, specifying the length of data segment
// 4. JSON data
// 5. Data segments
pub(crate) fn serialize_for_http<P: serde::Serialize>(
    params: P,
    data: &[impl AsRef<[u8]>],
) -> Vec<u8> {
    let msg_buf = serde_json::to_vec(&params).unwrap();

    let total_length = 1
        + super::varint::get_varint_u64_len(msg_buf.len() as u64) as usize
        + data
            .iter()
            .map(|x| super::varint::get_varint_u64_len(x.as_ref().len() as u64) as usize)
            .sum::<usize>()
        + msg_buf.len()
        + data.iter().map(|x| x.as_ref().len()).sum::<usize>();

    let mut ret = Vec::with_capacity(total_length);
    ret.push(
        data.len()
            .try_into()
            .expect("Cannot send more than u8::MAX data segments in one request."),
    );
    super::varint::append_varint_u64(msg_buf.len() as u64, &mut ret);
    for segment in data {
        super::varint::append_varint_u64(segment.as_ref().len() as u64, &mut ret);
    }
    ret.extend_from_slice(&msg_buf);
    for segment in data {
        ret.extend_from_slice(segment.as_ref());
    }

    ret
}

#[cfg(feature = "events")]
// Message protocol:
// 1. u32 id
// 2. u8 specifying number of additional data segments
// 3. Varint specifying length of JSON data
// 4. One varint for each data segment, specifying the length of data segment
// 5. JSON data
// 6. Data segments
pub(crate) fn serialize_for_websocket<P: serde::Serialize>(
    id: u32,
    message: RequestMessage<'_, P>,
    data: &[impl AsRef<[u8]>],
) -> Vec<u8> {
    let msg_buf = serde_json::to_vec(&message).unwrap();

    let total_length = 5
        + super::varint::get_varint_u64_len(msg_buf.len() as u64) as usize
        + data
            .iter()
            .map(|x| super::varint::get_varint_u64_len(x.as_ref().len() as u64) as usize)
            .sum::<usize>()
        + msg_buf.len()
        + data.iter().map(|x| x.as_ref().len()).sum::<usize>();

    let mut ret = Vec::with_capacity(total_length);
    ret.extend_from_slice(&id.to_be_bytes());
    ret.push(
        data.len()
            .try_into()
            .expect("Cannot send more than u8::MAX data segments in one request."),
    );
    super::varint::append_varint_u64(msg_buf.len() as u64, &mut ret);
    for segment in data {
        super::varint::append_varint_u64(segment.as_ref().len() as u64, &mut ret);
    }
    ret.extend_from_slice(&msg_buf);
    for segment in data {
        ret.extend_from_slice(segment.as_ref());
    }

    ret
}

// Message protocol:
// 1. Varint specifying length of JSON data
// 2. JSON data
// Repeated:
// 3. Varint encoding length of next data segment
// 4. next data segment
pub(crate) fn deserialize_for_http(
    data: bytes::Bytes,
) -> Result<(bytes::Bytes, Vec<bytes::Bytes>), ()> {
    if data.is_empty() {
        return Err(());
    }
    let v_length = super::varint::get_serialized_varint_u64_len(&data) as usize;
    if data.len() < v_length {
        return Err(());
    }
    let (length, _) = super::varint::deserialize_varint_u64(&data);
    let length = length as usize;

    if data.len() < v_length + length {
        return Err(());
    }

    let first_segment = data.slice(v_length..v_length + length);

    let mut additional_segments = Vec::new();
    let mut pos = v_length + length;
    while pos < data.len() {
        let segment_v_length = super::varint::get_serialized_varint_u64_len(&data[pos..]) as usize;
        if data.len() < pos + segment_v_length {
            return Err(());
        }
        let (segment_length, _) = super::varint::deserialize_varint_u64(&data[pos..]);
        let segment_length = segment_length as usize;

        if data.len() < pos + segment_v_length + segment_length {
            return Err(());
        }

        additional_segments
            .push(data.slice(pos + segment_v_length..pos + segment_v_length + segment_length));
        pos += segment_length + segment_v_length;
    }

    Ok((first_segment, additional_segments))
}

#[cfg(feature = "events")]
pub enum RpcResponseOrEvent {
    RpcResponse(u32),
    Event(bytes::Bytes),
}

#[cfg(feature = "events")]
// Message protocol:
// 1. u8 literal 0 if result, 1 if event
// 2. If result: u32 id
// 3: If event: u8 specifying length of api string
// 4. If event: api string
// 5. Varint specifying length of JSON data
// 6. JSON data
// Repeated:
// 7. Varint encoding length of next data segment
// 8. next data segment
pub(crate) fn deserialize_for_websocket(
    data: bytes::Bytes,
) -> Result<(RpcResponseOrEvent, bytes::Bytes, Vec<bytes::Bytes>), ()> {
    if data.is_empty() {
        return Err(());
    }

    let first_byte = data[0];
    let mut pos = 1;
    let rpc_response_or_event = if first_byte == 0 {
        // Response message
        if data.len() < pos + 4 {
            return Err(());
        }
        let id = u32::from_be_bytes(data[pos..pos + 4].try_into().unwrap());
        pos += 4;

        RpcResponseOrEvent::RpcResponse(id)
    } else {
        if data.len() < pos + 1 {
            return Err(());
        }
        let api_length = data[pos] as usize;
        pos += 1;

        if data.len() < pos + api_length {
            return Err(());
        }
        let api_bytes = data.slice(pos..pos + api_length);
        pos += api_length;

        RpcResponseOrEvent::Event(api_bytes)
    };

    if data.len() < pos + 1 {
        return Err(());
    }

    let v_length = super::varint::get_serialized_varint_u64_len(&data[pos..]) as usize;
    if data.len() < pos + v_length {
        return Err(());
    }
    let (length, _) = super::varint::deserialize_varint_u64(&data[pos..]);
    let length = length as usize;

    pos += v_length;

    if data.len() < pos + length {
        return Err(());
    }

    let first_segment = data.slice(pos..pos + length);
    pos += length;

    let mut additional_segments = Vec::new();
    while pos < data.len() {
        let segment_v_length = super::varint::get_serialized_varint_u64_len(&data[pos..]) as usize;
        if data.len() < pos + segment_v_length {
            return Err(());
        }
        let (segment_length, _) = super::varint::deserialize_varint_u64(&data[pos..]);
        let segment_length = segment_length as usize;

        pos += segment_v_length;

        if data.len() < pos + segment_length {
            return Err(());
        }

        additional_segments.push(data.slice(pos..pos + segment_length));
        pos += segment_length;
    }

    Ok((rpc_response_or_event, first_segment, additional_segments))
}
