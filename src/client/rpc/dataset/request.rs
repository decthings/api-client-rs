use crate::{
    client::rpc::TagProvider,
    tensor::{DecthingsParameterDefinition, DecthingsTensor},
};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDatasetParams<'a> {
    /// The dataset's name.
    pub name: &'a str,
    /// A description of the dataset.
    pub description: &'a str,
    /// If true, all Decthings users can find and use this dataset. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access: Option<bool>,
    /// Tags are used to specify things like dataset type (image classification, etc.) and other metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<&'a [TagProvider<'a>]>,
    /// Each key contains separate data, allowing you to mix multiple types. For example, for an image dataset you
    /// could have an "image" of type image, and "label" of type string.
    pub keys: &'a [&'a DecthingsParameterDefinition],
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDatasetProperties<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<&'a [TagProvider<'a>]>,
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
pub struct GetDatasetsFilter<'a, S: AsRef<str>> {
    #[serde(serialize_with = "super::super::serialize_option_asref_str_seq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners: Option<&'a [S]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<&'a [TagProvider<'a>]>,
    #[serde(serialize_with = "super::super::serialize_option_asref_str_seq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<&'a [S]>,
    #[serde(serialize_with = "super::super::serialize_option_asref_str_seq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<&'a [S]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_name: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(bound(serialize = ""))]
pub struct GetDatasetsParams<'a, S: AsRef<str>> {
    /// Number of items from the results to skip. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// Max number of items to return. Defaults to 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// If specified, determines which items to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<GetDatasetsFilter<'a, S>>,
    /// Specifies a field in the returned items to sort by. Defaults to "createdAt".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
}

#[derive(Debug, Clone)]
pub struct DataToAddForKey<'a> {
    pub key: &'a str,
    pub data: Vec<DecthingsTensor<'a>>,
}

impl Serialize for DataToAddForKey<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
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
