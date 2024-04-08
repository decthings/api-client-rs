use serde::{Deserialize, Serialize};

use crate::{
    rpc::{ExecutionLocation, LauncherConfig, LauncherSpec, ParameterDefinitions},
    tensor::OwnedDecthingsTensor,
    DecthingsParameter,
};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateModelResult {
    /// A unique identifier which you should use in subsequent API calls.
    pub model_id: String,
    /// Will be true if an initial state is being create, which means the model is being created until the operation is
    /// finished.
    pub is_now_creating: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum CreateModelError {
    ModelNotFound,
    SnapshotNotFound,
    QuotaExceeded,
    ServerOverloaded,
    InvalidExecutableFile,
    ReadExecutableFileFailed,
    #[serde(rename_all = "camelCase")]
    DatasetNotFound {
        dataset_id: String,
    },
    #[serde(rename_all = "camelCase")]
    DatasetKeyNotFound {
        dataset_id: String,
        dataset_key: String,
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
pub enum AtCodeStartupOrCreateState {
    CodeStartup,
    CreateState,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum CreateModelFailedCreateStateError {
    InvalidExecutableFile,
    ReadExecutableFileFailed,
    LauncherTerminated,
    #[serde(rename_all = "camelCase")]
    MaxDurationExceeded {
        at: AtCodeStartupOrCreateState,
    },
    #[serde(rename_all = "camelCase")]
    CodeTerminated {
        exit_code: Option<i32>,
        signal: Option<String>,
        oom: bool,
    },
    #[serde(rename_all = "camelCase")]
    Exception {
        at: AtCodeStartupOrCreateState,
        exception_details: Option<String>,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateModelFailedInitialStateDurations {
    pub create_launcher: u64,
    pub code_startup: Option<u64>,
    pub create_state: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum CreateModelFailedError {
    Cancelled,
    ServerOverloaded,
    Unknown,
    #[serde(rename_all = "camelCase")]
    CreateStateError {
        details: CreateModelFailedCreateStateError,
        create_initial_state_durations: CreateModelFailedInitialStateDurations,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateModelInitialStateDurations {
    pub create_launcher: u64,
    pub code_startup: u64,
    pub create_state: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WaitForModelToBeCreatedResult {
    #[serde(rename_all = "camelCase")]
    CreateModelFailed { error: CreateModelFailedError },
    #[serde(rename_all = "camelCase")]
    CreateModelSuccess {
        create_initial_state_durations: CreateModelInitialStateDurations,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum WaitForModelToBeCreatedError {
    ModelNotFound,
    ModelAlreadyCreated,
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
pub struct DeleteModelResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum DeleteModelError {
    ModelNotFound,
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
pub struct SnapshotModelResult {
    /// A unique identifier which you should use in subsequent API calls.
    pub snapshot_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum SnapshotModelError {
    ModelNotFound,
    AccessDenied,
    QuotaExceeded,
    ServerOverloaded,
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
pub struct UpdateSnapshotResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum UpdateSnapshotError {
    ModelNotFound,
    SnapshotNotFound,
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
pub struct DeleteSnapshotResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum DeleteSnapshotError {
    ModelNotFound,
    SnapshotNotFound,
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
pub struct UpdateModelResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum UpdateModelError {
    ModelNotFound,
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
pub enum ModelAccess {
    Read,
    Readwrite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Language {
    Go,
    Javascript,
    Typescript,
    Python,
    Rust,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultLauncherSpecs {
    pub create_state: LauncherSpec,
    pub evaluate: LauncherSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxDurationsSeconds {
    pub code_startup: u32,
    pub instantiate_model: u32,
    pub create_state: u32,
    pub train: u32,
    pub evaluate: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateKey {
    pub key: String,
    pub byte_size: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateMountedModel {
    pub model_id: String,
    pub snapshot_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelState {
    pub id: String,
    pub name: String,
    /// Identifiers of all the training operations that have been performed to reach this state.
    pub training_operations: Vec<String>,
    pub created_at: i64,
    pub being_deleted: bool,
    pub state: Vec<StateKey>,
    pub mounted_models: Vec<StateMountedModel>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotState {
    pub name: String,
    pub state: Vec<StateKey>,
    pub mounted_models: Vec<StateMountedModel>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelSnapshot {
    pub id: String,
    pub name: String,
    pub created_at: i64,
    pub filesystem_size_mebibytes: u32,
    pub launcher_config: LauncherConfig,
    pub parameter_definitions: ParameterDefinitions,
    pub default_launcher_specs: DefaultLauncherSpecs,
    pub max_durations_seconds: MaxDurationsSeconds,
    pub state: SnapshotState,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasedOnSnapshot {
    pub model_id: String,
    pub snapshot_id: String,
    pub no_longer_exists: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<super::super::Tag>,
    pub owner: String,
    pub access: ModelAccess,
    pub language: Language,
    pub wasm: bool,
    pub parameter_definitions: ParameterDefinitions,
    pub default_launcher_specs: DefaultLauncherSpecs,
    pub max_durations_seconds: MaxDurationsSeconds,
    pub filesystem_size_mebibytes: u32,
    pub launcher_config: LauncherConfig,
    pub ongoing_training_sessions: Vec<String>,
    pub training_sessions: Vec<String>,
    pub states: Vec<ModelState>,
    pub active_state: String,
    pub snapshots: Vec<ModelSnapshot>,
    pub based_on_snapshot: Option<BasedOnSnapshot>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetModelsResult {
    pub models: Vec<Model>,
    pub being_created: Vec<Model>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetModelsError {
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
pub struct SetFilesystemSizeResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum SetFilesystemSizeError {
    ModelNotFound,
    InvalidExecutorType,
    NotEnoughSpace,
    AccessDenied,
    QuotaExceeded,
    ServerOverloaded,
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
pub struct GetFilesystemUsageResult {
    pub block_size: u64,
    pub total_blocks: u64,
    pub free_blocks: u64,
    pub total_inodes: u64,
    pub free_inodes: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetFilesystemUsageError {
    ModelNotFound,
    NotEnoughSpace,
    AccessDenied,
    QuotaExceeded,
    ServerOverloaded,
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
pub struct CreateStateFailedDurations {
    pub total: u64,
    pub create_launcher: Option<u64>,
    pub code_startup: Option<u64>,
    pub create_state: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum CreateStateFailedReason {
    LauncherTerminated,
    Cancelled,
    ServerOverloaded,
    InvalidExecutableFile,
    ReadExecutableFileFailed,
    Unknown,
    #[serde(rename_all = "camelCase")]
    MaxDurationExceeded {
        at: AtCodeStartupOrCreateState,
    },
    #[serde(rename_all = "camelCase")]
    CodeTerminated {
        exit_code: Option<i32>,
        signal: Option<String>,
        oom: bool,
    },
    #[serde(rename_all = "camelCase")]
    Exception {
        at: AtCodeStartupOrCreateState,
        exception_details: Option<String>,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStateDurations {
    pub total: u64,
    pub create_launcher: Option<u64>,
    pub code_startup: Option<u64>,
    pub create_state: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CreateStateResult {
    #[serde(rename_all = "camelCase")]
    Failed {
        durations: CreateStateFailedDurations,
        error: CreateStateFailedReason,
    },
    #[serde(rename_all = "camelCase")]
    Success {
        durations: CreateStateDurations,
        state_id: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum CreateStateError {
    ModelNotFound,
    ModelToMountNotFound,
    StateForModelToMountNotFound,
    SnapshotForModelToMountNotFound,
    PersistentLauncherNotFound,
    SnapshotNoLongerExists,
    AccessDenied,
    QuotaExceeded,
    #[serde(rename_all = "camelCase")]
    DatasetNotFound {
        dataset_id: String,
    },
    #[serde(rename_all = "camelCase")]
    DatasetKeyNotFound {
        dataset_id: String,
        dataset_key: String,
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
pub struct UploadStateResult {
    pub state_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum UploadStateError {
    ModelNotFound,
    ModelToMountNotFound,
    SnapshotForModelToMountNotFound,
    AccessDenied,
    QuotaExceeded,
    StateNotFound,
    StateIsActive,
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
pub struct CancelCreateStateResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum CancelCreateStateError {
    ModelNotFound,
    StateNotFound,
    StateAlreadyCreated,
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
pub struct CreatingState {
    pub id: String,
    pub name: String,
    pub started_at: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCreatingStatesResult {
    pub states: Vec<CreatingState>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetCreatingStatesError {
    ModelNotFound,
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
pub enum WaitForStateToBeCreatedResult {
    #[serde(rename_all = "camelCase")]
    Failed {
        durations: CreateStateFailedDurations,
        error: CreateStateFailedReason,
    },
    #[serde(rename_all = "camelCase")]
    Success {
        durations: CreateStateDurations,
        state_id: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum WaitForStateToBeCreatedError {
    ModelNotFound,
    StateNotFound,
    StateAlreadyCreated,
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
pub struct UpdateModelStateResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum UpdateModelStateError {
    ModelNotFound,
    StateNotFound,
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
pub struct SetActiveModelStateResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum SetActiveModelStateError {
    ModelNotFound,
    StateNotFound,
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
pub struct DeleteModelStateResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum DeleteModelStateError {
    ModelNotFound,
    StateNotFound,
    StateIsActive,
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
pub struct GetModelStateResult {
    #[serde(rename = "stateKeyNames")]
    pub data: Vec<super::super::StateKeyData>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetModelStateError {
    ModelNotFound,
    StateNotFound,
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
pub struct GetSnapshotStateResult {
    #[serde(rename = "stateKeyNames")]
    pub data: Vec<super::super::StateKeyData>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetSnapshotStateError {
    ModelNotFound,
    SnapshotNotFound,
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
pub struct TrainResult {
    /// A unique identifier which you should use in subsequent API calls.
    pub training_session_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum TrainError {
    ModelNotFound,
    PersistentLauncherNotFound,
    SnapshotNoLongerExists,
    AccessDenied,
    QuotaExceeded,
    #[serde(rename_all = "camelCase")]
    DatasetNotFound {
        dataset_id: String,
    },
    #[serde(rename_all = "camelCase")]
    DatasetKeyNotFound {
        dataset_id: String,
        dataset_key: String,
    },
    ModelToMountNoLongerExists,
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
pub struct TrainMetric {
    pub name: String,
    pub amount: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrainingStartDurations {
    pub create_launcher: Option<u64>,
    pub code_startup: Option<u64>,
    pub create_instantiated_model: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AtCodeStartupOrInstantiateModelOrTrainOrGetState {
    CodeStartup,
    InstantiateModel,
    Train,
    GetState,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum TrainingSessionFailReason {
    Cancelled,
    LauncherTerminated,
    ServerOverloaded,
    InvalidExecutableFile,
    ReadExecutableFileFailed,
    Unknown,
    #[serde(rename_all = "camelCase")]
    Exception {
        at: AtCodeStartupOrInstantiateModelOrTrainOrGetState,
        exception_details: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    CodeTerminated {
        exit_code: Option<i32>,
        signal: Option<String>,
        oom: bool,
    },
    #[serde(rename_all = "camelCase")]
    MaxDurationExceeded {
        at: AtCodeStartupOrInstantiateModelOrTrainOrGetState,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "state")]
pub enum TrainingStatus {
    Starting,
    #[serde(rename_all = "camelCase")]
    Running {
        start_durations: TrainingStartDurations,
        progress: f32,
    },
    #[serde(rename_all = "camelCase")]
    GettingState {
        start_durations: TrainingStartDurations,
        train_duration: u64,
    },
    #[serde(rename_all = "camelCase")]
    Completed {
        start_durations: TrainingStartDurations,
        train_duration: u64,
        get_state_duration: u64,
        finished_at: i64,
        created_state_id: String,
    },
    #[serde(rename_all = "camelCase")]
    Failed {
        start_durations: TrainingStartDurations,
        train_duration: Option<u64>,
        get_state_duration: Option<u64>,
        finished_at: i64,
        fail_reason: TrainingSessionFailReason,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTrainingStatusResult {
    pub id: String,
    pub model_id: String,
    pub new_state_name: String,
    pub created_at: i64,
    pub metrics: Vec<TrainMetric>,
    pub execution_location: ExecutionLocation,
    pub status: TrainingStatus,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetTrainingStatusError {
    ModelNotFound,
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
pub struct FetchedTrainingMetricEntry {
    pub timestamp: i64,
    #[serde(deserialize_with = "crate::tensor::deserialize_empty_owned_decthings_tensor")]
    pub data: OwnedDecthingsTensor,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchedTrainingMetric {
    pub name: String,
    pub start_index: u32,
    pub entries: Vec<FetchedTrainingMetricEntry>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTrainingMetricsResult {
    pub metrics: Vec<FetchedTrainingMetric>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetTrainingMetricsError {
    ModelNotFound,
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
pub struct SysinfoDataPoint {
    pub timestamp: i64,
    pub cpus: f32,
    pub memory: u32,
    pub disk: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTrainingSysinfoResult {
    pub sysinfo: Vec<SysinfoDataPoint>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetTrainingSysinfoError {
    ModelNotFound,
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
pub struct CancelTrainingSessionResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum CancelTrainingSessionError {
    ModelNotFound,
    TrainingSessionNotFound,
    TrainingSessionNotRunning,
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
pub struct ClearPreviousTrainingSessionResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum ClearPreviousTrainingSessionError {
    ModelNotFound,
    TrainingSessionNotFound,
    TrainingSessionRunning,
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
pub struct FailedEvaluationDurations {
    pub create_launcher: Option<u64>,
    pub code_startup: Option<u64>,
    pub create_instantiated_model: Option<u64>,
    pub evaluate: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AtCodeStartupOrInstantiateModelOrEvaluate {
    CodeStartup,
    InstantiateModel,
    Evaluate,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvalidOutputType {
    Invalid,
    NotApplicableToParameterDefinitions,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum FailedEvaluationError {
    LauncherTerminated,
    Cancelled,
    ServerOverloaded,
    InvalidExecutableFile,
    ReadExecutableFileFailed,
    Unknown,
    #[serde(rename_all = "camelCase")]
    MaxDurationExceeded {
        at: AtCodeStartupOrInstantiateModelOrEvaluate,
    },
    #[serde(rename_all = "camelCase")]
    CodeTerminated {
        exit_code: Option<i32>,
        signal: Option<String>,
        oom: bool,
    },
    #[serde(rename_all = "camelCase")]
    Exception {
        at: AtCodeStartupOrInstantiateModelOrEvaluate,
        exception_details: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    InvalidOutput {
        reason: InvalidOutputType,
        details: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluationDurations {
    pub create_launcher: Option<u64>,
    pub code_startup: Option<u64>,
    pub create_instantiated_model: Option<u64>,
    pub evaluate: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EvaluateResult {
    #[serde(rename_all = "camelCase")]
    Failed {
        total_duration: u64,
        durations: FailedEvaluationDurations,
        executed_on_launcher: ExecutionLocation,
        error: FailedEvaluationError,
    },
    #[serde(rename_all = "camelCase")]
    Success {
        total_duration: u64,
        durations: EvaluationDurations,
        executed_on_launcher: ExecutionLocation,
        output: Vec<DecthingsParameter>,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum EvaluateError {
    ModelNotFound,
    SnapshotNotFound,
    QuotaExceeded,
    #[serde(rename_all = "camelCase")]
    DatasetNotFound {
        dataset_id: String,
    },
    #[serde(rename_all = "camelCase")]
    DatasetKeyNotFound {
        dataset_id: String,
        dataset_key: String,
    },
    ModelToMountNoLongerExists,
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
pub struct RunningEvaluation {
    pub id: String,
    pub started_at: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinishedEvaluation {
    pub id: String,
    pub started_at: i64,
    pub finished_at: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEvaluationsResult {
    pub running: Vec<RunningEvaluation>,
    pub finished: Vec<FinishedEvaluation>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetEvaluationsError {
    ModelNotFound,
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
pub enum GetFinishedEvaluationResultResult {
    #[serde(rename_all = "camelCase")]
    Failed {
        total_duration: u64,
        durations: FailedEvaluationDurations,
        executed_on_launcher: ExecutionLocation,
        error: FailedEvaluationError,
    },
    #[serde(rename_all = "camelCase")]
    Success {
        total_duration: u64,
        durations: EvaluationDurations,
        executed_on_launcher: ExecutionLocation,
        output: Vec<DecthingsParameter>,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetFinishedEvaluationResultError {
    ModelNotFound,
    EvaluationNotFound,
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
pub struct CancelEvaluationResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum CancelEvaluationError {
    ModelNotFound,
    EvaluationNotFound,
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
pub struct SetUsedPersistentLaunchersForEvaluateResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum SetUsedPersistentLaunchersForEvaluateError {
    PersistentLauncherNotFound,
    ModelNotFound,
    SnapshotNoLongerExists,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UsedPersistentLauncherLevel {
    Launcher,
    CodeStart,
    InstantiatedModel,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsedPersistentLauncher {
    pub persistent_launcher_id: String,
    pub level: UsedPersistentLauncherLevel,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUsedPersistentLaunchersForEvaluateResult {
    pub used_persistent_launchers: Vec<UsedPersistentLauncher>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetUsedPersistentLaunchersForEvaluateError {
    ModelNotFound,
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
