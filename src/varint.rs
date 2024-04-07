pub(crate) fn get_serialized_varint_u64_len(data: &[u8]) -> u8 {
    let first = data[0];
    match first {
        253 => 3,
        254 => 5,
        255 => 9,
        _ => 1,
    }
}

pub(crate) fn get_varint_u64_len(value: u64) -> u8 {
    if value < 253 {
        1
    } else if value < 2u64.pow(16) {
        3
    } else if value < 2u64.pow(32) {
        5
    } else {
        9
    }
}

pub(crate) fn append_varint_u64(value: u64, data: &mut Vec<u8>) {
    if value < 253 {
        data.push(value as u8);
    } else if value < 2u64.pow(16) {
        data.reserve(3);
        data.push(253);
        data.extend_from_slice(&(value as u16).to_be_bytes());
    } else if value < 2u64.pow(32) {
        data.reserve(5);
        data.push(254);
        data.extend_from_slice(&(value as u32).to_be_bytes());
    } else {
        data.reserve(9);
        data.push(255);
        data.extend_from_slice(&value.to_be_bytes());
    }
}

pub(crate) fn deserialize_varint_u64(data: &[u8]) -> (u64, u8) {
    let first = data[0];
    match first {
        253 => (u16::from_be_bytes(data[1..3].try_into().unwrap()) as u64, 3),
        254 => (u32::from_be_bytes(data[1..5].try_into().unwrap()) as u64, 5),
        255 => (u64::from_be_bytes(data[1..9].try_into().unwrap()), 9),
        _ => (first as u64, 1),
    }
}

#[test]
fn test_varint_u64() {
    fn inner_test(val: u64, expected_val: Vec<u8>) {
        let mut serialized = vec![];
        append_varint_u64(val, &mut serialized);
        println!("serialized {} into {:?}", val, serialized);
        assert_eq!(serialized.len(), get_varint_u64_len(val) as usize);
        assert_eq!(serialized, expected_val);
        let deserialized = deserialize_varint_u64(&serialized);
        assert_eq!(deserialized.1 as usize, serialized.len());
        assert_eq!(deserialized.0, val);
        assert_eq!(
            get_serialized_varint_u64_len(&serialized) as usize,
            expected_val.len()
        );
        assert_eq!(get_varint_u64_len(val) as usize, expected_val.len());
    }

    inner_test(0, vec![0]);
    inner_test(15, vec![15]);
    inner_test(252, vec![252]);
    inner_test(253, vec![253, 0, 253]);
    inner_test(65535, vec![253, 255, 255]);
    inner_test(65536, vec![254, 0, 1, 0, 0]);
    inner_test(4294967295, vec![254, 255, 255, 255, 255]);
    inner_test(4294967296, vec![255, 0, 0, 0, 1, 0, 0, 0, 0]);
    inner_test(
        18446744073709551615,
        vec![255, 255, 255, 255, 255, 255, 255, 255, 255],
    );
}
