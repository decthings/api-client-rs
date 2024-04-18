mod request;
mod response;

pub use request::*;
pub use response::*;
use serde::Serialize;

use crate::client::StateModification;

pub struct SpawnedRpc {
    rpc: crate::client::DecthingsClientRpc,
}

impl SpawnedRpc {
    pub(crate) fn new(rpc: crate::client::DecthingsClientRpc) -> Self {
        Self { rpc }
    }

    pub async fn spawn_command(
        &self,
        params: SpawnCommandParams<'_, impl AsRef<str>>,
    ) -> Result<SpawnCommandResult, crate::client::DecthingsRpcError<SpawnCommandError>> {
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
                "Spawned",
                "spawnCommand",
                params,
                &[],
                protocol,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<SpawnCommandResult, SpawnCommandError>,
                                crate::client::DecthingsRpcError<SpawnCommandError>,
                            > = serde_json::from_slice(&val.0).map_err(Into::into);
                            match res {
                                Ok(super::Response::Result(val)) => {
                                    #[cfg(feature = "events")]
                                    let spawned_command_id = val.spawned_command_id.clone();

                                    tx.send(Ok(val)).ok();

                                    #[cfg(feature = "events")]
                                    if subscribe_to_events {
                                        return StateModification {
                                            add_events: vec![spawned_command_id],
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

    pub async fn spawn_command_for_model(
        &self,
        params: SpawnCommandForModelParams<'_, impl AsRef<str>>,
    ) -> Result<
        SpawnCommandForModelResult,
        crate::client::DecthingsRpcError<SpawnCommandForModelError>,
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
                "Spawned",
                "spawnCommandForModel",
                params,
                &[],
                protocol,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<
                                    SpawnCommandForModelResult,
                                    SpawnCommandForModelError,
                                >,
                                crate::client::DecthingsRpcError<SpawnCommandForModelError>,
                            > = serde_json::from_slice(&val.0).map_err(Into::into);
                            match res {
                                Ok(super::Response::Result(val)) => {
                                    #[cfg(feature = "events")]
                                    let spawned_command_id = val.spawned_command_id.clone();

                                    tx.send(Ok(val)).ok();

                                    #[cfg(feature = "events")]
                                    if subscribe_to_events {
                                        return StateModification {
                                            add_events: vec![spawned_command_id],
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

    pub async fn terminate_spawned_command(
        &self,
        params: TerminateSpawnedCommandParams<'_>,
    ) -> Result<
        TerminateSpawnedCommandResult,
        crate::client::DecthingsRpcError<TerminateSpawnedCommandError>,
    > {
        #[cfg(feature = "events")]
        let spawned_command_id_owned = params.spawned_command_id.to_owned();

        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Spawned",
                "terminateSpawnedCommand",
                params,
                &[],
                crate::client::RpcProtocol::Http,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<
                                    TerminateSpawnedCommandResult,
                                    TerminateSpawnedCommandError,
                                >,
                                crate::client::DecthingsRpcError<TerminateSpawnedCommandError>,
                            > = serde_json::from_slice(&val.0).map_err(Into::into);
                            match res {
                                Ok(super::Response::Result(val)) => {
                                    tx.send(Ok(val)).ok();

                                    #[cfg(feature = "events")]
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
        rx.await.unwrap()
    }

    pub async fn get_spawned_commands(
        &self,
        params: GetSpawnedCommandsParams<'_, impl AsRef<str> + Serialize>,
    ) -> Result<GetSpawnedCommandsResult, crate::client::DecthingsRpcError<GetSpawnedCommandsError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Spawned",
                "getSpawnedCommands",
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
                let res: super::Response<GetSpawnedCommandsResult, GetSpawnedCommandsError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn write_to_spawned_command(
        &self,
        params: WriteToSpawnedCommandParams<'_, impl AsRef<[u8]>>,
    ) -> Result<
        WriteToSpawnedCommandResult,
        crate::client::DecthingsRpcError<WriteToSpawnedCommandError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call(
                "Spawned",
                "writeToSpawnedCommand",
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
                let res: super::Response<WriteToSpawnedCommandResult, WriteToSpawnedCommandError> =
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
        params: SpawnedSubscribeToEventsParams<'_>,
    ) -> Result<
        SpawnedSubscribeToEventsResult,
        crate::client::DecthingsRpcError<SpawnedSubscribeToEventsError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let spawned_command_id_owned = params.spawned_command_id.to_owned();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Spawned",
                "subscribeToEvents",
                params,
                &[],
                crate::client::RpcProtocol::Ws,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<
                                    SpawnedSubscribeToEventsResult,
                                    SpawnedSubscribeToEventsError,
                                >,
                                crate::client::DecthingsRpcError<SpawnedSubscribeToEventsError>,
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
        params: SpawnedUnsubscribeFromEventsParams<'_>,
    ) -> Result<
        SpawnedUnsubscribeFromEventsResult,
        crate::client::DecthingsRpcError<SpawnedUnsubscribeFromEventsError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let spawned_command_id_owned = params.spawned_command_id.to_owned();
        let did_call = self
            .rpc
            .raw_method_call::<_, _, &[u8]>(
                "Spawned",
                "unsubscribeFromEvents",
                params,
                &[],
                crate::client::RpcProtocol::WsIfAvailableOtherwiseNone,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<
                                    SpawnedUnsubscribeFromEventsResult,
                                    SpawnedUnsubscribeFromEventsError,
                                >,
                                crate::client::DecthingsRpcError<SpawnedUnsubscribeFromEventsError>,
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
                SpawnedUnsubscribeFromEventsError::NotSubscribed,
            ));
        }
        rx.await.unwrap()
    }
}
