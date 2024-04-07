mod request;
mod response;

pub use request::*;
pub use response::*;
use serde::Serialize;

use crate::StateModification;

pub struct TerminalRpc {
    rpc: crate::DecthingsClientRpc,
}

impl TerminalRpc {
    pub(crate) fn new(rpc: crate::DecthingsClientRpc) -> Self {
        Self { rpc }
    }

    pub async fn launch_terminal_session(
        &self,
        params: LaunchTerminalSessionParams<'_>,
    ) -> Result<LaunchTerminalSessionResult, crate::DecthingsRpcError<LaunchTerminalSessionError>>
    {
        #[cfg(feature = "events")]
        let subscribe_to_events = params.subscribe_to_events != Some(false);

        #[cfg(feature = "events")]
        let protocol = if subscribe_to_events {
            crate::RpcProtocol::Ws
        } else {
            crate::RpcProtocol::Http
        };

        #[cfg(not(feature = "events"))]
        let protocol = crate::RpcProtocol::Http;

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
                                crate::DecthingsRpcError<LaunchTerminalSessionError>,
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
                                    tx.send(Err(crate::DecthingsRpcError::Rpc(val))).ok();
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
        crate::DecthingsRpcError<TerminateTerminalSessionError>,
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
                crate::RpcProtocol::Http,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<
                                    TerminateTerminalSessionResult,
                                    TerminateTerminalSessionError,
                                >,
                                crate::DecthingsRpcError<TerminateTerminalSessionError>,
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
                                    tx.send(Err(crate::DecthingsRpcError::Rpc(val))).ok();
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
    ) -> Result<GetTerminalSessionsResult, crate::DecthingsRpcError<GetTerminalSessionsError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Terminal",
                "getTerminalSessions",
                params,
                &[],
                crate::RpcProtocol::Http,
                |x| {
                    tx.send(x).ok();
                    StateModification::empty()
                },
            )
            .await;
        rx.await
            .unwrap()
            .map_err(crate::DecthingsRpcError::Request)
            .and_then(|x| {
                let res: super::Response<GetTerminalSessionsResult, GetTerminalSessionsError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn write_to_terminal_session(
        &self,
        params: WriteToTerminalSessionParams<'_, impl AsRef<[u8]>>,
    ) -> Result<WriteToTerminalSessionResult, crate::DecthingsRpcError<WriteToTerminalSessionError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call(
                "Terminal",
                "writeToTerminalSession",
                &params,
                &[&params.data],
                crate::RpcProtocol::Http,
                |x| {
                    tx.send(x).ok();
                    StateModification::empty()
                },
            )
            .await;
        rx.await
            .unwrap()
            .map_err(crate::DecthingsRpcError::Request)
            .and_then(|x| {
                let res: super::Response<
                    WriteToTerminalSessionResult,
                    WriteToTerminalSessionError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn resize_terminal_session(
        &self,
        params: ResizeTerminalSessionParams<'_>,
    ) -> Result<ResizeTerminalSessionResult, crate::DecthingsRpcError<ResizeTerminalSessionError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Terminal",
                "resizeTerminalSession",
                params,
                &[],
                crate::RpcProtocol::Http,
                |x| {
                    tx.send(x).ok();
                    StateModification::empty()
                },
            )
            .await;
        rx.await
            .unwrap()
            .map_err(crate::DecthingsRpcError::Request)
            .and_then(|x| {
                let res: super::Response<ResizeTerminalSessionResult, ResizeTerminalSessionError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    #[cfg(feature = "events")]
    pub async fn subscribe_to_events(
        &self,
        params: TerminalSubscribeToEventsParams<'_>,
    ) -> Result<
        TerminalSubscribeToEventsResult,
        crate::DecthingsRpcError<TerminalSubscribeToEventsError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let spawned_command_id_owned = params.terminal_session_id.to_owned();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Terminal",
                "subscribeToEvents",
                params,
                &[],
                crate::RpcProtocol::Ws,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<
                                    TerminalSubscribeToEventsResult,
                                    TerminalSubscribeToEventsError,
                                >,
                                crate::DecthingsRpcError<TerminalSubscribeToEventsError>,
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
                                    tx.send(Err(crate::DecthingsRpcError::Rpc(val))).ok();
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
        crate::DecthingsRpcError<TerminalUnsubscribeFromEventsError>,
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
                crate::RpcProtocol::WsIfAvailableOtherwiseNone,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<
                                    TerminalUnsubscribeFromEventsResult,
                                    TerminalUnsubscribeFromEventsError,
                                >,
                                crate::DecthingsRpcError<TerminalUnsubscribeFromEventsError>,
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
                                    tx.send(Err(crate::DecthingsRpcError::Rpc(val))).ok();
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
            return Err(crate::DecthingsRpcError::Rpc(
                TerminalUnsubscribeFromEventsError::NotSubscribed,
            ));
        }
        rx.await.unwrap()
    }
}
