use crate::client::rpc::LauncherSpec;
use serde::Serialize;

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
pub struct GetPersistentLaunchersFilter<'a, S: AsRef<str>> {
    #[serde(serialize_with = "super::super::serialize_option_asref_str_seq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<&'a [S]>,
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
pub struct GetPersistentLaunchersParams<'a, S: AsRef<str>> {
    /// Number of items from the results to skip. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// Max number of items to return. Defaults to 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// If specified, determines which items to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<GetPersistentLaunchersFilter<'a, S>>,
    /// Specifies a field in the returned items to sort by. Defaults to "createdAt".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
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
