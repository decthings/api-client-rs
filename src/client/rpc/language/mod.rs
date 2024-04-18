mod request;
mod response;

pub use request::*;
pub use response::*;

use crate::client::StateModification;

pub struct LanguageRpc {
    rpc: crate::client::DecthingsClientRpc,
}

impl LanguageRpc {
    pub(crate) fn new(rpc: crate::client::DecthingsClientRpc) -> Self {
        Self { rpc }
    }

    pub async fn start_language_server(
        &self,
        params: StartLanguageServerParams<'_>,
    ) -> Result<StartLanguageServerResult, crate::client::DecthingsRpcError<StartLanguageServerError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Language",
                "startLanguageServer",
                params,
                &[],
                crate::client::RpcProtocol::Ws,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<
                                    StartLanguageServerResult,
                                    StartLanguageServerError,
                                >,
                                crate::client::DecthingsRpcError<StartLanguageServerError>,
                            > = serde_json::from_slice(&val.0).map_err(Into::into);
                            match res {
                                Ok(super::Response::Result(val)) => {
                                    let language_server_id = val.language_server_id.clone();
                                    tx.send(Ok(val)).ok();
                                    return StateModification {
                                        add_events: vec![language_server_id],
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

    pub async fn write_to_language_server(
        &self,
        params: WriteToLanguageServerParams<'_>,
    ) -> Result<
        WriteToLanguageServerResult,
        crate::client::DecthingsRpcError<WriteToLanguageServerError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let did_call = self
            .rpc
            .raw_method_call(
                "Language",
                "writeToLanguageServer",
                &params,
                [params.data],
                crate::client::RpcProtocol::WsIfAvailableOtherwiseNone,
                |x| {
                    tx.send(x).ok();
                    StateModification::empty()
                },
            )
            .await;
        if !did_call {
            return Err(crate::client::DecthingsRpcError::Rpc(
                WriteToLanguageServerError::LanguageServerNotFound,
            ));
        }
        rx.await
            .unwrap()
            .map_err(crate::client::DecthingsRpcError::Request)
            .and_then(|x| {
                let res: super::Response<WriteToLanguageServerResult, WriteToLanguageServerError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn unsubscribe_from_events(
        &self,
        params: LanguageUnsubscribeFromEventsParams<'_>,
    ) -> Result<
        LanguageUnsubscribeFromEventsResult,
        crate::client::DecthingsRpcError<LanguageUnsubscribeFromEventsError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let language_server_id_owned = params.language_server_id.to_owned();
        let did_call = self
            .rpc
            .raw_method_call::<_, _, &[u8]>(
                "Language",
                "unsubscribeFromEvents",
                params,
                &[],
                crate::client::RpcProtocol::WsIfAvailableOtherwiseNone,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<
                                    LanguageUnsubscribeFromEventsResult,
                                    LanguageUnsubscribeFromEventsError,
                                >,
                                crate::client::DecthingsRpcError<
                                    LanguageUnsubscribeFromEventsError,
                                >,
                            > = serde_json::from_slice(&val.0).map_err(Into::into);
                            match res {
                                Ok(super::Response::Result(val)) => {
                                    tx.send(Ok(val)).ok();
                                    return StateModification {
                                        add_events: vec![],
                                        remove_events: vec![language_server_id_owned],
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
                LanguageUnsubscribeFromEventsError::NotSubscribed,
            ));
        }
        rx.await.unwrap()
    }
}
