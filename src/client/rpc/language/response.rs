use serde::Deserialize;

#[cfg(feature = "events")]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LanguageServerTerminatedReason {
    Timedout,
    Oom,
    Unknown,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "params")]
pub enum LanguageEvent {
    #[serde(rename_all = "camelCase")]
    Exit {
        language_server_id: String,
        reason: LanguageServerTerminatedReason,
    },
    #[serde(rename_all = "camelCase")]
    Data {
        language_server_id: String,
        #[serde(skip_deserializing)]
        data: bytes::Bytes,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartLanguageServerResult {
    /// A unique identifier which you should use in subsequent API calls.
    pub language_server_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum StartLanguageServerError {
    ModelNotFound,
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
pub struct WriteToLanguageServerResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum WriteToLanguageServerError {
    LanguageServerNotFound,
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
pub struct LanguageUnsubscribeFromEventsResult {}

#[cfg(feature = "events")]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum LanguageUnsubscribeFromEventsError {
    NotSubscribed,
    TooManyRequests,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}
