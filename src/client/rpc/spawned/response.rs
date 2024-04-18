use serde::Deserialize;

use crate::client::rpc::ExecutionLocation;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum SpawnedCommandTerminatedReason {
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "params")]
pub enum SpawnedEvent {
    #[serde(rename_all = "camelCase")]
    Exit {
        spawned_command_id: String,
        reason: SpawnedCommandTerminatedReason,
    },
    #[serde(rename_all = "camelCase")]
    Stdout {
        spawned_command_id: String,
        #[serde(skip_deserializing)]
        data: bytes::Bytes,
    },
    #[serde(rename_all = "camelCase")]
    Stderr {
        spawned_command_id: String,
        #[serde(skip_deserializing)]
        data: bytes::Bytes,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnCommandResult {
    /// A unique identifier which you should use in subsequent API calls.
    pub spawned_command_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum SpawnCommandError {
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
pub struct SpawnCommandForModelResult {
    /// A unique identifier which you should use in subsequent API calls.
    pub spawned_command_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum SpawnCommandForModelError {
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
pub struct TerminateSpawnedCommandResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum TerminateSpawnedCommandError {
    SpawnedCommandNotFound,
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
pub struct SpawnedCommand {
    pub id: String,
    pub started_at: i64,
    pub model_id: Option<String>,
    pub execution_location: ExecutionLocation,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSpawnedCommandsResult {
    pub spawned_commands: Vec<SpawnedCommand>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetSpawnedCommandsError {
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
pub struct WriteToSpawnedCommandResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum WriteToSpawnedCommandError {
    SpawnedCommandNotFound,
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
pub struct SpawnedSubscribeToEventsResult {}

#[cfg(feature = "events")]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum SpawnedSubscribeToEventsError {
    SpawnedCommandNotFound,
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
pub struct SpawnedUnsubscribeFromEventsResult {}

#[cfg(feature = "events")]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum SpawnedUnsubscribeFromEventsError {
    NotSubscribed,
    TooManyRequests,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}
