use serde::{ser::SerializeSeq, Deserialize, Serialize, Serializer};

use crate::tensor::DecthingsParameterDefinition;

pub mod dataset;
pub mod debug;
pub mod fs;
pub mod image;

#[cfg(feature = "events")]
pub mod language;

pub mod model;
pub mod persistent_launcher;
pub mod spawned;
pub mod terminal;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TagProvider<'a> {
    pub tag: &'a str,
    pub value: &'a str,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub tag: String,
    pub value: String,
}

fn serialize_option_asref_str_seq<S: Serializer, T: AsRef<str>>(
    values: &Option<&[T]>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let Some(values) = values else {
        return serializer.serialize_none();
    };
    let mut seq = serializer.serialize_seq(Some(values.len()))?;
    for value in *values {
        seq.serialize_element(value.as_ref())?;
    }
    seq.end()
}

fn serialize_asref_str_seq<S: Serializer, T: AsRef<str>>(
    values: &&[T],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut seq = serializer.serialize_seq(Some(values.len()))?;
    for value in *values {
        seq.serialize_element(value.as_ref())?;
    }
    seq.end()
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum Response<R, E> {
    Result(R),
    Error(E),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LauncherSpecGpus {
    /// GPU type, such as "L4" or "A100 80GB"
    pub model: String,
    pub count: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LauncherSpec {
    pub cpus: f64,
    pub memory_mebibytes: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_mebibytes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_mebibytes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpus: Option<LauncherSpecGpus>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ExecutionLocationProvider<'a> {
    #[serde(rename_all = "camelCase")]
    PersistentLauncher { persistent_launcher_id: &'a str },
    #[serde(rename_all = "camelCase")]
    TemporaryLauncher { spec: &'a LauncherSpec },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ExecutionLocation {
    #[serde(rename_all = "camelCase")]
    PersistentLauncher {
        persistent_launcher_id: String,
        spec: LauncherSpec,
    },
    #[serde(rename_all = "camelCase")]
    TemporaryLauncher { spec: LauncherSpec },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDefinitions {
    pub create_state: Vec<DecthingsParameterDefinition>,
    pub train: Vec<DecthingsParameterDefinition>,
    pub evaluate_input: Vec<DecthingsParameterDefinition>,
    pub evaluate_output: Vec<DecthingsParameterDefinition>,
}

#[derive(Debug, Clone)]
pub struct StateKeyData {
    pub key: String,
    pub data: bytes::Bytes,
}

impl<'de> Deserialize<'de> for StateKeyData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self {
            key: s,
            data: vec![].into(),
        })
    }
}

fn many_decthings_tensors_from_bytes(
    data: bytes::Bytes,
) -> Result<Vec<crate::tensor::OwnedDecthingsTensor>, ()> {
    let mut res = vec![];
    let mut pos = 0;
    while data.len() > pos {
        let val =
            crate::tensor::OwnedDecthingsTensor::from_bytes(data.slice(pos..)).map_err(|_| ())?;
        pos += val.byte_size();
        res.push(val);
    }
    Ok(res)
}

pub(crate) fn deserialize_empty_owned_decthings_tensor<'de, D: serde::Deserializer<'de>>(
    _: D,
) -> Result<crate::tensor::OwnedDecthingsTensor, D::Error> {
    Ok(crate::tensor::OwnedDecthingsTensor {
        data: vec![].into(),
    })
}
