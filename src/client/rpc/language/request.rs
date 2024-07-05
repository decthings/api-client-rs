use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Language {
    Go,
    Python,
    Rust,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartLanguageServerParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// The language to use.
    pub language: Language,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteToLanguageServerParams<'a, D: AsRef<[u8]>> {
    /// The language server's id.
    pub language_server_id: &'a str,
    /// Data to write.
    #[serde(skip_serializing)]
    pub data: D,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageUnsubscribeFromEventsParams<'a> {
    /// The language server's id.
    pub language_server_id: &'a str,
}
