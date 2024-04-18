mod request;
mod response;

pub use request::*;
pub use response::*;
use serde::Serialize;

use crate::client::StateModification;

pub struct TerminalRpc {
    rpc: crate::client::DecthingsClientRpc,
}

impl TerminalRpc {
    pub(crate) fn new(rpc: crate::client::DecthingsClientRpc) -> Self {
        Self { rpc }
    }

    pub async fn launch_terminal_session(
        &self,
        params: LaunchTerminalSessionParams<'_>,
    ) -> Result<
        LaunchTerminalSessionResult,
        crate::client::DecthingsRpcError<LaunchTerminalSessionError>,
    > {
        #[cfg(feature = "events")]
        let subscribe_to_events = params.subscribe_to_events != Some(false);

        #[cfg(feature = "events")]
        let protocol = if subscribe_to_events {
            crate::client::RpcProtocol::Ws
        } else {
            crate::client::RpcProtocol::Http
        };

        #[cfg(not(feature = "events"))]
        let protocol = crate::client::RpcProtocol::Http;

        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Terminal",
                "launchTerminalSession",
                params,
                &[],
                protocol,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<
                                    LaunchTerminalSessionResult,
                                    LaunchTerminalSessionError,
                                >,
                                crate::client::DecthingsRpcError<LaunchTerminalSessionError>,
                            > = serde_json::from_slice(&val.0).map_err(Into::into);
                            match res {
                                Ok(super::Response::Result(val)) => {
                                    #[cfg(feature = "events")]
                                    let terminal_session_id = val.terminal_session_id.clone();

                                    tx.send(Ok(val)).ok();

                                    #[cfg(feature = "events")]
                                    if subscribe_to_events {
                                        return StateModification {
                                            add_events: vec![terminal_session_id],
                                            remove_events: vec![],
                                        };
                                    }
                                }
                                Ok(super::Response::Error(val)) => {
                                    tx.send(Err(crate::client::DecthingsRpcError::Rpc(val)))
                                        .ok();
                                }
                                Err(e) => {
                                    tx.send(Err(e)).ok();
                                }
                            }
                        }
                        Err(err) => {
                            tx.send(Err(err.into())).ok();
                        }
                    }
                    StateModification::empty()
                },
            )
            .await;
        rx.await.unwrap()
    }

    pub async fn terminate_terminal_session(
        &self,
        params: TerminateTerminalSessionParams<'_>,
    ) -> Result<
        TerminateTerminalSessionResult,
        crate::client::DecthingsRpcError<TerminateTerminalSessionError>,
    > {
        #[cfg(feature = "events")]
        let terminal_session_id_owned = params.terminal_session_id.to_owned();

        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Terminal",
                "terminateTerminalSession",
                params,
                &[],
                crate::client::RpcProtocol::Http,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<
                                    TerminateTerminalSessionResult,
                                    TerminateTerminalSessionError,
                                >,
                                crate::client::DecthingsRpcError<TerminateTerminalSessionError>,
                            > = serde_json::from_slice(&val.0).map_err(Into::into);
                            match res {
                                Ok(super::Response::Result(val)) => {
                                    tx.send(Ok(val)).ok();

                                    #[cfg(feature = "events")]
                                    return StateModification {
                                        add_events: vec![],
                                        remove_events: vec![terminal_session_id_owned],
                                    };
                                }
                                Ok(super::Response::Error(val)) => {
                                    tx.send(Err(crate::client::DecthingsRpcError::Rpc(val)))
                                        .ok();
                                }
                                Err(e) => {
                                    tx.send(Err(e)).ok();
                                }
                            }
                        }
                        Err(err) => {
                            tx.send(Err(err.into())).ok();
                        }
                    }
                    StateModification::empty()
                },
            )
            .await;
        rx.await.unwrap()
    }

    pub async fn get_terminal_sessions(
        &self,
        params: GetTerminalSessionsParams<'_, impl AsRef<str> + Serialize>,
    ) -> Result<GetTerminalSessionsResult, crate::client::DecthingsRpcError<GetTerminalSessionsError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Terminal",
                "getTerminalSessions",
                params,
                &[],
                crate::client::RpcProtocol::Http,
                |x| {
                    tx.send(x).ok();
                    StateModification::empty()
                },
            )
            .await;
        rx.await
            .unwrap()
            .map_err(crate::client::DecthingsRpcError::Request)
            .and_then(|x| {
                let res: super::Response<GetTerminalSessionsResult, GetTerminalSessionsError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn write_to_terminal_session(
        &self,
        params: WriteToTerminalSessionParams<'_, impl AsRef<[u8]>>,
    ) -> Result<
        WriteToTerminalSessionResult,
        crate::client::DecthingsRpcError<WriteToTerminalSessionError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call(
                "Terminal",
                "writeToTerminalSession",
                &params,
                &[&params.data],
                crate::client::RpcProtocol::Http,
                |x| {
                    tx.send(x).ok();
                    StateModification::empty()
                },
            )
            .await;
        rx.await
            .unwrap()
            .map_err(crate::client::DecthingsRpcError::Request)
            .and_then(|x| {
                let res: super::Response<
                    WriteToTerminalSessionResult,
                    WriteToTerminalSessionError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn resize_terminal_session(
        &self,
        params: ResizeTerminalSessionParams<'_>,
    ) -> Result<
        ResizeTerminalSessionResult,
        crate::client::DecthingsRpcError<ResizeTerminalSessionError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Terminal",
                "resizeTerminalSession",
                params,
                &[],
                crate::client::RpcProtocol::Http,
                |x| {
                    tx.send(x).ok();
                    StateModification::empty()
                },
            )
            .await;
        rx.await
            .unwrap()
            .map_err(crate::client::DecthingsRpcError::Request)
            .and_then(|x| {
                let res: super::Response<ResizeTerminalSessionResult, ResizeTerminalSessionError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    #[cfg(feature = "events")]
    pub async fn subscribe_to_events(
        &self,
        params: TerminalSubscribeToEventsParams<'_>,
    ) -> Result<
        TerminalSubscribeToEventsResult,
        crate::client::DecthingsRpcError<TerminalSubscribeToEventsError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let spawned_command_id_owned = params.terminal_session_id.to_owned();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Terminal",
                "subscribeToEvents",
                params,
                &[],
                crate::client::RpcProtocol::Ws,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<
                                    TerminalSubscribeToEventsResult,
                                    TerminalSubscribeToEventsError,
                                >,
                                crate::client::DecthingsRpcError<TerminalSubscribeToEventsError>,
                            > = serde_json::from_slice(&val.0).map_err(Into::into);
                            match res {
                                Ok(super::Response::Result(val)) => {
                                    tx.send(Ok(val)).ok();
                                    return StateModification {
                                        add_events: vec![spawned_command_id_owned],
                                        remove_events: vec![],
                                    };
                                }
                                Ok(super::Response::Error(val)) => {
                                    tx.send(Err(crate::client::DecthingsRpcError::Rpc(val)))
                                        .ok();
                                }
                                Err(e) => {
                                    tx.send(Err(e)).ok();
                                }
                            }
                        }
                        Err(err) => {
                            tx.send(Err(err.into())).ok();
                        }
                    }
                    StateModification::empty()
                },
            )
            .await;
        rx.await.unwrap()
    }

    #[cfg(feature = "events")]
    pub async fn unsubscribe_from_events(
        &self,
        params: TerminalUnsubscribeFromEventsParams<'_>,
    ) -> Result<
        TerminalUnsubscribeFromEventsResult,
        crate::client::DecthingsRpcError<TerminalUnsubscribeFromEventsError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let spawned_command_id_owned = params.terminal_session_id.to_owned();
        let did_call = self
            .rpc
            .raw_method_call::<_, _, &[u8]>(
                "Terminal",
                "unsubscribeFromEvents",
                params,
                &[],
                crate::client::RpcProtocol::WsIfAvailableOtherwiseNone,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<
                                    TerminalUnsubscribeFromEventsResult,
                                    TerminalUnsubscribeFromEventsError,
                                >,
                                crate::client::DecthingsRpcError<
                                    TerminalUnsubscribeFromEventsError,
                                >,
                            > = serde_json::from_slice(&val.0).map_err(Into::into);
                            match res {
                                Ok(super::Response::Result(val)) => {
                                    tx.send(Ok(val)).ok();
                                    return StateModification {
                                        add_events: vec![],
                                        remove_events: vec![spawned_command_id_owned],
                                    };
                                }
                                Ok(super::Response::Error(val)) => {
                                    tx.send(Err(crate::client::DecthingsRpcError::Rpc(val)))
                                        .ok();
                                }
                                Err(e) => {
                                    tx.send(Err(e)).ok();
                                }
                            }
                        }
                        Err(err) => {
                            tx.send(Err(err.into())).ok();
                        }
                    }
                    StateModification::empty()
                },
            )
            .await;
        if !did_call {
            return Err(crate::client::DecthingsRpcError::Rpc(
                TerminalUnsubscribeFromEventsError::NotSubscribed,
            ));
        }
        rx.await.unwrap()
    }
}
