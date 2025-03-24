use crate::{
    client::rpc::{ExecutionLocation, LauncherSpec, ParameterDefinitions, Tag, WeightKeyData},
    client::DecthingsParameter,
    tensor::OwnedDecthingsTensor,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateModelResult {
    /// A unique identifier which you should use in subsequent API calls.
    pub model_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum CreateModelError {
    NameAlreadyUsed,
    OrganizationNotFound,
    AccessDenied,
    ModelNotFound,
    ModelVersionNotFound,
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
pub struct UpdateModelResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum UpdateModelError {
    ModelNotFound,
    AccessDenied,
    NameAlreadyUsed,
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
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ModelOwner {
    #[serde(rename_all = "camelCase")]
    User { user_id: String, username: String },
    #[serde(rename_all = "camelCase")]
    Organization {
        organization_id: String,
        organization_name: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ModelAccess {
    Read,
    Readwrite,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ModelSource {
    #[serde(rename_all = "camelCase")]
    Code {
        filesystem_size_mebibytes: u32,
        block_size: u64,
        total_blocks: u64,
        free_blocks: u64,
        total_inodes: u64,
        free_inodes: u64,
    },
    #[serde(rename_all = "camelCase")]
    Model {
        model_id: String,
        version_id: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultLauncherSpecs {
    pub initialize_weights: LauncherSpec,
    pub evaluate: LauncherSpec,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxDurationsSeconds {
    pub code_startup: u32,
    pub instantiate_model: u32,
    pub initialize_weights: u32,
    pub train: u32,
    pub evaluate: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub domain: String,
    pub repository: String,
    pub reference: String,
    pub digest: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigFile {
    pub parameter_definitions: ParameterDefinitions,
    pub default_launcher_specs: DefaultLauncherSpecs,
    pub max_durations_seconds: MaxDurationsSeconds,
    pub image: Image,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MountedModel {
    pub model_id: String,
    pub version_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightKey {
    pub key: String,
    pub byte_size: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "state")]
pub enum VersionStatus {
    InitializingWeights,
    #[serde(rename_all = "camelCase")]
    Training {
        training_session_id: String,
    },
    #[serde(rename_all = "camelCase")]
    Created {
        weights: Vec<WeightKey>,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelVersion {
    pub id: String,
    pub name: String,
    pub created_at: i64,
    pub filesystem_size_mebibytes: u32,
    pub config: ConfigFile,
    pub mounted_models: Vec<MountedModel>,
    /// IDs of all the training operations that have been performed to reach this state.
    pub training_operations: Vec<String>,
    pub status: VersionStatus,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub id: String,
    pub name: String,
    pub description: String,
    pub public_access: bool,
    pub created_at: i64,
    pub tags: Vec<Tag>,
    pub owner: ModelOwner,
    pub access: ModelAccess,
    pub source: ModelSource,
    pub versions: Vec<ModelVersion>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetModelsResult {
    pub models: Vec<Model>,
    /// The total number of models that matched the filter.
    pub total: u32,
    pub offset: u32,
    pub limit: u32,
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
    InvalidModelSourceType,
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
pub struct InitializeWeightsDurations {
    pub total: u64,
    pub create_launcher: Option<u64>,
    pub code_startup: Option<u64>,
    pub initialize_weights: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeWeightsFailedDurations {
    pub total: u64,
    pub create_launcher: Option<u64>,
    pub code_startup: Option<u64>,
    pub initialize_weights: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ModelFunction {
    #[serde(rename = "codeStartup")]
    CodeStartup,
    #[serde(rename = "initializeWeights")]
    InitializeWeights,
    #[serde(rename = "loadWeights")]
    LoadWeights,
    Evaluate,
    Train,
    #[serde(rename = "getWeights")]
    GetWeights,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum InitializeWeightsFailedReason {
    Cancelled,
    InvalidExecutableFile,
    ReadExecutableFileFailed,
    LauncherTerminated,
    #[serde(rename_all = "camelCase")]
    MaxDurationExceeded {
        at: ModelFunction,
    },
    #[serde(rename_all = "camelCase")]
    CodeTerminated {
        exit_code: Option<i32>,
        signal: Option<String>,
        oom: bool,
    },
    #[serde(rename_all = "camelCase")]
    Exception {
        at: ModelFunction,
        exception_details: Option<String>,
    },
    ServerOverloaded,
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateModelVersionResult {
    /// A unique identifier which you should use in subsequent API calls.
    pub version_id: String,
    pub initialize_weights_durations: InitializeWeightsDurations,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum CreateModelVersionError {
    ModelNotFound,
    AccessDenied,
    QuotaExceeded,
    ModelToMountNotFound,
    VersionForModelToMountNotFound,
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
    InitializeWeightsFailed {
        durations: InitializeWeightsFailedDurations,
        reason: InitializeWeightsFailedReason,
    },
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
pub struct CreateModelVersionUploadWeightsResult {
    /// A unique identifier which you should use in subsequent API calls.
    pub version_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum CreateModelVersionUploadWeightsError {
    ModelNotFound,
    AccessDenied,
    QuotaExceeded,
    ModelToMountNotFound,
    VersionForModelToMountNotFound,
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
pub struct UpdateModelVersionResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum UpdateModelVersionError {
    ModelNotFound,
    ModelVersionNotFound,
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
pub struct GetWeightsResult {
    #[serde(skip_deserializing)]
    pub data: Vec<WeightKeyData>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetWeightsError {
    ModelNotFound,
    ModelVersionNotFound,
    WeightKeyNotFound,
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
pub struct DeleteModelVersionResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum DeleteModelVersionError {
    ModelNotFound,
    ModelVersionNotFound,
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
pub struct TrainResult {
    /// A unique identifier which you should use in subsequent API calls.
    pub training_session_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum TrainError {
    ModelNotFound,
    ModelVersionNotFound,
    PersistentLauncherNotFound,
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
    pub instantiate_model: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum TrainingSessionFailReason {
    Cancelled,
    InvalidExecutableFile,
    ReadExecutableFileFailed,
    LauncherTerminated,
    #[serde(rename_all = "camelCase")]
    MaxDurationExceeded {
        at: ModelFunction,
    },
    #[serde(rename_all = "camelCase")]
    CodeTerminated {
        exit_code: Option<i32>,
        signal: Option<String>,
        oom: bool,
    },
    #[serde(rename_all = "camelCase")]
    Exception {
        at: ModelFunction,
        exception_details: Option<String>,
    },
    ServerOverloaded,
    Unknown,
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
    GettingWeights {
        start_durations: TrainingStartDurations,
        train_duration: u64,
    },
    #[serde(rename_all = "camelCase")]
    Completed {
        start_durations: TrainingStartDurations,
        train_duration: u64,
        get_weights_duration: u64,
        finished_at: i64,
        created_version_id: String,
    },
    #[serde(rename_all = "camelCase")]
    Failed {
        start_durations: TrainingStartDurations,
        train_duration: Option<u64>,
        get_weights_duration: Option<u64>,
        finished_at: i64,
        fail_reason: TrainingSessionFailReason,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTrainingStatusResult {
    pub id: String,
    pub model_id: String,
    pub version_id: String,
    pub new_version_name: String,
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
    #[serde(deserialize_with = "super::super::deserialize_empty_owned_decthings_tensor")]
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
pub struct EvaluateDurations {
    pub total: u64,
    pub create_launcher: Option<u64>,
    pub code_startup: Option<u64>,
    pub instantiate_model: Option<u64>,
    pub evaluate: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateFailedDurations {
    pub total: u64,
    pub create_launcher: Option<u64>,
    pub code_startup: Option<u64>,
    pub instantiate_model: Option<u64>,
    pub evaluate: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvalidOutputType {
    Invalid,
    NotApplicableToParameterDefinitions,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum EvaluateFailedReason {
    Cancelled,
    InvalidExecutableFile,
    ReadExecutableFileFailed,
    LauncherTerminated,
    #[serde(rename_all = "camelCase")]
    MaxDurationExceeded {
        at: ModelFunction,
    },
    #[serde(rename_all = "camelCase")]
    CodeTerminated {
        exit_code: Option<i32>,
        signal: Option<String>,
        oom: bool,
    },
    #[serde(rename_all = "camelCase")]
    Exception {
        at: ModelFunction,
        exception_details: Option<String>,
    },
    ServerOverloaded,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidOutput {
        reason: InvalidOutputType,
        details: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateResult {
    pub durations: EvaluateDurations,
    pub executed_on_launcher: ExecutionLocation,
    pub output: Vec<DecthingsParameter>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum EvaluateError {
    ModelNotFound,
    ModelVersionNotFound,
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
    #[serde(rename_all = "camelCase")]
    EvaluateFailed {
        durations: EvaluateFailedDurations,
        executed_on_launcher: ExecutionLocation,
        reason: EvaluateFailedReason,
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
pub struct RunningEvaluation {
    pub id: String,
    pub version_id: String,
    pub started_at: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinishedEvaluation {
    pub id: String,
    pub version_id: String,
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
pub struct GetFinishedEvaluationResultResult {
    pub durations: EvaluateDurations,
    pub executed_on_launcher: ExecutionLocation,
    pub output: Vec<DecthingsParameter>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetFinishedEvaluationResultError {
    ModelNotFound,
    EvaluationNotFound,
    #[serde(rename_all = "camelCase")]
    EvaluateFailed {
        durations: EvaluateFailedDurations,
        executed_on_launcher: ExecutionLocation,
        reason: EvaluateFailedReason,
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
    ModelVersionNotFound,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
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
    ModelVersionNotFound,
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
