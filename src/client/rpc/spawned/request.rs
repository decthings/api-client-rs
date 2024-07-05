use crate::client::rpc::{ExecutionLocationProvider, LauncherConfig};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnedCommandOptions<'a> {
    /// Will automatically terminate the command after this amount of time. Default: 3600.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u32>,
    /// Will automatically terminate the command if no output is received from the process for this amount of time.
    /// Default: 600.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_after_inactive_seconds: Option<u32>,
    /// LauncherConfig to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launcher_config: Option<&'a LauncherConfig>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnCommandParams<'a, S: AsRef<str>> {
    /// Which launcher to use for running the command.
    pub execution_location: ExecutionLocationProvider<'a>,
    /// Name of the command to run, without any arguments.
    pub command: &'a str,
    /// Arguments to pass to the command
    #[serde(serialize_with = "super::super::serialize_asref_str_seq")]
    pub args: &'a [S],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<SpawnedCommandOptions<'a>>,
    /// If true, immediately subscribes you to events "stdout", "stderr" and "exit" for the spawned command. Default:
    /// true.
    #[cfg(feature = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_to_events: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilesystemAccess<'a> {
    pub model_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(bound(serialize = ""))]
pub struct SpawnCommandForModelParams<'a, S: AsRef<str>> {
    /// The model's id.
    pub model_id: &'a str,
    /// Which launcher to use for running the command.
    pub execution_location: ExecutionLocationProvider<'a>,
    /// Name of the command to run, without any arguments.
    pub command: &'a str,
    /// Arguments to pass to the command
    #[serde(serialize_with = "super::super::serialize_asref_str_seq")]
    pub args: &'a [S],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<SpawnedCommandOptions<'a>>,
    /// If true, immediately subscribes you to events "stdout", "stderr" and "exit" for the spawned command. Default:
    /// true.
    #[cfg(feature = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_to_events: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminateSpawnedCommandParams<'a> {
    /// The spawned command's id.
    pub spawned_command_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSpawnedCommandsParams<'a, S: AsRef<str>> {
    /// Which spawned commands to fetch. If unspecified, all running commands will be fetched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawned_command_ids: Option<&'a [S]>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteToSpawnedCommandParams<'a, T: AsRef<[u8]>> {
    /// The spawned command's id.
    pub spawned_command_id: &'a str,
    #[serde(skip_serializing)]
    pub data: T,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnedSubscribeToEventsParams<'a> {
    /// The spawned command's id.
    pub spawned_command_id: &'a str,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnedUnsubscribeFromEventsParams<'a> {
    /// The spawned command's id.
    pub spawned_command_id: &'a str,
}
