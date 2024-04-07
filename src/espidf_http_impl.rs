use std::sync::Arc;

use crate::error::DecthingsClientError;
use embedded_svc::{http::client::Client, io::Write, utils::io::try_read_full};
use esp_idf_svc::{
    http::client::{Configuration, EspHttpConnection},
    io::EspIOError,
};
use esp_idf_sys::EspError;

#[derive(Clone, Default)]
pub(crate) struct HttpImpl {}

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
        let path = format!("{}/{}/{}", http_server_address, api, method);
        tokio::task::spawn_blocking(move || {
            let mut client = Client::wrap(EspHttpConnection::new(&Configuration {
                crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
                buffer_size_tx: Some(2048),
                ..Default::default()
            })?);

            let mut headers =
                Vec::with_capacity(extra_headers.len() + 2 + if api_key.is_some() { 1 } else { 0 });

            headers.push(("content-type", "application/octet-stream"));
            let content_length_str = body.len().to_string();
            headers.push(("content-length", &content_length_str));

            let bearer_auth = api_key.map(|api_key| format!("Bearer {api_key}"));
            if let Some(bearer_auth) = &bearer_auth {
                headers.push(("authorization", bearer_auth));
            }

            for (key, value) in &*extra_headers {
                headers.push((
                    key.as_str(),
                    std::str::from_utf8(value.as_bytes()).map_err(|_| {
                        EspError::from_infallible::<{ esp_idf_sys::ESP_ERR_INVALID_ARG }>()
                    })?,
                ));
            }

            let mut req = client.post(&path, &headers)?;

            req.write_all(&body)?;
            req.flush()?;

            let mut response = req.submit()?;

            let response_length: usize = response
                .header("content-length")
                .map(|length| length.parse().ok())
                .flatten()
                .ok_or(DecthingsClientError::Http(EspError::from_infallible::<
                    { esp_idf_sys::ESP_ERR_INVALID_RESPONSE },
                >()))?;

            let status = response.status();

            let mut response_body = vec![0; response_length];
            let bytes_read = try_read_full(response, &mut response_body).map_err(|x| x.0)?;

            if status >= 400 || bytes_read != response_length {
                return Err(
                    EspError::from_infallible::<{ esp_idf_sys::ESP_ERR_INVALID_RESPONSE }>().into(),
                );
            }

            Ok(response_body.into())
        })
        .await
        .unwrap()
    }
}
