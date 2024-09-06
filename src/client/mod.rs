mod error;

#[cfg(feature = "events")]
pub mod event;

#[cfg(target_os = "espidf")]
mod espidf_http_impl;
#[cfg(target_os = "espidf")]
use espidf_http_impl::*;

mod parameter;
mod protocol;

#[cfg(not(target_os = "espidf"))]
mod reqwest_http_impl;
#[cfg(not(target_os = "espidf"))]
use reqwest_http_impl::*;

pub mod rpc;

#[cfg(feature = "events")]
mod websocket;

use std::sync::Arc;
use tokio::sync::RwLock;

pub use ndarray;

pub use error::{DecthingsClientError, DecthingsRpcError};
pub use parameter::*;

struct StateModification {
    #[cfg(feature = "events")]
    add_events: Vec<String>,
    #[cfg(feature = "events")]
    remove_events: Vec<String>,
}

impl StateModification {
    fn empty() -> Self {
        Self {
            #[cfg(feature = "events")]
            add_events: vec![],
            #[cfg(feature = "events")]
            remove_events: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct DecthingsClientOptions {
    #[cfg(feature = "events")]
    /// Server address to use for WebSocket API. Defaults to "wss://api.decthings.com/v0/ws`
    pub ws_server_address: String,

    /// Server address to use for HTTP API. Defaults to `https://api.decthings.com/v0`
    pub http_server_address: String,
    /// Optional API key. Some methods require this to be set.
    pub api_key: Option<String>,
    /// Additional headers to add to each request.
    pub extra_headers: http::HeaderMap<http::HeaderValue>,
}

impl std::default::Default for DecthingsClientOptions {
    fn default() -> Self {
        Self {
            #[cfg(feature = "events")]
            ws_server_address: "wss://api.decthings.com/v0/v0".to_string(),

            http_server_address: "https://api.decthings.com/v0".to_string(),
            api_key: None,
            extra_headers: http::HeaderMap::new(),
        }
    }
}

/// The protocol to use for a RPC request.
#[derive(Debug, Clone)]
pub enum RpcProtocol {
    /// Force use of HTTP.
    Http,
    #[cfg(feature = "events")]
    /// Force use of WebSocket. If no WebSocket is connected, a new one will be created.
    Ws,
    #[cfg(feature = "events")]
    /// Use WebSocket if one is connected, otherwise do not send the request.
    WsIfAvailableOtherwiseNone,
}

#[derive(Clone)]
pub(crate) struct DecthingsClientRpc {
    #[cfg(feature = "events")]
    ws_server_address: String,

    http_server_address: String,
    api_key: Arc<RwLock<Option<Arc<str>>>>,
    extra_headers: Arc<http::HeaderMap<http::HeaderValue>>,

    #[cfg(feature = "events")]
    event_listeners: Arc<event::EventListeners>,

    #[cfg(feature = "events")]
    ws: Arc<RwLock<(u64, Option<(u64, Arc<websocket::DecthingsClientWebsocket>)>)>>,

    http: HttpImpl,
}

impl DecthingsClientRpc {
    fn new(options: DecthingsClientOptions) -> Self {
        Self {
            #[cfg(feature = "events")]
            ws_server_address: options.ws_server_address,

            http_server_address: options.http_server_address,
            api_key: Arc::new(RwLock::new(options.api_key.map(Arc::from))),
            extra_headers: Arc::new(options.extra_headers),

            #[cfg(feature = "events")]
            event_listeners: Arc::new(event::EventListeners::new()),

            #[cfg(feature = "events")]
            ws: Arc::new(RwLock::new((0, None))),

            http: HttpImpl::default(),
        }
    }

    async fn set_api_key(&self, api_key: String) {
        let mut locked = self.api_key.write().await;
        *locked = Some(Arc::from(api_key));
    }

    #[cfg(feature = "events")]
    async fn on_event(
        &self,
        cb: impl Fn(&event::DecthingsEvent) + Send + Sync + 'static,
    ) -> event::EventListenerDisposer {
        self.event_listeners.add(cb).await
    }

    #[cfg(feature = "events")]
    async fn maybe_get_socket(&self) -> Option<Arc<websocket::DecthingsClientWebsocket>> {
        let ws = self.ws.read().await;
        ws.1.as_ref().map(|inner_ws| Arc::clone(&inner_ws.1))
    }

    #[cfg(feature = "events")]
    async fn get_or_create_socket(&self) -> Arc<websocket::DecthingsClientWebsocket> {
        let ws = self.ws.read().await;
        if let Some(inner_ws) = ws.1.as_ref() {
            return Arc::clone(&inner_ws.1);
        }
        drop(ws);
        let mut ws_mut = self.ws.write().await;
        if let Some(inner_ws) = ws_mut.1.as_ref() {
            return Arc::clone(&inner_ws.1);
        }
        let ws_clone = Arc::clone(&self.ws);
        let ws_clone2 = Arc::clone(&self.ws);
        let id = ws_mut.0;
        ws_mut.0 += 1;

        let event_listeners_clone = Arc::clone(&self.event_listeners);
        let sock = Arc::new(websocket::DecthingsClientWebsocket::connect(
            &self.extra_headers,
            move || async move {
                let mut ws_clone_lock = ws_clone.write().await;
                if let Some(inner_ws_clone) = ws_clone_lock.1.as_mut() {
                    if inner_ws_clone.0 == id {
                        ws_clone_lock.1 = None;
                    }
                }
                event_listeners_clone
                    .call(&event::DecthingsEvent::SubscriptionsRemoved)
                    .await;
            },
            move || {
                let ws_clone3 = Arc::clone(&ws_clone2);
                async move {
                    let mut ws_clone_lock = ws_clone3.write().await;
                    if let Some(inner_ws_clone) = ws_clone_lock.1.as_mut() {
                        if inner_ws_clone.1.is_unused().await {
                            ws_clone_lock.1 = None;
                        }
                    }
                }
            },
            &self.ws_server_address,
            Arc::downgrade(&self.event_listeners),
        ));
        let _ = ws_mut.1.insert((id, Arc::clone(&sock)));
        sock
    }

    /// Call an RPC method on the server.
    /// You most likely want to use the helper classes (client.model, client.data, etc.) instead.
    ///
    /// Returns false if the request is not sent (and on_result is not called). This happens if
    /// mode is RpcProtocol::WsIfAvailableOtherwiseNone and no WebSocket is connected.
    async fn raw_method_call<
        P: serde::Serialize,
        F: FnOnce(
                Result<(bytes::Bytes, Vec<bytes::Bytes>), DecthingsClientError>,
            ) -> StateModification
            + Send
            + 'static,
        D: AsRef<[u8]>,
    >(
        &self,
        api: &str,
        method: &str,
        params: P,
        data: impl AsRef<[D]>,
        #[allow(unused)] mode: RpcProtocol,
        on_result: F,
    ) -> bool {
        #[cfg(feature = "events")]
        {
            let maybe_ws = match mode {
                RpcProtocol::Http => None,
                RpcProtocol::Ws => Some(self.get_or_create_socket().await),
                RpcProtocol::WsIfAvailableOtherwiseNone => {
                    if let Some(ws) = self.maybe_get_socket().await {
                        Some(ws)
                    } else {
                        return false;
                    }
                }
            };

            if let Some(ws) = maybe_ws {
                // Send over WebSocket
                let api_key = {
                    let locked = self.api_key.read().await;
                    locked.clone()
                };
                ws.call(
                    api,
                    method,
                    params,
                    api_key.as_deref(),
                    data,
                    Box::new(move |x| on_result(x.map_err(|e| e.into()))),
                )
                .await;
                return true;
            }
        }

        // Send over HTTP
        let res = async {
            let body = protocol::serialize_for_http(params, data.as_ref());
            drop(data);

            let api_key_locked = self.api_key.read().await;
            let api_key = api_key_locked.clone();
            drop(api_key_locked);

            let response_body = self
                .http
                .get(
                    &self.http_server_address,
                    api,
                    method,
                    body,
                    api_key,
                    self.extra_headers.clone(),
                )
                .await?;

            let deserialized = protocol::deserialize_for_http(response_body)
                .map_err(|_| DecthingsClientError::InvalidMessage)?;
            Ok(deserialized)
        }
        .await;

        on_result(res);
        true
    }
}

pub struct DecthingsClient {
    rpc: DecthingsClientRpc,
    pub dataset: rpc::dataset::DatasetRpc,
    pub debug: rpc::debug::DebugRpc,
    pub fs: rpc::fs::FsRpc,
    pub image: rpc::image::ImageRpc,
    #[cfg(feature = "events")]
    pub language: rpc::language::LanguageRpc,
    pub model: rpc::model::ModelRpc,
    pub persistent_launcher: rpc::persistent_launcher::PersistentLauncherRpc,
    pub spawned: rpc::spawned::SpawnedRpc,
    pub terminal: rpc::terminal::TerminalRpc,
}

impl Default for DecthingsClient {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl DecthingsClient {
    pub fn new(options: DecthingsClientOptions) -> Self {
        let rpc = DecthingsClientRpc::new(options);
        Self {
            dataset: rpc::dataset::DatasetRpc::new(rpc.clone()),
            debug: rpc::debug::DebugRpc::new(rpc.clone()),
            fs: rpc::fs::FsRpc::new(rpc.clone()),
            image: rpc::image::ImageRpc::new(rpc.clone()),
            #[cfg(feature = "events")]
            language: rpc::language::LanguageRpc::new(rpc.clone()),
            model: rpc::model::ModelRpc::new(rpc.clone()),
            persistent_launcher: rpc::persistent_launcher::PersistentLauncherRpc::new(rpc.clone()),
            spawned: rpc::spawned::SpawnedRpc::new(rpc.clone()),
            terminal: rpc::terminal::TerminalRpc::new(rpc.clone()),
            rpc,
        }
    }

    #[cfg(feature = "events")]
    pub async fn on_event(
        &self,
        cb: impl Fn(&event::DecthingsEvent) + Send + Sync + 'static,
    ) -> event::EventListenerDisposer {
        self.rpc.on_event(Box::new(cb)).await
    }

    pub async fn set_api_key(&self, api_key: String) {
        self.rpc.set_api_key(api_key).await;
    }
}
