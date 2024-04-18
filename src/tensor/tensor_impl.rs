use super::{DecthingsElementAudio, DecthingsElementImage, DecthingsElementVideo};
use byte_slice_cast::{AsByteSlice, AsMutByteSlice, AsSliceOf, FromByteSlice, ToMutByteSlice};
use ndarray::{Array, ArrayView, CowArray, IxDyn};

#[derive(Debug)]
pub enum DeserializeDecthingsTensorError {
    UnexpectedEndOfBytes,
    InvalidBytes(String),
}

const TYPE_SPEC_F32: u8 = 1;
const TYPE_SPEC_F64: u8 = 2;
const TYPE_SPEC_I8: u8 = 3;
const TYPE_SPEC_I16: u8 = 4;
const TYPE_SPEC_I32: u8 = 5;
const TYPE_SPEC_I64: u8 = 6;
const TYPE_SPEC_U8: u8 = 7;
const TYPE_SPEC_U16: u8 = 8;
const TYPE_SPEC_U32: u8 = 9;
const TYPE_SPEC_U64: u8 = 10;
const TYPE_SPEC_STRING: u8 = 11;
const TYPE_SPEC_BINARY: u8 = 12;
const TYPE_SPEC_BOOLEAN: u8 = 13;
const TYPE_SPEC_IMAGE: u8 = 14;
const TYPE_SPEC_AUDIO: u8 = 15;
const TYPE_SPEC_VIDEO: u8 = 16;

