use serde::Serialize;

use crate::{rpc::ExecutionLocationProvider, DecthingsParameterProvider};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugSessionOptions {
    /// Will automatically terminate the session if no method is called on the debug session for this amount of time.
    /// Default: 1800.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_after_inactive_seconds: Option<u32>,
    /// Whether to run the process in remote debugger mode, allowing you to place breakpoints and step through the
    /// code. Default: true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_inspector: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchDebugSessionParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// Which launcher to use for running the session.
    pub execution_location: ExecutionLocationProvider<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<DebugSessionOptions>,
    /// If true, immediately subscribes you to events "exit", "stdout", "stderr", "initialized" and
    /// "remoteInspectorData" for the debug session. Default: true.
    #[cfg(feature = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_to_events: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDebugSessionsParams<'a, S: AsRef<str>> {
    /// Which sessions to fetch. If unspecified, all sessions will be fetched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_session_ids: Option<&'a [S]>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminateDebugSessionParams<'a> {
    /// The debug session's id.
    pub debug_session_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallCreateModelStateParams<'a> {
    /// The debug session's id.
    pub debug_session_id: &'a str,
    /// Parameters to provide to the function.
    pub params: Vec<DecthingsParameterProvider<'a>>,
}

#[derive(Debug, Clone)]
pub struct StateData<'a, D: AsRef<[u8]>> {
    pub key: &'a str,
    pub data: D,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum StateDataProvider<'a, D: AsRef<[u8]>> {
    #[serde(rename_all = "camelCase")]
    Data {
        #[serde(skip_serializing)]
        data: &'a [StateData<'a, D>],
    },
    #[serde(rename_all = "camelCase")]
    DataId { data_id: &'a str },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(bound(serialize = ""))]
pub struct CallInstantiateModelParams<'a, D: AsRef<[u8]>> {
    /// The debug session's id.
    pub debug_session_id: &'a str,
    /// Data to use as model state.
    pub state_data: StateDataProvider<'a, D>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallTrainParams<'a> {
    /// The debug session's id.
    pub debug_session_id: &'a str,
    /// Identifier of the instantiated model to use, as returned by the 'callInstantiateModel' function.
    pub instantiated_model_id: &'a str,
    /// Parameters to provide to the function.
    pub params: Vec<DecthingsParameterProvider<'a>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugGetTrainingStatusParams<'a> {
    /// The debug session's id.
    pub debug_session_id: &'a str,
    /// Training session identifier, as returned by the 'callTrain' function.
    pub training_session_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugTrainingMetricsToFetch<'a> {
    pub name: &'a str,
    pub start_index: u32,
    pub amount: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugGetTrainingMetricsParams<'a> {
    /// The debug session's id.
    pub debug_session_id: &'a str,
    /// Training session identifier, as returned by the 'callTrain' function.
    pub training_session_id: &'a str,
    /// Which metrics to fetch.
    pub metrics: &'a [DebugTrainingMetricsToFetch<'a>],
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugCancelTrainingSessionParams<'a> {
    /// The debug session's id.
    pub debug_session_id: &'a str,
    /// Training session identifier, as returned by the 'callTrain' function.
    pub training_session_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallEvaluateParams<'a> {
    /// The debug session's id.
    pub debug_session_id: &'a str,
    /// Identifier of the instantiated model to use, as returned by the 'callInstantiateModel' function.
    pub instantiated_model_id: &'a str,
    /// Parameters to provide to the function.
    pub params: Vec<DecthingsParameterProvider<'a>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallGetModelStateParams<'a> {
    /// The debug session's id.
    pub debug_session_id: &'a str,
    /// Identifier of the instantiated model to use, as returned by the 'callInstantiateModel' function.
    pub instantiated_model_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadStateDataParams<'a, S: AsRef<str>> {
    /// The debug session's id.
    pub debug_session_id: &'a str,
    /// The data's id, as returned by 'callCreateModelState' or 'callGetModelState'.
    pub data_id: &'a str,
    /// Which state keys to fetch. Defaults to all keys.
    #[serde(serialize_with = "super::super::serialize_option_asref_str_seq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<&'a [S]>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendToRemoteInspectorParams<'a, T: AsRef<[u8]>> {
    /// The debug session's id.
    pub debug_session_id: &'a str,
    #[serde(skip_serializing)]
    pub data: T,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugSubscribeToEventsParams<'a> {
    /// The debug session's id.
    pub debug_session_id: &'a str,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugUnsubscribeFromEventsParams<'a> {
    /// The debug session's id.
    pub debug_session_id: &'a str,
}
