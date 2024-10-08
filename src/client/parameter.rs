use serde::{Deserialize, Serialize, Serializer};

use crate::tensor::{DecthingsTensor, OwnedDecthingsTensor};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecthingsParameter {
    pub name: String,
    #[serde(skip_deserializing)]
    pub data: Vec<OwnedDecthingsTensor>,
}

fn serialize_null<T, S: Serializer>(_: T, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_none()
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "camelCase", bound(serialize = ""))]
pub enum DecthingsParameterProviderData<'a> {
    #[serde(serialize_with = "serialize_null")]
    Data(Vec<DecthingsTensor<'a>>),
    Dataset {
        dataset_id: &'a str,
        dataset_key: &'a str,
    },
}
impl DecthingsParameterProviderData<'_> {
    fn is_data(&self) -> bool {
        match self {
            Self::Data(_) => true,
            Self::Dataset { .. } => false,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase", bound(serialize = ""))]
pub struct DecthingsParameterProvider<'a> {
    pub name: &'a str,
    #[serde(skip_serializing_if = "DecthingsParameterProviderData::is_data")]
    pub data: DecthingsParameterProviderData<'a>,
}

pub(crate) fn serialize_parameter_provider_list<'a, 'b: 'a>(
    list: impl Iterator<Item = &'a DecthingsParameterProvider<'b>>,
) -> Vec<Vec<u8>> {
    list.filter_map(|x| match &x.data {
        DecthingsParameterProviderData::Dataset { .. } => None,
        DecthingsParameterProviderData::Data(d) => Some(d),
    })
    .map(|x| {
        let mut res = Vec::with_capacity(x.iter().map(|x| x.serialized_len()).sum::<usize>());
        for element in x {
            element.serialize_append(&mut res);
        }
        res
    })
    .collect()
}
