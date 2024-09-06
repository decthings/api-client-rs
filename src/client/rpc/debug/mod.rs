mod request;
mod response;

use crate::{client::StateModification, tensor::OwnedDecthingsTensor};

pub use request::*;
pub use response::*;

pub struct DebugRpc {
    rpc: crate::client::DecthingsClientRpc,
}

impl DebugRpc {
    pub(crate) fn new(rpc: crate::client::DecthingsClientRpc) -> Self {
        Self { rpc }
    }

    pub async fn launch_debug_session(
        &self,
        params: LaunchDebugSessionParams<'_>,
    ) -> Result<LaunchDebugSessionResult, crate::client::DecthingsRpcError<LaunchDebugSessionError>>
    {
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
                "Debug",
                "launchDebugSession",
                params,
                &[],
                protocol,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<
                                    response::LaunchDebugSessionResult,
                                    response::LaunchDebugSessionError,
                                >,
                                crate::client::DecthingsRpcError<LaunchDebugSessionError>,
                            > = serde_json::from_slice(&val.0).map_err(Into::into);
                            match res {
                                Ok(super::Response::Result(val)) => {
                                    #[cfg(feature = "events")]
                                    let debug_session_id = val.debug_session_id.clone();

                                    tx.send(Ok(val)).ok();

                                    #[cfg(feature = "events")]
                                    if subscribe_to_events {
                                        return StateModification {
                                            add_events: vec![debug_session_id],
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

    pub async fn get_debug_sessions(
        &self,
        params: GetDebugSessionsParams<'_, impl AsRef<str>>,
    ) -> Result<GetDebugSessionsResult, crate::client::DecthingsRpcError<GetDebugSessionsError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Debug",
                "getDebugSessions",
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
                let res: super::Response<
                    response::GetDebugSessionsResult,
                    response::GetDebugSessionsError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn terminate_debug_session(
        &self,
        params: TerminateDebugSessionParams<'_>,
    ) -> Result<
        TerminateDebugSessionResult,
        crate::client::DecthingsRpcError<TerminateDebugSessionError>,
    > {
        #[cfg(feature = "events")]
        let debug_session_id_owned = params.debug_session_id.to_owned();

        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Debug",
                "terminateDebugSession",
                params,
                &[],
                crate::client::RpcProtocol::Http,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<
                                    response::TerminateDebugSessionResult,
                                    response::TerminateDebugSessionError,
                                >,
                                crate::client::DecthingsRpcError<TerminateDebugSessionError>,
                            > = serde_json::from_slice(&val.0).map_err(Into::into);
                            match res {
                                Ok(super::Response::Result(val)) => {
                                    tx.send(Ok(val)).ok();

                                    #[cfg(feature = "events")]
                                    return StateModification {
                                        add_events: vec![],
                                        remove_events: vec![debug_session_id_owned],
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

    pub async fn call_create_model_state<D>(
        &self,
        params: CallCreateModelStateParams<'_>,
    ) -> Result<
        CallCreateModelStateResult,
        crate::client::DecthingsRpcError<CallCreateModelStateError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let serialized = crate::client::serialize_parameter_provider_list(params.params.iter());
        self.rpc
            .raw_method_call(
                "Debug",
                "callCreateModelState",
                params,
                serialized,
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
                    response::CallCreateModelStateResult,
                    response::CallCreateModelStateError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn call_instantiate_model(
        &self,
        params: CallInstantiateModelParams<'_, impl AsRef<[u8]>>,
    ) -> Result<
        CallInstantiateModelResult,
        crate::client::DecthingsRpcError<CallInstantiateModelError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let serialized = match &params.state_data {
            StateDataProvider::Data { data } => *data,
            _ => &[],
        };
        self.rpc
            .raw_method_call(
                "Debug",
                "callInstantiateModel",
                params,
                serialized.iter().map(|x| &x.data).collect::<Vec<_>>(),
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
                    response::CallInstantiateModelResult,
                    response::CallInstantiateModelError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn call_train<D>(
        &self,
        params: CallTrainParams<'_>,
    ) -> Result<CallTrainResult, crate::client::DecthingsRpcError<CallTrainError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let serialized = crate::client::serialize_parameter_provider_list(params.params.iter());
        self.rpc
            .raw_method_call(
                "Debug",
                "callTrain",
                params,
                serialized,
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
                let res: super::Response<response::CallTrainResult, response::CallTrainError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn get_training_status(
        &self,
        params: DebugGetTrainingStatusParams<'_>,
    ) -> Result<
        DebugGetTrainingStatusResult,
        crate::client::DecthingsRpcError<DebugGetTrainingStatusError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Debug",
                "getTrainingStatus",
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
                let res: super::Response<
                    response::DebugGetTrainingStatusResult,
                    response::DebugGetTrainingStatusError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn get_training_metrics(
        &self,
        params: DebugGetTrainingMetricsParams<'_>,
    ) -> Result<
        DebugGetTrainingMetricsResult,
        crate::client::DecthingsRpcError<DebugGetTrainingMetricsError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Debug",
                "getTrainingMetrics",
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
                let res: super::Response<
                    response::DebugGetTrainingMetricsResult,
                    response::DebugGetTrainingMetricsError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(mut val) => {
                        if val.metrics.iter().map(|x| x.entries.len()).sum::<usize>() != x.1.len() {
                            return Err(crate::client::DecthingsClientError::InvalidMessage.into());
                        }
                        for (entry, data) in
                            val.metrics.iter_mut().flat_map(|x| &mut x.entries).zip(x.1)
                        {
                            entry.data = OwnedDecthingsTensor::from_bytes(data)
                                .map_err(|_| crate::client::DecthingsClientError::InvalidMessage)?;
                        }
                        Ok(val)
                    }
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn cancel_training_session(
        &self,
        params: DebugCancelTrainingSessionParams<'_>,
    ) -> Result<
        DebugCancelTrainingSessionResult,
        crate::client::DecthingsRpcError<DebugCancelTrainingSessionError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Debug",
                "cancelTrainingSession",
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
                let res: super::Response<
                    response::DebugCancelTrainingSessionResult,
                    response::DebugCancelTrainingSessionError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn call_evaluate(
        &self,
        params: CallEvaluateParams<'_>,
    ) -> Result<CallEvaluateResult, crate::client::DecthingsRpcError<CallEvaluateError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let serialized = crate::client::serialize_parameter_provider_list(params.params.iter());
        self.rpc
            .raw_method_call(
                "Debug",
                "callEvaluate",
                params,
                serialized,
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
                    response::CallEvaluateResult,
                    response::CallEvaluateError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(mut val) => {
                        if val.output.len() != x.1.len() {
                            return Err(crate::client::DecthingsClientError::InvalidMessage.into());
                        }
                        for (entry, data) in val.output.iter_mut().zip(x.1) {
                            entry.data = super::many_decthings_tensors_from_bytes(data)
                                .map_err(|_| crate::client::DecthingsClientError::InvalidMessage)?;
                        }
                        Ok(val)
                    }
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn call_get_model_state(
        &self,
        params: CallGetModelStateParams<'_>,
    ) -> Result<CallGetModelStateResult, crate::client::DecthingsRpcError<CallGetModelStateError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Debug",
                "callGetModelState",
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
                let res: super::Response<
                    response::CallGetModelStateResult,
                    response::CallGetModelStateError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn download_state_data(
        &self,
        params: DownloadStateDataParams<'_, impl AsRef<str>>,
    ) -> Result<DownloadStateDataResult, crate::client::DecthingsRpcError<DownloadStateDataError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Debug",
                "downloadStateData",
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
                let res: super::Response<
                    response::DownloadStateDataResult,
                    response::DownloadStateDataError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(DownloadStateDataResult {
                        data: val
                            .data
                            .into_iter()
                            .zip(x.1)
                            .map(|(key, data)| super::StateKeyData { key: key.key, data })
                            .collect(),
                    }),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn send_to_remote_inspector(
        &self,
        params: SendToRemoteInspectorParams<'_, impl AsRef<[u8]>>,
    ) -> Result<
        SendToRemoteInspectorResult,
        crate::client::DecthingsRpcError<SendToRemoteInspectorError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call(
                "Debug",
                "sendToRemoteInspector",
                &params,
                [&params.data],
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
                    response::SendToRemoteInspectorResult,
                    response::SendToRemoteInspectorError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    #[cfg(feature = "events")]
    pub async fn subscribe_to_events(
        &self,
        params: DebugSubscribeToEventsParams<'_>,
    ) -> Result<
        DebugSubscribeToEventsResult,
        crate::client::DecthingsRpcError<DebugSubscribeToEventsError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let debug_session_id_owned = params.debug_session_id.to_owned();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Debug",
                "subscribeToEvents",
                params,
                &[],
                crate::client::RpcProtocol::Ws,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<
                                    response::DebugSubscribeToEventsResult,
                                    response::DebugSubscribeToEventsError,
                                >,
                                crate::client::DecthingsRpcError<DebugSubscribeToEventsError>,
                            > = serde_json::from_slice(&val.0).map_err(Into::into);
                            match res {
                                Ok(super::Response::Result(val)) => {
                                    tx.send(Ok(val)).ok();
                                    return StateModification {
                                        add_events: vec![debug_session_id_owned],
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
        params: DebugUnsubscribeFromEventsParams<'_>,
    ) -> Result<
        DebugUnsubscribeFromEventsResult,
        crate::client::DecthingsRpcError<DebugUnsubscribeFromEventsError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let debug_session_id_owned = params.debug_session_id.to_owned();
        let did_call = self
            .rpc
            .raw_method_call::<_, _, &[u8]>(
                "Debug",
                "unsubscribeFromEvents",
                params,
                &[],
                crate::client::RpcProtocol::WsIfAvailableOtherwiseNone,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<
                                    response::DebugUnsubscribeFromEventsResult,
                                    response::DebugUnsubscribeFromEventsError,
                                >,
                                crate::client::DecthingsRpcError<DebugUnsubscribeFromEventsError>,
                            > = serde_json::from_slice(&val.0).map_err(Into::into);
                            match res {
                                Ok(super::Response::Result(val)) => {
                                    tx.send(Ok(val)).ok();
                                    return StateModification {
                                        add_events: vec![],
                                        remove_events: vec![debug_session_id_owned],
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
                DebugUnsubscribeFromEventsError::NotSubscribed,
            ));
        }
        rx.await.unwrap()
    }
}
