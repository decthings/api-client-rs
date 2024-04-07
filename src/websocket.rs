use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Weak},
};

use futures::{SinkExt, StreamExt};
use tokio::sync::Mutex;

use crate::StateModification;

#[derive(Clone)]
pub(crate) enum WebSocketClientError {
    Connect(Arc<tokio_tungstenite::tungstenite::Error>),
    Write(Arc<tokio_tungstenite::tungstenite::Error>),
    Read(Arc<tokio_tungstenite::tungstenite::Error>),
    InvalidMessage,
}

struct DecthingsClientWebsocketState {
    request_id_counter: u32,
    events: HashSet<String>,
    requests: HashSet<u32>,
}

pub(crate) struct DecthingsClientWebsocket {
    state: Arc<Mutex<DecthingsClientWebsocketState>>,
    call_method_tx: tokio::sync::mpsc::Sender<(
        u32,
        Vec<u8>,
        Box<
            dyn FnOnce(
                    Result<(bytes::Bytes, Vec<bytes::Bytes>), WebSocketClientError>,
                ) -> StateModification
                + Send
                + 'static,
        >,
    )>,
}

impl DecthingsClientWebsocket {
    pub(crate) fn connect<
        Fut: futures::Future<Output = ()> + Send + 'static,
        Fut2: futures::Future<Output = ()> + Send + 'static,
    >(
        extra_headers: &http::HeaderMap<http::HeaderValue>,
        on_disconnected: impl FnOnce() -> Fut + Send + 'static,
        remove_if_unused: impl Fn() -> Fut2 + Send + Sync + 'static,
        ws_server_address: &str,
        weak_event_listeners: Weak<super::event::EventListeners>,
    ) -> Self {
        let state = Arc::new(Mutex::new(DecthingsClientWebsocketState {
            request_id_counter: 0,
            events: HashSet::new(),
            requests: HashSet::new(),
        }));
        let state2 = Arc::clone(&state);
        let (call_method_tx, mut call_method_rx) = tokio::sync::mpsc::channel::<(
            u32,
            Vec<u8>,
            Box<
                dyn FnOnce(
                        Result<(bytes::Bytes, Vec<bytes::Bytes>), WebSocketClientError>,
                    ) -> StateModification
                    + Send
                    + 'static,
            >,
        )>(10);
        let req = tokio_tungstenite::tungstenite::client::IntoClientRequest::into_client_request(
            ws_server_address,
        )
        .map(|mut req| {
            let headers = req.headers_mut();
            for (key, value) in extra_headers {
                headers.insert(key.to_owned(), value.to_owned());
            }
            req
        });
        tokio::spawn(async move {
            let connect_res = match req {
                Ok(req) => tokio_tungstenite::connect_async(req).await,
                Err(e) => Err(e),
            };
            let sock = match connect_res {
                Ok((ws_stream, _)) => ws_stream,
                Err(err) => {
                    on_disconnected().await;

                    let e = WebSocketClientError::Connect(Arc::new(err));

                    while let Some(next_method_call) = call_method_rx.recv().await {
                        (next_method_call.2)(Err(e.clone()));
                    }
                    return;
                }
            };

            let (mut write_half, mut read_half) = sock.split();

            let waiting_for_response = Mutex::new(HashMap::new());

            let write_fut = async {
                while let Some(next) = call_method_rx.recv().await {
                    let mut locked_waiting_for_response = waiting_for_response.lock().await;
                    locked_waiting_for_response.insert(next.0, next.2);
                    drop(locked_waiting_for_response);
                    write_half
                        .send(tokio_tungstenite::tungstenite::Message::Binary(next.1))
                        .await
                        .map_err(|x| WebSocketClientError::Write(Arc::new(x)))?;
                }

                // This will tell the server to close the connection. When that happens, read will
                // fail. If send_tx was dropped while waiting for RPC calls, these will return an
                // error. This shouldn't be the case since each caller holds an Arc to
                // DecthingsClientWebsocket.
                write_half
                    .send(tokio_tungstenite::tungstenite::Message::Close(None))
                    .await
                    .map_err(|x| WebSocketClientError::Write(Arc::new(x)))?;
                Ok::<_, WebSocketClientError>(())
            };

            let read_fut = async {
                loop {
                    let next = read_half
                        .next()
                        .await
                        .unwrap_or(Err(tokio_tungstenite::tungstenite::Error::ConnectionClosed))
                        .map_err(|x| WebSocketClientError::Read(Arc::new(x)));
                    let next = match next {
                        Err(e) => return Err::<(), _>(e),
                        Ok(val) => val,
                    };
                    let binary = match next {
                        tokio_tungstenite::tungstenite::Message::Binary(bin) => bin,
                        tokio_tungstenite::tungstenite::Message::Text(text) => text.into_bytes(),
                        _ => continue,
                    };
                    let (rpc_response_or_event, first_segment, additional_segments) =
                        super::protocol::deserialize_for_websocket(bytes::Bytes::from(binary))
                            .map_err(|_| WebSocketClientError::InvalidMessage)?;
                    match rpc_response_or_event {
                        super::protocol::RpcResponseOrEvent::RpcResponse(id) => {
                            // RPC response message
                            let mut locked_waiting_for_response = waiting_for_response.lock().await;
                            let maybe_waiting = locked_waiting_for_response.remove(&id);
                            drop(locked_waiting_for_response);

                            if let Some(waiting) = maybe_waiting {
                                let state_modification =
                                    (waiting)(Ok((first_segment, additional_segments)));

                                let mut state_locked = state2.lock().await;
                                for remove_event in state_modification.remove_events {
                                    state_locked.events.remove(&remove_event);
                                }
                                for add_event in state_modification.add_events {
                                    state_locked.events.insert(add_event);
                                }
                                state_locked.requests.remove(&id);
                                if state_locked.requests.is_empty()
                                    && state_locked.events.is_empty()
                                {
                                    remove_if_unused().await;
                                }
                            }
                        }
                        super::protocol::RpcResponseOrEvent::Event(api) => {
                            // Event message
                            if let Some(event_listeners) = weak_event_listeners.upgrade() {
                                let (parsed, state_modification) =
                                    super::event::DecthingsEvent::deserialize(
                                        &api,
                                        &first_segment,
                                        additional_segments,
                                    )
                                    .map_err(|_| WebSocketClientError::InvalidMessage)?;

                                event_listeners.call(&parsed).await;

                                if !state_modification.add_events.is_empty()
                                    || !state_modification.remove_events.is_empty()
                                {
                                    let mut state_locked = state2.lock().await;
                                    for remove_event in state_modification.remove_events {
                                        state_locked.events.remove(&remove_event);
                                    }
                                    for add_event in state_modification.add_events {
                                        state_locked.events.insert(add_event);
                                    }
                                    if state_locked.requests.is_empty()
                                        && state_locked.events.is_empty()
                                    {
                                        drop(state_locked);
                                        remove_if_unused().await;
                                    }
                                }
                            }
                        }
                    }
                }
            };

            let e = futures::try_join!(write_fut, read_fut).unwrap_err();

            on_disconnected().await;

            while let Some(next_method_call) = call_method_rx.recv().await {
                (next_method_call.2)(Err(e.clone()));
            }
        });
        Self {
            state,
            call_method_tx,
        }
    }

    pub(crate) async fn call<P: serde::Serialize, D: AsRef<[u8]>>(
        &self,
        api: &str,
        method: &str,
        params: P,
        api_key: Option<&str>,
        data: impl AsRef<[D]>,
        on_result: Box<
            dyn FnOnce(
                    Result<(bytes::Bytes, Vec<bytes::Bytes>), WebSocketClientError>,
                ) -> StateModification
                + Send
                + 'static,
        >,
    ) {
        let mut state_locked = self.state.lock().await;

        let id = state_locked.request_id_counter;
        state_locked.request_id_counter += 1;
        state_locked.requests.insert(id);
        drop(state_locked);

        let serialized = super::protocol::serialize_for_websocket(
            id,
            super::protocol::RequestMessage::new(api, method, params, api_key),
            data.as_ref(),
        );
        drop(data);

        self.call_method_tx
            .send((id, serialized, on_result))
            .await
            .ok();
    }

    pub(crate) async fn is_unused(&self) -> bool {
        let state_locked = self.state.lock().await;
        state_locked.events.is_empty() && state_locked.requests.is_empty()
    }
}
