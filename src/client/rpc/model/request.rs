use crate::client::{
    rpc::{ExecutionLocationProvider, ParameterDefinitions, TagProvider, WeightKeyDataProvider},
    DecthingsParameterProvider,
};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Language {
    Javascript,
    Typescript,
    Python,
    Rust,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum CreateModelOptions<'a> {
    #[serde(rename_all = "camelCase")]
    Code {
        #[serde(skip_serializing_if = "Option::is_none")]
        parameter_definitions: Option<ParameterDefinitions>,
        language: Language,
        #[serde(skip_serializing_if = "Option::is_none")]
        preset: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        wasm: Option<bool>,
    },
    #[serde(rename_all = "camelCase")]
    BasedOnModel {
        model_id: &'a str,
        version_id: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    DuplicateExisting { model_id: &'a str },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateModelParams<'a> {
    /// The model's name.
    pub name: &'a str,
    /// A description of the model.
    pub description: &'a str,
    /// If true, all Decthings users can find and use this model. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access: Option<bool>,
    /// Tags are used to specify things like model type (image classifier, etc.) and other metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<&'a [TagProvider<'a>]>,
    /// Required configuration for this model, such as model type, language to use, etc.
    pub options: CreateModelOptions<'a>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteModelParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateModelProperties<'a> {
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
pub struct UpdateModelParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// Properties and values to change. Empty fields will not be changed.
    pub properties: UpdateModelProperties<'a>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetModelsFilter<'a, S: AsRef<str>> {
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
#[serde(bound = "")]
pub struct GetModelsParams<'a, S: AsRef<str>> {
    /// Number of items from the results to skip. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// Max number of items to return. Defaults to 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// If specified, determines which items to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<GetModelsFilter<'a, S>>,
    /// Specifies a field in the returned items to sort by. Defaults to "createdAt".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
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
pub struct MountModel<'a> {
    /// Id of the other model to mount.
    pub model_id: &'a str,
    /// Version within the other model to mount.
    pub version_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateModelVersionParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The name of the version.
    pub version_name: &'a str,
    /// Parameters to provide to the initializeWeights function on the running model.
    pub params: Vec<DecthingsParameterProvider<'a>>,
    /// Allows your model to execute these additional models. Can be useful for merging models together.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_models: Option<&'a [MountModel<'a>]>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateModelVersionUploadWeightsParams<'a, D: AsRef<[u8]>> {
    /// The model's id.
    pub model_id: &'a str,
    /// The name of the version.
    pub version_name: &'a str,
    /// Data to upload.
    #[serde(skip_serializing)]
    pub data: &'a [WeightKeyDataProvider<'a, D>],
    /// Allows your model to execute these additional models. Can be useful for merging models together.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_models: Option<&'a [MountModel<'a>]>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateVersionProperties<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateModelVersionParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The version's id.
    pub version_id: &'a str,
    /// Properties and values to change. Empty fields will not be changed.
    pub properties: UpdateVersionProperties<'a>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWeightsParams<'a, S: AsRef<str>> {
    /// The model's id.
    pub model_id: &'a str,
    /// The model version's id.
    pub version_id: &'a str,
    /// Which weight keys to fetch. Defaults to all keys.
    #[serde(serialize_with = "super::super::serialize_option_asref_str_seq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<&'a [S]>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteModelVersionParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The model version's id.
    pub version_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrainParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The model version to use.
    pub version_id: &'a str,
    /// A name to give the new model version once it is created.
    pub new_version_name: &'a str,
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
    /// The model version to evaluate.
    pub version_id: &'a str,
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
    /// The model version's id.
    pub version_id: &'a str,
    pub persistent_launchers: &'a [PersistentLauncherToUse<'a>],
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUsedPersistentLaunchersForEvaluateParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The model version's id.
    pub version_id: &'a str,
}
