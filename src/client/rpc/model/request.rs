use crate::client::{
    rpc::{ExecutionLocationProvider, LauncherConfig, LauncherSpec, ParameterDefinitions},
    DecthingsParameterProvider,
};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct StateKeyData<'a, D: AsRef<[u8]>> {
    pub key: &'a str,
    pub data: D,
}

impl<D: AsRef<[u8]>> serde::Serialize for StateKeyData<'_, D> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.key)
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "method")]
#[serde(bound(serialize = ""))]
pub enum CreateModelInitialState<'a, D: AsRef<[u8]>> {
    Copy,
    #[serde(rename_all = "camelCase")]
    Create {
        name: &'a str,
        params: Vec<DecthingsParameterProvider<'a>>,
        launcher_spec: &'a LauncherSpec,
    },
    #[serde(rename_all = "camelCase")]
    Upload {
        name: &'a str,
        #[serde(rename = "stateKeyNames")]
        data: &'a [StateKeyData<'a, D>],
    },
}

impl<'a> CreateModelInitialState<'a, &'static [u8]> {
    pub fn copy() -> Self {
        Self::Copy
    }
}
impl<'a> CreateModelInitialState<'a, &'static [u8]> {
    pub fn create(
        name: &'a str,
        params: Vec<DecthingsParameterProvider<'a>>,
        launcher_spec: &'a LauncherSpec,
    ) -> Self {
        Self::Create {
            name,
            params,
            launcher_spec,
        }
    }
}
impl<'a, D: AsRef<[u8]>> CreateModelInitialState<'a, D> {
    pub fn upload(name: &'a str, data: &'a [StateKeyData<'a, D>]) -> Self {
        Self::Upload { name, data }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
#[serde(bound(serialize = ""))]
pub enum CreateModelOptions<'a, D: AsRef<[u8]>> {
    #[serde(rename_all = "camelCase")]
    Code {
        /// Tags are used to specify things like model type (image classifier, etc.) and other metadata.
        tags: Option<&'a [super::super::TagProvider<'a>]>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parameter_definitions: Option<ParameterDefinitions>,
        language: super::response::Language,
        /// At the time of writing, presets "none", "empty", "tensorflowjs", "pytorch" and "tensorflow" are available.
        #[serde(skip_serializing_if = "Option::is_none")]
        preset: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        wasm: Option<bool>,
    },
    #[serde(rename_all = "camelCase")]
    Upload {
        /// Tags are used to specify things like model type (image classifier, etc.) and other metadata.
        tags: Option<&'a [super::super::TagProvider<'a>]>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parameter_definitions: Option<ParameterDefinitions>,
        /// At the time of writing, formats "tflite" and "onnx" are available.
        format: &'a str,
        #[serde(skip_serializing)]
        data: &'a [u8],
    },
    #[serde(rename_all = "camelCase")]
    BasedOnModelSnapshot {
        model_id: &'a str,
        snapshot_id: &'a str,
        initial_state: CreateModelInitialState<'a, D>,
    },
    #[serde(rename_all = "camelCase")]
    FromExisting {
        model_id: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        snapshot_id: Option<&'a str>,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(bound(serialize = ""))]
pub struct CreateModelParams<'a, D: AsRef<[u8]>> {
    /// The model's name.
    pub name: &'a str,
    /// A description of the model.
    pub description: &'a str,
    /// Required configuration for this model, such as model type, language to use, etc.
    pub options: CreateModelOptions<'a, D>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WaitForModelToBeCreatedParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteModelParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotModelParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The name of the snapshot.
    pub snapshot_name: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSnapshotProperties<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSnapshotParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The snapshot's id.
    pub snapshot_id: &'a str,
    /// Properties and values to change. Empty fields will not be changed.
    pub properties: UpdateSnapshotProperties<'a>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSnapshotParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The snapshot's id.
    pub snapshot_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionalDefaultLauncherSpecs<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_state: Option<&'a LauncherSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluate: Option<&'a LauncherSpec>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateModelProperties<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<&'a [super::super::TagProvider<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_definitions: Option<ParameterDefinitions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_launcher_specs: Option<OptionalDefaultLauncherSpecs<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_durations_seconds: Option<super::response::MaxDurationsSeconds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launcher_config: Option<&'a LauncherConfig>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateModelParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// Properties and values to change. Empty fields will not be changed.
    pub properties: UpdateModelProperties<'a>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetModelsParams<'a, S: AsRef<str>> {
    /// Which models to fetch. If unspecified, all models will be fetched.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "super::super::serialize_option_asref_str_seq")]
    pub model_ids: Option<&'a [S]>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetFilesystemSizeParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The new size to use.
    pub new_filesystem_size_mebibytes: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFilesystemUsageParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStateMountModel<'a> {
    /// Id of the other model to mount.
    pub model_id: &'a str,
    /// Specifies which state on the other model to use. Defaults to the active state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_id: Option<&'a str>,
    /// If specified, this snapshot on the other model will be used. Cannot be used together with stateId, as the state
    /// in the snapshot will be used if snapshotId is specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStateParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// Name of the new state.
    pub name: &'a str,
    /// Parameters to provide to the createModelState function on the running model.
    pub params: Vec<DecthingsParameterProvider<'a>>,
    /// Allows your model to access to files and state of these additional models. Can be useful for merging models
    /// together.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_models: Option<&'a [CreateStateMountModel<'a>]>,
    /// Which launcher to use for running the operation.
    pub execution_location: ExecutionLocationProvider<'a>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MountModel<'a> {
    /// Id of the other model to mount.
    pub model_id: &'a str,
    /// If specified, this snapshot on the other model will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadStateParams<'a, S: AsRef<str>, D: AsRef<[u8]>> {
    /// The model's id.
    pub model_id: &'a str,
    /// Name of the new state.
    pub name: &'a str,
    /// Data to upload.
    #[serde(skip_serializing)]
    pub data: &'a [StateKeyData<'a, D>],
    /// If provided, these states will be deleted when the new state has been uploaded, in a single atomic operation.
    /// If either the upload or the delete fails, both the upload and the delete operations are aborted and an error is
    /// returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "super::super::serialize_option_asref_str_seq")]
    pub delete_states: Option<&'a [S]>,
    /// Allows your model to access to files and state of these additional models. Can be useful for merging models
    /// together.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_models: Option<&'a [MountModel<'a>]>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelCreateStateParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The state's id.
    pub state_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCreatingStatesParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WaitForStateToBeCreatedParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The state's id.
    pub state_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateModelStateProperties<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateModelStateParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The state's id.
    pub state_id: &'a str,
    /// Properties and values to change. Empty fields will not be changed.
    pub properties: UpdateModelStateProperties<'a>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetActiveModelStateParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The state's id.
    pub state_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteModelStateParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The state's id.
    pub state_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetModelStateParams<'a, S: AsRef<str>> {
    /// The model's id.
    pub model_id: &'a str,
    /// The state's id. Defaults to the active state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_id: Option<&'a str>,
    /// Which keys to fetch. Defaults to all keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "super::super::serialize_option_asref_str_seq")]
    pub keys: Option<&'a [S]>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSnapshotStateParams<'a, S: AsRef<str>> {
    /// The model's id.
    pub model_id: &'a str,
    /// The snapshot's id.
    pub snapshot_id: &'a str,
    /// Which keys to fetch. Defaults to all keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "super::super::serialize_option_asref_str_seq")]
    pub keys: Option<&'a [S]>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrainParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// Which state to use when instantiating the model. Defaults to the active state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_id: Option<&'a str>,
    /// A name to give the new state once it is created.
    pub new_state_name: &'a str,
    /// Parameters to provide to the train function on the running model.
    pub params: Vec<DecthingsParameterProvider<'a>>,
    /// Which launcher to use for running the operation.
    pub execution_location: ExecutionLocationProvider<'a>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTrainingStatusParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The training session's id.
    pub training_session_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrainingMetricsToFetch<'a> {
    pub name: &'a str,
    pub start_index: u32,
    pub amount: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTrainingMetricsParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The training session's id.
    pub training_session_id: &'a str,
    /// Which metrics to fetch
    pub metrics: &'a [TrainingMetricsToFetch<'a>],
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTrainingSysinfoParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The training session's id.
    pub training_session_id: &'a str,
    /// If specified, only data points after this time are included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_timestamp: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelTrainingSessionParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The training session's id.
    pub training_session_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearPreviousTrainingSessionParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The training session's id.
    pub training_session_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// Parameters to provide to the train function on the running model.
    pub params: Vec<DecthingsParameterProvider<'a>>,
    /// If provided, the snapshot with this id will be evaluated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEvaluationsParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFinishedEvaluationResultParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The evaluation's id.
    pub evaluation_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelEvaluationParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The evaluation's id.
    pub evaluation_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentLauncherToUse<'a> {
    pub persistent_launcher_id: &'a str,
    pub level: super::response::UsedPersistentLauncherLevel,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetUsedPersistentLaunchersForEvaluateParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    pub persistent_launchers: &'a [PersistentLauncherToUse<'a>],
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUsedPersistentLaunchersForEvaluateParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
}
