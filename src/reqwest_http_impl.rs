use std::sync::Arc;

use crate::DecthingsClientError;

#[derive(Clone, Default)]
pub(crate) struct HttpImpl {
    client: reqwest::Client,
}

impl HttpImpl {
    pub async fn get(
        &self,
        http_server_address: &str,
        api: &str,
        method: &str,
        body: Vec<u8>,
        api_key: Option<Arc<str>>,
        extra_headers: Arc<http::HeaderMap<http::HeaderValue>>,
    ) -> Result<bytes::Bytes, DecthingsClientError> {
        let mut builder = self
            .client
            .post(format!("{}/{}/{}", http_server_address, api, method))
            .header(reqwest::header::CONTENT_TYPE, "application/octet-stream")
            .body(body);
        if let Some(api_key) = api_key {
            builder = builder.bearer_auth(api_key);
        }

        for (key, value) in &*extra_headers {
            builder = builder.header(key, value);
        }
        let result = builder.send().await;
        let response = result
            .map_err(DecthingsClientError::Http)?
            .error_for_status()
            .map_err(DecthingsClientError::Http)?;
        response.bytes().await.map_err(DecthingsClientError::Http)
    }
}
