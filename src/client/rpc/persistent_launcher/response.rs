use crate::client::rpc::LauncherSpec;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePersistentLauncherResult {
    /// A unique identifier which you should use in subsequent API calls.
    pub persistent_launcher_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum CreatePersistentLauncherError {
    QuotaExceeded,
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
#[serde(rename_all = "snake_case")]
pub enum PersistentLauncherPreviousState {
    Exit,
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum PersistentLauncherState {
    Creating,
    Active,
    Deleting,
    #[serde(rename_all = "camelCase")]
    Recreating {
        previous: PersistentLauncherPreviousState,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PersistentLauncherRunningType {
    Terminal,
    Spawned,
    Debug,
    Train,
    Evaluate,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentLauncherRunning {
    pub id: String,
    pub r#type: PersistentLauncherRunningType,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistentLauncher {
    pub id: String,
    pub name: String,
    pub created_at: Option<i64>,
    pub spec: LauncherSpec,
    pub state: PersistentLauncherState,
    pub running: Vec<PersistentLauncherRunning>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPersistentLaunchersResult {
    pub persistent_launchers: Vec<PersistentLauncher>,
    /// The total number of datasets that matched the filter.
    pub total: u32,
    pub offset: u32,
    pub limit: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetPersistentLaunchersError {
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
pub struct GetSysinfoResult {
    pub sysinfo: Vec<SysinfoDataPoint>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetSysinfoError {
    PersistentLauncherNotFound,
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
pub struct DeletePersistentLauncherResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum DeletePersistentLauncherError {
    PersistentLauncherNotFound,
    PersistentLauncherBeingDeleted,
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
