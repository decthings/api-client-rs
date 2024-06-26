use crate::{client::DecthingsParameterDefinition, tensor::OwnedDecthingsTensor};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDatasetResult {
    /// A unique identifier which you should use in subsequent API calls.
    pub dataset_id: String,
    /// The initial version identifier.
    pub dataset_version_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum CreateDatasetError {
    QuotaExceeded,
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDatasetResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum UpdateDatasetError {
    DatasetNotFound,
    AccessDenied,
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDatasetResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum DeleteDatasetError {
    DatasetNotFound,
    AccessDenied,
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DatasetAccess {
    Read,
    Readwrite,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetEntries {
    pub count: u32,
    pub total_byte_size: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dataset {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<super::super::Tag>,
    /// If this dataset was created by a user, the owner will be the userId for that user. Otherwise, the dataset was
    /// be created by Decthings, in which case the owner will be "decthings".
    pub owner: String,
    pub access: DatasetAccess,
    pub keys: Vec<DecthingsParameterDefinition>,
    pub entries: DatasetEntries,
    pub needs_review_entries: DatasetEntries,
    pub entries_waiting_to_be_deleted: DatasetEntries,
    /// The version identifier will be updated every time the data in the dataset changes, for example when an element
    /// is added. It can be used to prevent synchronization issues if multiple sources edit a dataset simultaneously.
    pub version_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDatasetsResult {
    pub datasets: Vec<Dataset>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetDatasetsError {
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddEntriesResult {
    /// The new dataset version identifier, which should be used as the version identifier in subsequent requests.
    pub new_dataset_version_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum AddEntriesError {
    DatasetNotFound,
    AccessDenied,
    LimitExceeded,
    QuotaExceeded,
    #[serde(rename_all = "camelCase")]
    IncorrectVersionId {
        /// The correct current dataset version ID, which should be used instead.
        dataset_version_id: String,
    },
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddEntriesToNeedsReviewResult {
    /// The new dataset version identifier, which should be used as the version identifier in subsequent requests.
    pub new_dataset_version_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum AddEntriesToNeedsReviewError {
    DatasetNotFound,
    AccessDenied,
    LimitExceeded,
    QuotaExceeded,
    #[serde(rename_all = "camelCase")]
    IncorrectVersionId {
        /// The correct current dataset version ID, which should be used instead.
        dataset_version_id: String,
    },
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinalizeNeedsReviewEntriesResult {
    /// The new dataset version identifier, which should be used as the version identifier in subsequent requests.
    pub new_dataset_version_id: String,
    /// The number of bytes that was removed from 'needs review'.
    pub removed_bytes_from_needs_review: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum FinalizeNeedsReviewEntriesError {
    DatasetNotFound,
    IndexOutOfRange,
    AccessDenied,
    QuotaExceeded,
    #[serde(rename_all = "camelCase")]
    IncorrectVersionId {
        /// The correct current dataset version ID, which should be used instead.
        dataset_version_id: String,
    },
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct InnerGetEntriesResult {
    pub keys: Vec<String>,
    pub indexes: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct FetchedEntry {
    pub index: u32,
    pub data: OwnedDecthingsTensor,
}

#[derive(Debug, Clone)]
pub struct KeyData {
    pub name: String,
    pub data: Vec<FetchedEntry>,
}

#[derive(Debug, Clone)]
pub struct GetEntriesResult {
    pub keys: Vec<KeyData>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetEntriesError {
    DatasetNotFound,
    #[serde(rename_all = "camelCase")]
    IncorrectVersionId {
        /// The correct current dataset version ID, which should be used instead.
        dataset_version_id: String,
    },
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct InnerGetNeedsReviewEntriesResult {
    pub keys: Vec<String>,
    pub indexes: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct GetNeedsReviewEntriesResult {
    pub keys: Vec<KeyData>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetNeedsReviewEntriesError {
    DatasetNotFound,
    #[serde(rename_all = "camelCase")]
    IncorrectVersionId {
        /// The correct current dataset version ID, which should be used instead.
        dataset_version_id: String,
    },
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveEntriesResult {
    /// The new dataset version identifier, which should be used as the version identifier in subsequent requests.
    pub new_dataset_version_id: String,
    pub removed_bytes: u64,
    pub new_waiting_to_remove_bytes: u64,
    pub new_waiting_to_remove_amount: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum RemoveEntriesError {
    DatasetNotFound,
    IndexOutOfRange,
    AccessDenied,
    #[serde(rename_all = "camelCase")]
    IncorrectVersionId {
        /// The correct current dataset version ID, which should be used instead.
        dataset_version_id: String,
    },
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveNeedsReviewEntriesResult {
    /// The new dataset version identifier, which should be used as the version identifier in subsequent requests.
    pub new_dataset_version_id: String,
    pub removed_bytes: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum RemoveNeedsReviewEntriesError {
    DatasetNotFound,
    IndexOutOfRange,
    AccessDenied,
    #[serde(rename_all = "camelCase")]
    IncorrectVersionId {
        /// The correct current dataset version ID, which should be used instead.
        dataset_version_id: String,
    },
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}
