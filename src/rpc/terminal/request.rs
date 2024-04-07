use serde::Serialize;

use crate::rpc::ExecutionLocation;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilesystemAccess<'a> {
    pub model_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalOptions<'a> {
    /// Adds filesystem access for additional models. Useful for copying files between models for example.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_filesystem_access: Option<&'a [FilesystemAccess<'a>]>,
    /// Will automatically terminate the session if no input is provided for this amount of time. Default: 1800.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_after_inactive_seconds: Option<u32>,
    /// Terminal columns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cols: Option<u16>,
    /// Terminal rows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<u16>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchTerminalSessionParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// Which launcher to use for running the command.
    pub execution_location: ExecutionLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<TerminalOptions<'a>>,
    /// If true, immediately subscribes you to events "data" and "exit" for the terminal. Default: true.
    #[cfg(feature = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_to_events: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminateTerminalSessionParams<'a> {
    /// The terminal session's id.
    pub terminal_session_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTerminalSessionsParams<'a, S: AsRef<str>> {
    /// Which sessions to fetch. If unspecified, all running terminals will be fetched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_session_ids: Option<&'a [S]>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteToTerminalSessionParams<'a, T: AsRef<[u8]>> {
    /// The terminal session's id.
    pub terminal_session_id: &'a str,
    #[serde(skip_serializing)]
    pub data: T,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalSessionSize {
    pub cols: u16,
    pub rows: u16,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResizeTerminalSessionParams<'a> {
    /// The terminal session's id.
    pub terminal_session_id: &'a str,
    /// New size to set.
    pub size: TerminalSessionSize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddFilesystemAccessForTerminalSessionParams<'a> {
    /// The terminal session's id.
    pub terminal_session_id: &'a str,
    /// Identifier of the model to add access to.
    pub model_id: &'a str,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalSubscribeToEventsParams<'a> {
    /// The terminal session's id.
    pub terminal_session_id: &'a str,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalUnsubscribeFromEventsParams<'a> {
    /// The terminal session's id.
    pub terminal_session_id: &'a str,
}
