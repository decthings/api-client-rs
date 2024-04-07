use crate::{tensor::DecthingsTensor, DecthingsParameterDefinition};
use serde::{Serialize, Serializer};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDatasetParams<'a> {
    /// The dataset's name.
    pub name: &'a str,
    /// A description of the dataset.
    pub description: &'a str,
    /// Each key contains separate data, allowing you to mix multiple types. For example, for an image dataset you
    /// could have an "image" of type image, and "label" of type string.
    pub rules: &'a [&'a DecthingsParameterDefinition],
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDatasetProperties<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDatasetParams<'a> {
    /// The dataset's id.
    pub dataset_id: &'a str,
    /// Properties and values to change. Empty fields will not be changed.
    pub properties: UpdateDatasetProperties<'a>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDatasetParams<'a> {
    /// The dataset's id.
    pub dataset_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDatasetsParams<'a, S: AsRef<str>> {
    /// Which datasets to fetch. If unspecified, all datasets will be fetched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_ids: Option<&'a [S]>,
}

#[derive(Debug, Clone)]
pub struct DataToAddForKey<'a> {
    pub key: &'a str,
    pub data: Vec<DecthingsTensor<'a>>,
}

impl Serialize for DataToAddForKey<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.key)
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddEntriesParams<'a> {
    /// The dataset's id.
    pub dataset_id: &'a str,
    /// New data to add to the dataset. There should be one entry for each key in the dataset, and the length of the
    /// data to add to all keys must be the same.
    pub keys: Vec<DataToAddForKey<'a>>,
    /// If specified, the operation will only be performed if the current dataset versionId is equal to the specified
    /// string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_version_id: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddEntriesToNeedsReviewParams<'a> {
    /// The dataset's id.
    pub dataset_id: &'a str,
    /// New data to add to the dataset. There should be one entry for each key in the dataset, and the length of the
    /// data to add to all keys must be the same.
    pub keys: Vec<DataToAddForKey<'a>>,
    /// If specified, the operation will only be performed if the current dataset versionId is equal to the specified
    /// string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_version_id: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinalizeNeedsReviewEntriesParams<'a> {
    /// The dataset's id.
    pub dataset_id: &'a str,
    /// An array containing the index to remove from 'needs review'.
    pub indexes: &'a [u32],
    /// New data to add to the dataset, in place of the entries removed from 'needs review'. There should be one entry
    /// for each key in the dataset, and the length of the data in each key must equal the length of *indexes*.
    #[serde(skip_serializing)]
    pub keys: Vec<DataToAddForKey<'a>>,
    /// If specified, the operation will only be performed if the current dataset versionId is equal to the specified
    /// string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_version_id: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntriesToGetRange {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum EntriesToGet<'a> {
    Indexes(&'a [u32]),
    Range(EntriesToGetRange),
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEntriesParams<'a> {
    /// The dataset's id.
    pub dataset_id: &'a str,
    /// Which entries to fetch. Either an array of indexes or a start/end range.
    pub entries: EntriesToGet<'a>,
    /// If specified, the operation will only be performed if the current dataset versionId is equal to the specified
    /// string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_version_id: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNeedsReviewEntriesParams<'a> {
    /// The dataset's id.
    pub dataset_id: &'a str,
    /// Which entries to fetch. Either an array of indexes or a start/end range.
    pub entries: EntriesToGet<'a>,
    /// If specified, the operation will only be performed if the current dataset versionId is equal to the specified
    /// string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_version_id: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveEntriesParams<'a> {
    /// The dataset's id.
    pub dataset_id: &'a str,
    /// An array of indexes of the elements to remove.
    pub entries: &'a [u32],
    /// If specified, the operation will only be performed if the current dataset versionId is equal to the specified
    /// string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_version_id: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveNeedsReviewEntriesParams<'a> {
    /// The dataset's id.
    pub dataset_id: &'a str,
    /// An array of indexes of the elements to remove.
    pub entries: &'a [u32],
    /// If specified, the operation will only be performed if the current dataset versionId is equal to the specified
    /// string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_version_id: Option<&'a str>,
}