#[derive(Debug, Clone)]
pub enum DecthingsTensor<'a> {
    F32(CowArray<'a, f32, IxDyn>),
    F64(CowArray<'a, f64, IxDyn>),
    I8(CowArray<'a, i8, IxDyn>),
    I16(CowArray<'a, i16, IxDyn>),
    I32(CowArray<'a, i32, IxDyn>),
    I64(CowArray<'a, i64, IxDyn>),
    U8(CowArray<'a, u8, IxDyn>),
    U16(CowArray<'a, u16, IxDyn>),
    U32(CowArray<'a, u32, IxDyn>),
    U64(CowArray<'a, u64, IxDyn>),
    String(CowArray<'a, &'a str, IxDyn>),
    Binary(CowArray<'a, &'a [u8], IxDyn>),
    Boolean(CowArray<'a, bool, IxDyn>),
    Image(CowArray<'a, DecthingsElementImage<'a>, IxDyn>),
    Audio(CowArray<'a, DecthingsElementAudio<'a>, IxDyn>),
    Video(CowArray<'a, DecthingsElementVideo<'a>, IxDyn>),
}

impl<'a> DecthingsTensor<'a> {
    pub fn view(&'a self) -> Self {
        match self {
            Self::F32(inner) => DecthingsTensor::F32(inner.view().into()),
            Self::F64(inner) => DecthingsTensor::F64(inner.view().into()),
            Self::I8(inner) => DecthingsTensor::I8(inner.view().into()),
            Self::I16(inner) => DecthingsTensor::I16(inner.view().into()),
            Self::I32(inner) => DecthingsTensor::I32(inner.view().into()),
            Self::I64(inner) => DecthingsTensor::I64(inner.view().into()),
            Self::U8(inner) => DecthingsTensor::U8(inner.view().into()),
            Self::U16(inner) => DecthingsTensor::U16(inner.view().into()),
            Self::U32(inner) => DecthingsTensor::U32(inner.view().into()),
            Self::U64(inner) => DecthingsTensor::U64(inner.view().into()),
            Self::String(inner) => DecthingsTensor::String(inner.view().into()),
            Self::Binary(inner) => DecthingsTensor::Binary(inner.view().into()),
            Self::Boolean(inner) => DecthingsTensor::Boolean(inner.view().into()),
            Self::Image(inner) => DecthingsTensor::Image(inner.view().into()),
            Self::Audio(inner) => DecthingsTensor::Audio(inner.view().into()),
            Self::Video(inner) => DecthingsTensor::Video(inner.view().into()),
        }
    }

    pub fn shape(&self) -> &[usize] {
        match self {
            Self::F32(inner) => inner.shape(),
            Self::F64(inner) => inner.shape(),
            Self::I8(inner) => inner.shape(),
            Self::I16(inner) => inner.shape(),
            Self::I32(inner) => inner.shape(),
            Self::I64(inner) => inner.shape(),
            Self::U8(inner) => inner.shape(),
            Self::U16(inner) => inner.shape(),
            Self::U32(inner) => inner.shape(),
            Self::U64(inner) => inner.shape(),
            Self::String(inner) => inner.shape(),
            Self::Binary(inner) => inner.shape(),
            Self::Boolean(inner) => inner.shape(),
            Self::Image(inner) => inner.shape(),
            Self::Audio(inner) => inner.shape(),
            Self::Video(inner) => inner.shape(),
        }
    }

    pub fn len(&self) -> usize {
        self.shape().iter().product()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// If this is a numeric type (f32, f64, u8, u16, u32, u64, i8, i16, i32 or i64), casts it to a float
    /// array.
    ///
    /// Returns an array that is either owned or not. If the data was of type f64, the returned
    /// array is borrowed. Otherwise a new array is created, so the returned array is owned.
    pub fn as_f64(self) -> Option<CowArray<'a, f64, IxDyn>> {
        match self {
            Self::F32(val) => Some(CowArray::from(val.map(|x| (*x).into()))),
            Self::F64(val) => Some(val),
            Self::I8(val) => Some(CowArray::from(val.map(|x| (*x).into()))),
            Self::I16(val) => Some(CowArray::from(val.map(|x| (*x).into()))),
            Self::I32(val) => Some(CowArray::from(val.map(|x| (*x).into()))),
            Self::I64(val) => Some(CowArray::from(val.map(|x| (*x) as f64))),
            Self::U8(val) => Some(CowArray::from(val.map(|x| (*x).into()))),
            Self::U16(val) => Some(CowArray::from(val.map(|x| (*x).into()))),
            Self::U32(val) => Some(CowArray::from(val.map(|x| (*x).into()))),
            Self::U64(val) => Some(CowArray::from(val.map(|x| (*x) as f64))),
            _ => None,
        }
    }

    /// If this is a numeric type (f32, f64, u8, u16, u32, u64, i8, i16, i32 or i64) with length 1, casts the
    /// single element to an f64 and returns it.
    pub fn as_f64_item(self) -> Option<f64> {
        if self.len() != 1 {
            return None;
        }
        match self {
            Self::F32(val) => Some((*val.first().unwrap()).into()),
            Self::F64(val) => Some(*val.first().unwrap()),
            Self::I8(val) => Some((*val.first().unwrap()).into()),
            Self::I16(val) => Some((*val.first().unwrap()).into()),
            Self::I32(val) => Some((*val.first().unwrap()).into()),
            Self::I64(val) => Some(*val.first().unwrap() as f64),
            Self::U8(val) => Some((*val.first().unwrap()).into()),
            Self::U16(val) => Some((*val.first().unwrap()).into()),
            Self::U32(val) => Some((*val.first().unwrap()).into()),
            Self::U64(val) => Some(*val.first().unwrap() as f64),
            _ => None,
        }
    }

    /// If this is a numeric type (f32, f64, u8, u16, u32, u64, i8, i16, i32 or i64), cast it to an i64
    /// array.
    ///
    /// Returns an array that is either owned or not. If the data was of type i64, the returned
    /// array is borrowed. Otherwise a new array is created, so the returned array is owned.
    pub fn as_i64(&'a self) -> Option<CowArray<'a, i64, IxDyn>> {
        match self {
            Self::F32(val) => Some(CowArray::from(val.map(|x| *x as i64))),
            Self::F64(val) => Some(CowArray::from(val.map(|x| *x as i64))),
            Self::I8(val) => Some(CowArray::from(val.map(|x| (*x).into()))),
            Self::I16(val) => Some(CowArray::from(val.map(|x| (*x).into()))),
            Self::I32(val) => Some(CowArray::from(val.map(|x| (*x).into()))),
            Self::I64(val) => Some(val.into()),
            Self::U8(val) => Some(CowArray::from(val.map(|x| (*x).into()))),
            Self::U16(val) => Some(CowArray::from(val.map(|x| (*x).into()))),
            Self::U32(val) => Some(CowArray::from(val.map(|x| (*x).into()))),
            Self::U64(val) => Some(CowArray::from(
                val.map(|x| (*x).try_into().unwrap_or(i64::MAX)),
            )),
            _ => None,
        }
    }

    /// If this is a numeric type (f32, f64, u8, u16, u32, u64, i8, i16, i32 or i64) with length 1, casts the
    /// single element to an i64 and returns it.
    pub fn as_i64_item(self) -> Option<i64> {
        if self.len() != 1 {
            return None;
        }
        match self {
            Self::F32(val) => Some(*val.first().unwrap() as i64),
            Self::F64(val) => Some(*val.first().unwrap() as i64),
            Self::I8(val) => Some((*val.first().unwrap()).into()),
            Self::I16(val) => Some((*val.first().unwrap()).into()),
            Self::I32(val) => Some((*val.first().unwrap()).into()),
            Self::I64(val) => Some(*val.first().unwrap()),
            Self::U8(val) => Some((*val.first().unwrap()).into()),
            Self::U16(val) => Some((*val.first().unwrap()).into()),
            Self::U32(val) => Some((*val.first().unwrap()).into()),
            Self::U64(val) => Some((*val.first().unwrap()).try_into().unwrap_or(i64::MAX)),
            _ => None,
        }
    }

    /// If this is a numeric type (f32, f64, u8, u16, u32, u64, i8, i16, i32 or i64), casts it to a u64.
    /// If the value is i8, i16, i32 or i64 and negative, None is returned.
    pub fn as_u64(&'a self) -> Option<CowArray<'a, u64, IxDyn>> {
        match self {
            Self::F32(val) => Some(CowArray::from(val.map(|x| *x as u64))),
            Self::F64(val) => Some(CowArray::from(val.map(|x| *x as u64))),
            Self::U8(val) => Some(CowArray::from(val.map(|x| (*x).into()))),
            Self::U16(val) => Some(CowArray::from(val.map(|x| (*x).into()))),
            Self::U32(val) => Some(CowArray::from(val.map(|x| (*x).into()))),
            Self::U64(val) => Some(val.into()),
            Self::I8(val) => Some(CowArray::from(val.map(|x| (*x).try_into().unwrap_or(0)))),
            Self::I16(val) => Some(CowArray::from(val.map(|x| (*x).try_into().unwrap_or(0)))),
            Self::I32(val) => Some(CowArray::from(val.map(|x| (*x).try_into().unwrap_or(0)))),
            Self::I64(val) => Some(CowArray::from(val.map(|x| (*x).try_into().unwrap_or(0)))),
            _ => None,
        }
    }

    /// If this is a numeric type (f32, f64, u8, u16, u32, u64, i8, i16, i32 or i64) with length 1, casts the
    /// single element to an u64 and returns it.
    pub fn as_u64_item(self) -> Option<u64> {
        if self.len() != 1 {
            return None;
        }
        match self {
            Self::F32(val) => Some(*val.first().unwrap() as u64),
            Self::F64(val) => Some(*val.first().unwrap() as u64),
            Self::I8(val) => Some((*val.first().unwrap()).try_into().unwrap_or(0)),
            Self::I16(val) => Some((*val.first().unwrap()).try_into().unwrap_or(0)),
            Self::I32(val) => Some((*val.first().unwrap()).try_into().unwrap_or(0)),
            Self::I64(val) => Some((*val.first().unwrap()).try_into().unwrap_or(0)),
            Self::U8(val) => Some((*val.first().unwrap()).into()),
            Self::U16(val) => Some((*val.first().unwrap()).into()),
            Self::U32(val) => Some((*val.first().unwrap()).into()),
            Self::U64(val) => Some(*val.first().unwrap()),
            _ => None,
        }
    }

    pub(crate) fn serialized_len(&self) -> usize {
        let size_from_elements = match self {
            Self::F32(inner) => inner.len() * std::mem::size_of::<f32>(),
            Self::F64(inner) => inner.len() * std::mem::size_of::<f64>(),
            Self::I8(inner) => inner.len() * std::mem::size_of::<i8>(),
            Self::I16(inner) => inner.len() * std::mem::size_of::<i16>(),
            Self::I32(inner) => inner.len() * std::mem::size_of::<i32>(),
            Self::I64(inner) => inner.len() * std::mem::size_of::<i64>(),
            Self::U8(inner) => inner.len() * std::mem::size_of::<u8>(),
            Self::U16(inner) => inner.len() * std::mem::size_of::<u16>(),
            Self::U32(inner) => inner.len() * std::mem::size_of::<u32>(),
            Self::U64(inner) => inner.len() * std::mem::size_of::<u64>(),
            Self::String(inner) => inner
                .iter()
                .map(|x| {
                    crate::varint::get_varint_u64_len(x.len().try_into().unwrap()) as usize
                        + x.len()
                })
                .sum::<usize>(),
            Self::Binary(inner) => inner
                .iter()
                .map(|x| {
                    crate::varint::get_varint_u64_len(x.len().try_into().unwrap()) as usize
                        + x.len()
                })
                .sum::<usize>(),
            Self::Boolean(inner) => inner.len() * std::mem::size_of::<u8>(),
            Self::Image(inner) => inner
                .iter()
                .map(|x| {
                    let len: u64 = x.data.len().try_into().unwrap();
                    crate::varint::get_varint_u64_len(3 + len) as usize + 3 + x.data.len()
                })
                .sum::<usize>(),
            Self::Audio(inner) => inner
                .iter()
                .map(|x| {
                    let len: u64 = x.data.len().try_into().unwrap();
                    crate::varint::get_varint_u64_len(3 + len) as usize + 3 + x.data.len()
                })
                .sum::<usize>(),
            Self::Video(inner) => inner
                .iter()
                .map(|x| {
                    let len: u64 = x.data.len().try_into().unwrap();
                    crate::varint::get_varint_u64_len(3 + len) as usize + 3 + x.data.len()
                })
                .sum::<usize>(),
        };

        let shape = self.shape();
        let size_from_shape = 1 + shape
            .iter()
            .map(|x| crate::varint::get_varint_u64_len(*x as u64) as usize)
            .sum::<usize>();

        1 + size_from_shape + size_from_elements
    }

    pub(crate) fn serialize_append(&self, res: &mut Vec<u8>) {
        let first_byte = match self {
            Self::F32(_) => TYPE_SPEC_F32,
            Self::F64(_) => TYPE_SPEC_F64,
            Self::I8(_) => TYPE_SPEC_I8,
            Self::I16(_) => TYPE_SPEC_I16,
            Self::I32(_) => TYPE_SPEC_I32,
            Self::I64(_) => TYPE_SPEC_I64,
            Self::U8(_) => TYPE_SPEC_U8,
            Self::U16(_) => TYPE_SPEC_U16,
            Self::U32(_) => TYPE_SPEC_U32,
            Self::U64(_) => TYPE_SPEC_U64,
            Self::String(_) => TYPE_SPEC_STRING,
            Self::Binary(_) => TYPE_SPEC_BINARY,
            Self::Boolean(_) => TYPE_SPEC_BOOLEAN,
            Self::Image(_) => TYPE_SPEC_IMAGE,
            Self::Audio(_) => TYPE_SPEC_AUDIO,
            Self::Video(_) => TYPE_SPEC_VIDEO,
        };

        res.push(first_byte);

        let shape = self.shape();
        res.push(
            shape
                .len()
                .try_into()
                .expect("The data cannot contain more than 255 dimensions."),
        );

        let mut written_bytes = 2;

        for dim in shape {
            crate::varint::append_varint_u64((*dim).try_into().unwrap(), res);
            written_bytes += crate::varint::get_varint_u64_len((*dim).try_into().unwrap()) as usize;
        }

        #[cfg(not(target_endian = "little"))]
        use byteorder::{LittleEndian, WriteBytesExt};

        match self {
            Self::F32(inner) => {
                #[cfg(target_endian = "little")]
                res.extend_from_slice(
                    inner
                        .as_standard_layout()
                        .as_slice()
                        .unwrap()
                        .as_byte_slice(),
                );
                #[cfg(not(target_endian = "little"))]
                for &val in inner.as_standard_layout().as_slice().unwrap() {
                    res.write_f32::<LittleEndian>(val).unwrap();
                }
            }
            Self::F64(inner) => {
                #[cfg(target_endian = "little")]
                res.extend_from_slice(
                    inner
                        .as_standard_layout()
                        .as_slice()
                        .unwrap()
                        .as_byte_slice(),
                );
                #[cfg(not(target_endian = "little"))]
                for &val in inner.as_standard_layout().as_slice().unwrap() {
                    res.write_f64::<LittleEndian>(val).unwrap();
                }
            }
            Self::I8(inner) => {
                res.extend_from_slice(
                    inner
                        .as_standard_layout()
                        .as_slice()
                        .unwrap()
                        .as_byte_slice(),
                );
            }
            Self::I16(inner) => {
                #[cfg(target_endian = "little")]
                res.extend_from_slice(
                    inner
                        .as_standard_layout()
                        .as_slice()
                        .unwrap()
                        .as_byte_slice(),
                );
                #[cfg(not(target_endian = "little"))]
                for &val in inner.as_standard_layout().as_slice().unwrap() {
                    res.write_i16::<LittleEndian>(val).unwrap();
                }
            }
            Self::I32(inner) => {
                #[cfg(target_endian = "little")]
                res.extend_from_slice(
                    inner
                        .as_standard_layout()
                        .as_slice()
                        .unwrap()
                        .as_byte_slice(),
                );
                #[cfg(not(target_endian = "little"))]
                for &val in inner.as_standard_layout().as_slice().unwrap() {
                    res.write_i32::<LittleEndian>(val).unwrap();
                }
            }
            Self::I64(inner) => {
                #[cfg(target_endian = "little")]
                res.extend_from_slice(
                    inner
                        .as_standard_layout()
                        .as_slice()
                        .unwrap()
                        .as_byte_slice(),
                );
                #[cfg(not(target_endian = "little"))]
                for &val in inner.as_standard_layout().as_slice().unwrap() {
                    res.write_i64::<LittleEndian>(val).unwrap();
                }
            }
            Self::U8(inner) => {
                res.extend_from_slice(
                    inner
                        .as_standard_layout()
                        .as_slice()
                        .unwrap()
                        .as_byte_slice(),
                );
            }
            Self::U16(inner) => {
                #[cfg(target_endian = "little")]
                res.extend_from_slice(
                    inner
                        .as_standard_layout()
                        .as_slice()
                        .unwrap()
                        .as_byte_slice(),
                );
                #[cfg(not(target_endian = "little"))]
                for &val in inner.as_standard_layout().as_slice().unwrap() {
                    res.write_u16::<LittleEndian>(val).unwrap();
                }
            }
            Self::U32(inner) => {
                #[cfg(target_endian = "little")]
                res.extend_from_slice(
                    inner
                        .as_standard_layout()
                        .as_slice()
                        .unwrap()
                        .as_byte_slice(),
                );
                #[cfg(not(target_endian = "little"))]
                for &val in inner.as_standard_layout().as_slice().unwrap() {
                    res.write_u32::<LittleEndian>(val).unwrap();
                }
            }
            Self::U64(inner) => {
                #[cfg(target_endian = "little")]
                res.extend_from_slice(
                    inner
                        .as_standard_layout()
                        .as_slice()
                        .unwrap()
                        .as_byte_slice(),
                );
                #[cfg(not(target_endian = "little"))]
                for &val in inner.as_standard_layout().as_slice().unwrap() {
                    res.write_u64::<LittleEndian>(val).unwrap();
                }
            }
            Self::String(inner) => {
                crate::varint::append_varint_u64(
                    (self.serialized_len() - written_bytes) as u64,
                    res,
                );
                inner.iter().for_each(|x| {
                    crate::varint::append_varint_u64(x.len().try_into().unwrap(), res);
                    res.extend_from_slice(x.as_bytes())
                })
            }
            Self::Binary(inner) => {
                crate::varint::append_varint_u64(
                    (self.serialized_len() - written_bytes) as u64,
                    res,
                );
                inner.iter().for_each(|x| {
                    crate::varint::append_varint_u64(x.len().try_into().unwrap(), res);
                    res.extend_from_slice(x)
                })
            }
            Self::Boolean(inner) => res.extend_from_slice(
                inner
                    .map(|x| if *x { 1u8 } else { 0 })
                    .as_standard_layout()
                    .as_slice()
                    .unwrap(),
            ),
            Self::Image(inner) => {
                crate::varint::append_varint_u64(
                    (self.serialized_len() - written_bytes) as u64,
                    res,
                );
                inner.iter().for_each(|x| {
                    let len: u64 = x.data.len().try_into().unwrap();
                    crate::varint::append_varint_u64(3 + len, res);
                    res.extend_from_slice(x.format().as_bytes());
                    res.extend_from_slice(x.data)
                })
            }
            Self::Audio(inner) => {
                crate::varint::append_varint_u64(
                    (self.serialized_len() - written_bytes) as u64,
                    res,
                );
                inner.iter().for_each(|x| {
                    let len: u64 = x.data.len().try_into().unwrap();
                    crate::varint::append_varint_u64(3 + len, res);
                    res.extend_from_slice(x.format().as_bytes());
                    res.extend_from_slice(x.data)
                })
            }
            Self::Video(inner) => {
                crate::varint::append_varint_u64(
                    (self.serialized_len() - written_bytes) as u64,
                    res,
                );
                inner.iter().for_each(|x| {
                    let len: u64 = x.data.len().try_into().unwrap();
                    crate::varint::append_varint_u64(3 + len, res);
                    res.extend_from_slice(x.format().as_bytes());
                    res.extend_from_slice(x.data)
                })
            }
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(self.serialized_len());
        self.serialize_append(&mut res);
        res
    }
}

#[derive(Clone)]
pub struct OwnedDecthingsTensor {
    pub(crate) data: bytes::Bytes,
}

impl OwnedDecthingsTensor {
    pub fn from_bytes(data: bytes::Bytes) -> Result<Self, DeserializeDecthingsTensorError> {
        let Some(&first_byte) = data.first() else {
            return Err(DeserializeDecthingsTensorError::UnexpectedEndOfBytes);
        };

        let Some(&num_dims) = data.get(1) else {
            return Err(DeserializeDecthingsTensorError::UnexpectedEndOfBytes);
        };

        let mut shape: Vec<usize> = Vec::with_capacity(num_dims.into());
        let mut pos = 2;

        for _ in 0..num_dims {
            if data.len() < pos + 1 {
                return Err(DeserializeDecthingsTensorError::UnexpectedEndOfBytes);
            };
            if data.len()
                < pos + crate::varint::get_serialized_varint_u64_len(&data[pos..]) as usize
            {
                return Err(DeserializeDecthingsTensorError::UnexpectedEndOfBytes);
            }
            let (dim, varint_len) = crate::varint::deserialize_varint_u64(&data[pos..]);
            pos += varint_len as usize;
            shape.push(dim.try_into().unwrap());
        }

        let numel = shape.iter().fold(1usize, |a, b| a * (*b));

        let element_size = match first_byte {
            TYPE_SPEC_F32 | TYPE_SPEC_I32 | TYPE_SPEC_U32 => Some(4),
            TYPE_SPEC_F64 | TYPE_SPEC_I64 | TYPE_SPEC_U64 => Some(8),
            TYPE_SPEC_BOOLEAN | TYPE_SPEC_I8 | TYPE_SPEC_U8 => Some(1),
            TYPE_SPEC_I16 | TYPE_SPEC_U16 => Some(2),
            TYPE_SPEC_STRING | TYPE_SPEC_BINARY | TYPE_SPEC_IMAGE | TYPE_SPEC_AUDIO
            | TYPE_SPEC_VIDEO => None,
            _ => {
                return Err(DeserializeDecthingsTensorError::InvalidBytes(format!(
                    "Unexected first byte {first_byte}"
                )));
            }
        };

        match element_size {
            Some(element_size) => {
                if data.len() < pos + numel * element_size {
                    return Err(DeserializeDecthingsTensorError::UnexpectedEndOfBytes);
                }
                pos += numel * element_size;
            }
            None => {
                if data.len() < pos + 1 {
                    return Err(DeserializeDecthingsTensorError::UnexpectedEndOfBytes);
                };
                if data.len()
                    < pos + crate::varint::get_serialized_varint_u64_len(&data[pos..]) as usize
                {
                    return Err(DeserializeDecthingsTensorError::UnexpectedEndOfBytes);
                }
                pos += crate::varint::get_serialized_varint_u64_len(&data[pos..]) as usize;

                for _ in 0..numel {
                    if data.len() < pos + 1 {
                        return Err(DeserializeDecthingsTensorError::UnexpectedEndOfBytes);
                    };
                    if data.len()
                        < pos + crate::varint::get_serialized_varint_u64_len(&data[pos..]) as usize
                    {
                        return Err(DeserializeDecthingsTensorError::UnexpectedEndOfBytes);
                    }
                    let (len, varint_len) = crate::varint::deserialize_varint_u64(&data[pos..]);
                    let len: usize = len.try_into().unwrap();
                    if matches!(first_byte, TYPE_SPEC_STRING) {
                        if let Err(e) = std::str::from_utf8(&data[pos..pos + len]) {
                            return Err(DeserializeDecthingsTensorError::InvalidBytes(format!(
                                "The string was not UTF-8: {e:?}"
                            )));
                        }
                    }
                    if matches!(first_byte, TYPE_SPEC_IMAGE) && len < 3 {
                        if len < 3 {
                            return Err(DeserializeDecthingsTensorError::InvalidBytes(format!(
                                "Expected three bytes for image format - got {len}"
                            )));
                        }
                        if let Err(e) = std::str::from_utf8(&data[pos..pos + 3]) {
                            return Err(DeserializeDecthingsTensorError::InvalidBytes(format!(
                                "The image format was not UTF-8: {e:?}"
                            )));
                        }
                    }
                    if matches!(first_byte, TYPE_SPEC_AUDIO) && len < 3 {
                        if len < 3 {
                            return Err(DeserializeDecthingsTensorError::InvalidBytes(format!(
                                "Expected three bytes for audio format - got {len}"
                            )));
                        }
                        if let Err(e) = std::str::from_utf8(&data[pos..pos + 3]) {
                            return Err(DeserializeDecthingsTensorError::InvalidBytes(format!(
                                "The audio format was not UTF-8: {e:?}"
                            )));
                        }
                    }
                    if matches!(first_byte, TYPE_SPEC_VIDEO) && len < 3 {
                        if len < 3 {
                            return Err(DeserializeDecthingsTensorError::InvalidBytes(format!(
                                "Expected three bytes for video format - got {len}"
                            )));
                        }
                        if let Err(e) = std::str::from_utf8(&data[pos..pos + 3]) {
                            return Err(DeserializeDecthingsTensorError::InvalidBytes(format!(
                                "The video format was not UTF-8: {e:?}"
                            )));
                        }
                    }
                    pos += len + varint_len as usize;
                }
            }
        }

        Ok(Self {
            data: data.slice(0..pos),
        })
    }

    pub fn byte_size(&self) -> usize {
        self.data.len()
    }

    pub fn tensor(&self) -> DecthingsTensor<'_> {
        let first_byte = self.data[0];
        let num_dims = self.data[1];

        let mut shape: Vec<usize> = Vec::with_capacity(num_dims.into());
        let mut pos = 2;

        for _ in 0..num_dims {
            let (dim, varint_len) = crate::varint::deserialize_varint_u64(&self.data[pos..]);
            pos += varint_len as usize;
            shape.push(dim.try_into().unwrap());
        }

        let numel = shape.iter().fold(1usize, |a, b| a * (*b));

        fn sized_into_tensor<'a, T: Clone + Default + FromByteSlice + ToMutByteSlice + 'a>(
            shape: &[usize],
            data: &'a [u8],
            pos: usize,
            numel: usize,
            f: impl FnOnce(CowArray<'a, T, IxDyn>) -> DecthingsTensor<'a>,
        ) -> DecthingsTensor<'a> {
            let slice = &data[pos..pos + numel * std::mem::size_of::<T>()];

            #[cfg(target_endian = "little")]
            if let Ok(val) = slice.as_slice_of::<T>() {
                return f(ArrayView::from(val)
                    .into_shape(IxDyn(shape))
                    .unwrap()
                    .into());
            }

            // We are either big-endian, or got an alignment error, in which case we need to copy.

            #[cfg(target_endian = "little")]
            {
                let mut res: Vec<T> = vec![T::default(); numel];
                res.as_mut_byte_slice().copy_from_slice(slice);
                f(Array::from(res).into_shape(IxDyn(shape)).unwrap().into())
            }

            #[cfg(not(target_endian = "little"))]
            {
                let mut res: Vec<T> = Vec::with_capacity(numel);
                let cursor = std::io::Cursor::new(slice);
                for _ in 0..numel {
                    res.push();
                }
                f(Array::from(res).into_shape(IxDyn(shape)).unwrap().into())
            }
        }

        match first_byte {
            TYPE_SPEC_F32 => {
                return sized_into_tensor(&shape, &self.data, pos, numel, DecthingsTensor::F32);
            }
            TYPE_SPEC_F64 => {
                return sized_into_tensor(&shape, &self.data, pos, numel, DecthingsTensor::F64);
            }
            TYPE_SPEC_I8 => {
                return sized_into_tensor(&shape, &self.data, pos, numel, DecthingsTensor::I8);
            }
            TYPE_SPEC_I16 => {
                return sized_into_tensor(&shape, &self.data, pos, numel, DecthingsTensor::I16);
            }
            TYPE_SPEC_I32 => {
                return sized_into_tensor(&shape, &self.data, pos, numel, DecthingsTensor::I32);
            }
            TYPE_SPEC_I64 => {
                return sized_into_tensor(&shape, &self.data, pos, numel, DecthingsTensor::I64);
            }
            TYPE_SPEC_U8 => {
                return sized_into_tensor(&shape, &self.data, pos, numel, DecthingsTensor::U8);
            }
            TYPE_SPEC_U16 => {
                return sized_into_tensor(&shape, &self.data, pos, numel, DecthingsTensor::U16);
            }
            TYPE_SPEC_U32 => {
                return sized_into_tensor(&shape, &self.data, pos, numel, DecthingsTensor::U32);
            }
            TYPE_SPEC_U64 => {
                return sized_into_tensor(&shape, &self.data, pos, numel, DecthingsTensor::U64);
            }
            TYPE_SPEC_STRING => {
                pos += crate::varint::get_serialized_varint_u64_len(&self.data[pos..]) as usize;

                let mut strings = Vec::with_capacity(numel);
                for _ in 0..numel {
                    let (len, varint_len) =
                        crate::varint::deserialize_varint_u64(&self.data[pos..]);
                    let len: usize = len.try_into().unwrap();
                    pos += varint_len as usize;
                    strings.push(std::str::from_utf8(&self.data[pos..pos + len]).unwrap());
                    pos += len;
                }
                return DecthingsTensor::String(
                    Array::from_vec(strings)
                        .into_shape(IxDyn(&shape))
                        .unwrap()
                        .into(),
                );
            }
            TYPE_SPEC_BINARY => {
                pos += crate::varint::get_serialized_varint_u64_len(&self.data[pos..]) as usize;

                let mut binaries = Vec::with_capacity(numel);
                for _ in 0..numel {
                    let (len, varint_len) =
                        crate::varint::deserialize_varint_u64(&self.data[pos..]);
                    let len: usize = len.try_into().unwrap();
                    pos += varint_len as usize;
                    binaries.push(&self.data[pos..pos + len]);
                    pos += len;
                }
                return DecthingsTensor::Binary(
                    Array::from_vec(binaries)
                        .into_shape(IxDyn(&shape))
                        .unwrap()
                        .into(),
                );
            }
            TYPE_SPEC_BOOLEAN => {
                let converted = self.data[pos..pos + numel * std::mem::size_of::<u8>()]
                    .as_slice_of::<u8>()
                    .unwrap();
                return DecthingsTensor::Boolean(
                    ArrayView::from(converted)
                        .into_shape(IxDyn(&shape))
                        .unwrap()
                        .map(|&x| x != 0)
                        .into(),
                );
            }
            TYPE_SPEC_IMAGE => {
                pos += crate::varint::get_serialized_varint_u64_len(&self.data[pos..]) as usize;

                let mut images = Vec::with_capacity(numel);
                for _ in 0..numel {
                    let (len, varint_len) =
                        crate::varint::deserialize_varint_u64(&self.data[pos..]);
                    let len: usize = len.try_into().unwrap();
                    pos += varint_len as usize;
                    let format = std::str::from_utf8(&self.data[pos..pos + 3]).unwrap();
                    images.push(
                        DecthingsElementImage::new(format, &self.data[pos + 3..pos + len]).unwrap(),
                    );
                    pos += len;
                }
                return DecthingsTensor::Image(
                    Array::from_vec(images)
                        .into_shape(IxDyn(&shape))
                        .unwrap()
                        .into(),
                );
            }
            TYPE_SPEC_AUDIO => {
                pos += crate::varint::get_serialized_varint_u64_len(&self.data[pos..]) as usize;

                let mut audios = Vec::with_capacity(numel);
                for _ in 0..numel {
                    let (len, varint_len) =
                        crate::varint::deserialize_varint_u64(&self.data[pos..]);
                    let len: usize = len.try_into().unwrap();
                    pos += varint_len as usize;
                    let format = std::str::from_utf8(&self.data[pos..pos + 3]).unwrap();
                    audios.push(
                        DecthingsElementAudio::new(format, &self.data[pos + 3..pos + len]).unwrap(),
                    );
                    pos += len;
                }
                return DecthingsTensor::Audio(
                    Array::from_vec(audios)
                        .into_shape(IxDyn(&shape))
                        .unwrap()
                        .into(),
                );
            }
            TYPE_SPEC_VIDEO => {
                pos += crate::varint::get_serialized_varint_u64_len(&self.data[pos..]) as usize;

                let mut videos = Vec::with_capacity(numel);
                for _ in 0..numel {
                    let (len, varint_len) =
                        crate::varint::deserialize_varint_u64(&self.data[pos..]);
                    let len: usize = len.try_into().unwrap();
                    pos += varint_len as usize;
                    let format = std::str::from_utf8(&self.data[pos..pos + 3]).unwrap();
                    videos.push(
                        DecthingsElementVideo::new(format, &self.data[pos + 3..pos + len]).unwrap(),
                    );
                    pos += len;
                }
                DecthingsTensor::Video(
                    Array::from_vec(videos)
                        .into_shape(IxDyn(&shape))
                        .unwrap()
                        .into(),
                )
            }
            _ => {
                unreachable!()
            }
        }
    }
}

impl std::fmt::Debug for OwnedDecthingsTensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OwnedDecthingsTensor({:?})", self.tensor())
    }
}
