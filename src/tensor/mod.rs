mod element;
mod tensor_impl;

pub use element::*;
pub use tensor_impl::*;

use serde::{ser::SerializeSeq, Deserialize, Serialize, Serializer};

fn serialize_shape<S: Serializer>(
    values: &[Option<u32>],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut seq = serializer.serialize_seq(Some(values.len()))?;
    for value in values {
        if let Some(value) = value {
            seq.serialize_element(value)?;
        } else {
            seq.serialize_element(&-1i8)?;
        }
    }
    seq.end()
}

fn deserialize_shape<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<Option<u32>>, D::Error> {
    let a = Vec::<i64>::deserialize(deserializer)?;
    Ok(a.into_iter().map(|x| x.try_into().ok()).collect())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecthingsTensorRules {
    #[serde(
        serialize_with = "serialize_shape",
        deserialize_with = "deserialize_shape"
    )]
    pub shape: Vec<Option<u32>>,
    pub allowed_types: Vec<DecthingsElementType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecthingsParameterDefinition {
    pub name: String,
    pub rules: DecthingsTensorRules,
}
