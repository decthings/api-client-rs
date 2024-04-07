use serde::Deserialize;

use crate::rpc::ExecutionLocation;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum TerminalSessionTerminatedReason {
    TerminatedOnRequest,
    LauncherTerminated,
    InactiveTimeout,
    Unknown,
    #[serde(rename_all = "camelCase")]
    ProcessExit {
        exit_code: Option<i32>,
        signal: Option<String>,
        oom: bool,
    },
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "params")]
pub enum TerminalEvent {
    #[serde(rename_all = "camelCase")]
    Exit {
        terminal_session_id: String,
        reason: TerminalSessionTerminatedReason,
    },
    #[serde(rename_all = "camelCase")]
    Data {
        terminal_session_id: String,
        #[serde(skip_deserializing)]
        data: bytes::Bytes,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchTerminalSessionResult {
    /// A unique identifier which you should use in subsequent API calls.
    pub terminal_session_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum LaunchTerminalSessionError {
    ModelNotFound,
    PersistentLauncherNotFound,
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
pub struct TerminateTerminalSessionResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum TerminateTerminalSessionError {
    TerminalSessionNotFound,
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
pub struct TerminalSession {
    pub id: String,
    pub started_at: i64,
    pub model_id: String,
    pub execution_location: ExecutionLocation,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTerminalSessionsResult {
    pub terminal_sessions: Vec<TerminalSession>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetTerminalSessionsError {
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
pub struct WriteToTerminalSessionResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum WriteToTerminalSessionError {
    TerminalSessionNotFound,
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
pub struct ResizeTerminalSessionResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum ResizeTerminalSessionError {
    TerminalSessionNotFound,
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
pub struct AddFilesystemAccessForTerminalSessionResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum AddFilesystemAccessForTerminalSessionError {
    TerminalSessionNotFound,
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

#[cfg(feature = "events")]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalSubscribeToEventsResult {}

#[cfg(feature = "events")]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum TerminalSubscribeToEventsError {
    TerminalSessionNotFound,
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
pub struct TerminalUnsubscribeFromEventsResult {}

#[cfg(feature = "events")]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum TerminalUnsubscribeFromEventsError {
    NotSubscribed,
    TooManyRequests,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}
