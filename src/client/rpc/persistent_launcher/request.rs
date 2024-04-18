use serde::Serialize;

use crate::client::rpc::LauncherSpec;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePersistentLauncherParams<'a> {
    /// A name for the launcher.
    pub name: &'a str,
    /// Launcher specification to use.
    pub spec: &'a LauncherSpec,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPersistentLaunchersParams<'a, S: AsRef<str>> {
    /// Which launchers to fetch. If unspecified, all persistent launchers will be fetched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_launcher_ids: Option<&'a [S]>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSysinfoParams<'a> {
    /// The persistent launcher's id.
    pub persistent_launcher_id: &'a str,
    /// If specified, only data points after this time are included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_timestamp: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletePersistentLauncherParams<'a> {
    /// The persistent launcher's id.
    pub persistent_launcher_id: &'a str,
}
