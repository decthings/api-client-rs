#[cfg(feature = "events")]
use std::sync::Arc;

#[derive(Debug)]
pub enum DecthingsClientError {
    #[cfg(not(target_os = "espidf"))]
    /// The HTTP request to Decthings failed unexpectedly, for example due to a network error.
    /// If Decthings returned an error, DecthingsRpcError::Rpc should have been returned instead.
    Http(reqwest::Error),

    #[cfg(target_os = "espidf")]
    /// The HTTP request to Decthings failed unexpectedly, for example due to a network error.
    /// If Decthings returned an error, DecthingsRpcError::Rpc should have been returned instead.
    Http(esp_idf_svc::sys::EspError),

    #[cfg(feature = "events")]
    /// Failed to connect websocket to Decthings.
    WebSocketConnect(Arc<tokio_tungstenite::tungstenite::Error>),

    #[cfg(feature = "events")]
    /// Failed to write websocket data to Decthings.
    WebSocketWrite(Arc<tokio_tungstenite::tungstenite::Error>),

    #[cfg(feature = "events")]
    /// Failed to read websocket data from Decthings.
    WebSocketRead(Arc<tokio_tungstenite::tungstenite::Error>),

    /// JSON parse failed for the data received from Decthings.
    ParseResponseFailed(serde_json::Error),

    /// The data received by Decthings was invalid.
    InvalidMessage,
}

#[cfg(target_os = "espidf")]
impl From<esp_idf_svc::sys::EspError> for DecthingsClientError {
    fn from(value: esp_idf_svc::sys::EspError) -> Self {
        DecthingsClientError::Http(value)
    }
}

#[cfg(target_os = "espidf")]
impl From<esp_idf_svc::io::EspIOError> for DecthingsClientError {
    fn from(value: esp_idf_svc::io::EspIOError) -> Self {
        DecthingsClientError::Http(value.0)
    }
}

impl std::fmt::Display for DecthingsClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DecthingsClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Http(e) => Some(e),
            #[cfg(feature = "events")]
            Self::WebSocketConnect(e) => Some(e),
            #[cfg(feature = "events")]
            Self::WebSocketRead(e) => Some(e),
            #[cfg(feature = "events")]
            Self::WebSocketWrite(e) => Some(e),
            Self::ParseResponseFailed(e) => Some(e),
            Self::InvalidMessage => None,
        }
    }
}

#[cfg(feature = "events")]
impl From<super::websocket::WebSocketClientError> for DecthingsClientError {
    fn from(x: super::websocket::WebSocketClientError) -> Self {
        match x {
            super::websocket::WebSocketClientError::Connect(e) => {
                DecthingsClientError::WebSocketConnect(e)
            }
            super::websocket::WebSocketClientError::Write(e) => {
                DecthingsClientError::WebSocketWrite(e)
            }
            super::websocket::WebSocketClientError::Read(e) => {
                DecthingsClientError::WebSocketRead(e)
            }
            super::websocket::WebSocketClientError::InvalidMessage => {
                DecthingsClientError::InvalidMessage
            }
        }
    }
}

#[derive(Debug)]
pub enum DecthingsRpcError<E> {
    /// The request failed, for example network or JSON error.
    Request(DecthingsClientError),
    /// The request was successful, but an error was returned by Decthings.
    Rpc(E),
}

impl<E: std::fmt::Debug> std::fmt::Display for DecthingsRpcError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<E> From<serde_json::Error> for DecthingsRpcError<E> {
    fn from(x: serde_json::Error) -> Self {
        Self::Request(DecthingsClientError::ParseResponseFailed(x))
    }
}

impl<E> From<DecthingsClientError> for DecthingsRpcError<E> {
    fn from(x: DecthingsClientError) -> Self {
        Self::Request(x)
    }
}

impl<E: std::fmt::Debug> std::error::Error for DecthingsRpcError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Request(e) => Some(e),
            Self::Rpc(_) => None,
        }
    }
}
