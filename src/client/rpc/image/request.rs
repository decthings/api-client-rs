use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRepositoryParams<'a> {
    /// The repository's name.
    pub name: &'a str,
    /// A description of the repository.
    pub description: &'a str,
    /// If true, all Decthings users can find and use this repository. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRepositoryProperties<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRepositoryParams<'a> {
    /// The repository's id.
    pub name: &'a str,
    /// Properties and values to change. Empty fields will not be changed.
    pub properties: UpdateRepositoryProperties<'a>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRepositoryParams<'a> {
    /// The repository's name.
    pub name: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRepositoriesFilter<'a, S: AsRef<str>> {
    #[serde(serialize_with = "super::super::serialize_option_asref_str_seq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners: Option<&'a [S]>,
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
#[serde(bound(serialize = ""))]
pub struct GetRepositoriesParams<'a, S: AsRef<str>> {
    /// Number of items from the results to skip. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// Max number of items to return. Defaults to 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// If specified, determines which items to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<GetRepositoriesFilter<'a, S>>,
    /// Specifies a field in the returned items to sort by. Defaults to "createdAt".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
}
