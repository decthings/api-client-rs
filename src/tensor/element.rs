use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DecthingsElementType {
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    String,
    Boolean,
    Binary,
    Image,
    Audio,
    Video,
}

impl std::fmt::Display for DecthingsElementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::F32 => "f32",
            Self::F64 => "f64",
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::String => "string",
            Self::Boolean => "boolean",
            Self::Binary => "binary",
            Self::Image => "image",
            Self::Audio => "audio",
            Self::Video => "video",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum SetFormatError {
    FormatStringTooLong,
}

#[derive(Debug, Clone)]
pub struct DecthingsElementImage<'a> {
    format: Cow<'a, str>,
    pub data: Cow<'a, [u8]>,
}

impl<'a> DecthingsElementImage<'a> {
    pub fn new(
        format: impl Into<Cow<'a, str>>,
        data: impl Into<Cow<'a, [u8]>>,
    ) -> Result<Self, SetFormatError> {
        let format = format.into();
        if format.len() > u8::MAX.into() {
            return Err(SetFormatError::FormatStringTooLong);
        }
        Ok(Self {
            format,
            data: data.into(),
        })
    }

    pub fn format(&self) -> &str {
        &self.format
    }

    pub fn set_format(&mut self, format: impl Into<Cow<'a, str>>) -> Result<(), SetFormatError> {
        let format = format.into();
        if format.len() > u8::MAX.into() {
            return Err(SetFormatError::FormatStringTooLong);
        }
        self.format = format.into();
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DecthingsElementAudio<'a> {
    format: Cow<'a, str>,
    pub data: Cow<'a, [u8]>,
}

impl<'a> DecthingsElementAudio<'a> {
    pub fn new(
        format: impl Into<Cow<'a, str>>,
        data: impl Into<Cow<'a, [u8]>>,
    ) -> Result<Self, SetFormatError> {
        let format = format.into();
        if format.len() > u8::MAX.into() {
            return Err(SetFormatError::FormatStringTooLong);
        }
        Ok(Self {
            format,
            data: data.into(),
        })
    }

    pub fn format(&self) -> &str {
        &self.format
    }

    pub fn set_format(&mut self, format: impl Into<Cow<'a, str>>) -> Result<(), SetFormatError> {
        let format = format.into();
        if format.len() > u8::MAX.into() {
            return Err(SetFormatError::FormatStringTooLong);
        }
        self.format = format;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DecthingsElementVideo<'a> {
    format: Cow<'a, str>,
    pub data: Cow<'a, [u8]>,
}

impl<'a> DecthingsElementVideo<'a> {
    pub fn new(
        format: impl Into<Cow<'a, str>>,
        data: impl Into<Cow<'a, [u8]>>,
    ) -> Result<Self, SetFormatError> {
        let format = format.into();
        if format.len() > u8::MAX.into() {
            return Err(SetFormatError::FormatStringTooLong);
        }
        Ok(Self {
            format,
            data: data.into(),
        })
    }

    pub fn format(&self) -> &str {
        &self.format
    }

    pub fn set_format(&mut self, format: impl Into<Cow<'a, str>>) -> Result<(), SetFormatError> {
        let format = format.into();
        if format.len() > u8::MAX.into() {
            return Err(SetFormatError::FormatStringTooLong);
        }
        self.format = format;
        Ok(())
    }
}
