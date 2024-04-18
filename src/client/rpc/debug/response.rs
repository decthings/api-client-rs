use serde::Deserialize;

use crate::{
    client::{
        rpc::{ExecutionLocation, ParameterDefinitions},
        DecthingsParameter,
    },
    tensor::OwnedDecthingsTensor,
};

#[cfg(feature = "events")]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum DebugSessionTerminatedReason {
    TerminatedOnRequest,
    LauncherTerminated,
    InactiveTimeout,
    Unknown,
    #[serde(rename_all = "camelCase")]
    CodeTerminated {
        exit_code: Option<i32>,
        signal: Option<String>,
        oom: bool,
    },
    #[serde(rename_all = "camelCase")]
    Exception {
        exception_details: Option<String>,
    },
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "params")]
pub enum DebugEvent {
    #[serde(rename_all = "camelCase")]
    Exit {
        debug_session_id: String,
        reason: DebugSessionTerminatedReason,
    },
    #[serde(rename_all = "camelCase")]
    Stdout {
        debug_session_id: String,
        #[serde(skip_deserializing)]
        data: bytes::Bytes,
    },
    #[serde(rename_all = "camelCase")]
    Stderr {
        debug_session_id: String,
        #[serde(skip_deserializing)]
        data: bytes::Bytes,
    },
    #[serde(rename_all = "camelCase")]
    Initialized { debug_session_id: String },
    #[serde(rename_all = "camelCase")]
    RemoteInspectorData {
        debug_session_id: String,
        #[serde(skip_deserializing)]
        data: bytes::Bytes,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchDebugSessionResult {
    /// A unique identifier which you should use in subsequent API calls.
    pub debug_session_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum LaunchDebugSessionError {
    ModelNotFound,
    InvalidExecutorType,
    PersistentLauncherNotFound,
    QuotaExceeded,
    ServerOverloaded,
    InvalidExecutable,
    ReadExecutableFileFailed,
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
pub enum DebugTrainingSessionState {
    Starting,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugTrainingSession {
    pub id: String,
    pub created_at: i64,
    pub state: DebugTrainingSessionState,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugSession {
    pub id: String,
    pub started_at: i64,
    pub model_id: String,
    pub parameter_definitions: ParameterDefinitions,
    pub training_sessions: Vec<DebugTrainingSession>,
    pub execution_location: ExecutionLocation,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDebugSessionsResult {
    pub debug_sessions: Vec<DebugSession>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetDebugSessionsError {
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
pub struct TerminateDebugSessionResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum TerminateDebugSessionError {
    DebugSessionNotFound,
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
pub struct StateKey {
    pub key: String,
    pub byte_size: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallCreateModelStateResult {
    pub data_id: String,
    pub state: Vec<StateKey>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum CallCreateModelStateError {
    DebugSessionNotFound,
    DebugSessionTerminated,
    #[serde(rename_all = "camelCase")]
    DatasetNotFound {
        dataset_id: String,
    },
    #[serde(rename_all = "camelCase")]
    DatasetKeyNotFound {
        dataset_id: String,
        dataset_key: String,
    },
    #[serde(rename_all = "camelCase")]
    Exception {
        exception_details: Option<String>,
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
pub struct CallInstantiateModelResult {
    /// A unique identifier which you should use in subsequent API calls.
    pub instantiated_model_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum CallInstantiateModelError {
    DebugSessionNotFound,
    DebugSessionTerminated,
    StateDataNotFound,
    #[serde(rename_all = "camelCase")]
    Exception {
        exception_details: Option<String>,
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
pub struct CallTrainResult {
    /// A unique identifier which you should use in subsequent API calls.
    pub training_session_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum CallTrainError {
    DebugSessionNotFound,
    InstantiatedModelNotFound,
    DebugSessionTerminated,
    #[serde(rename_all = "camelCase")]
    DatasetNotFound {
        dataset_id: String,
    },
    #[serde(rename_all = "camelCase")]
    DatasetKeyNotFound {
        dataset_id: String,
        dataset_key: String,
    },
    #[serde(rename_all = "camelCase")]
    Exception {
        exception_details: Option<String>,
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
pub struct DebugTrainingSessionMetrics {
    pub name: String,
    pub amount: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum DebugTrainingSessionFailReason {
    Unknown,
    Cancelled,
    MaxDurationExceeded,
    #[serde(rename_all = "camelCase")]
    Exception {
        exception_details: Option<String>,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "state")]
pub enum DebugTrainingSessionStatus {
    #[serde(rename_all = "camelCase")]
    Running { progress: f32 },
    #[serde(rename_all = "camelCase")]
    Completed {
        finished_at: i64,
        train_duration: u64,
    },
    #[serde(rename_all = "camelCase")]
    Failed {
        finished_at: i64,
        train_duration: Option<u64>,
        fail_reason: DebugTrainingSessionFailReason,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugGetTrainingStatusResult {
    pub id: String,
    pub created_at: i64,
    pub metrics: Vec<DebugTrainingSessionMetrics>,
    pub status: DebugTrainingSessionStatus,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum DebugGetTrainingStatusError {
    DebugSessionNotFound,
    TrainingSessionNotFound,
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
pub struct DebugFetchedTrainingMetric {
    pub timestamp: i64,
    #[serde(deserialize_with = "super::super::deserialize_empty_owned_decthings_tensor")]
    pub data: OwnedDecthingsTensor,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugFetchedTrainingMetrics {
    pub name: String,
    pub start_index: u32,
    pub entries: Vec<DebugFetchedTrainingMetric>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugGetTrainingMetricsResult {
    pub metrics: Vec<DebugFetchedTrainingMetrics>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum DebugGetTrainingMetricsError {
    DebugSessionNotFound,
    TrainingSessionNotFound,
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
pub struct DebugCancelTrainingSessionResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum DebugCancelTrainingSessionError {
    DebugSessionNotFound,
    TrainingSessionNotFound,
    TrainingSessionNotRunning,
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
pub struct CallEvaluateResult {
    pub output: Vec<DecthingsParameter>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum CallEvaluateError {
    DebugSessionNotFound,
    InstantiatedModelNotFound,
    DebugSessionTerminated,
    #[serde(rename_all = "camelCase")]
    DatasetNotFound {
        dataset_id: String,
    },
    #[serde(rename_all = "camelCase")]
    DatasetKeyNotFound {
        dataset_id: String,
        dataset_key: String,
    },
    #[serde(rename_all = "camelCase")]
    Exception {
        exception_details: Option<String>,
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
pub struct CallGetModelStateResult {
    pub data_id: String,
    pub state: Vec<StateKey>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum CallGetModelStateError {
    DebugSessionNotFound,
    InstantiatedModelNotFound,
    DebugSessionTerminated,
    #[serde(rename_all = "camelCase")]
    Exception {
        exception_details: Option<String>,
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
pub struct DownloadStateDataResult {
    #[serde(rename = "stateKeyNames")]
    pub data: Vec<super::super::StateKeyData>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum DownloadStateDataError {
    DebugSessionNotFound,
    StateDataNotFound,
    StateKeyNotFound,
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
pub struct SendToRemoteInspectorResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum SendToRemoteInspectorError {
    DebugSessionNotFound,
    NotRemoteInspector,
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

#[cfg(feature = "events")]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugSubscribeToEventsResult {}

#[cfg(feature = "events")]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum DebugSubscribeToEventsError {
    DebugSessionNotFound,
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

#[cfg(feature = "events")]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugUnsubscribeFromEventsResult {}

#[cfg(feature = "events")]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum DebugUnsubscribeFromEventsError {
    NotSubscribed,
    TooManyRequests,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}
