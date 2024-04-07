mod request;
mod response;

pub use request::*;
pub use response::*;
use serde::Serialize;

use crate::{tensor::OwnedDecthingsTensor, StateModification};

pub struct DebugRpc {
    rpc: crate::DecthingsClientRpc,
}

impl DebugRpc {
    pub(crate) fn new(rpc: crate::DecthingsClientRpc) -> Self {
        Self { rpc }
    }

    pub async fn launch_debug_session(
        &self,
        params: LaunchDebugSessionParams<'_>,
    ) -> Result<LaunchDebugSessionResult, crate::DecthingsRpcError<LaunchDebugSessionError>> {
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
                                crate::DecthingsRpcError<LaunchDebugSessionError>,
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

    pub async fn get_debug_sessions(
        &self,
        params: GetDebugSessionsParams<'_, impl AsRef<str> + Serialize>,
    ) -> Result<GetDebugSessionsResult, crate::DecthingsRpcError<GetDebugSessionsError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Debug",
                "getDebugSessions",
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
                let res: super::Response<
                    response::GetDebugSessionsResult,
                    response::GetDebugSessionsError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn terminate_debug_session(
        &self,
        params: TerminateDebugSessionParams<'_>,
    ) -> Result<TerminateDebugSessionResult, crate::DecthingsRpcError<TerminateDebugSessionError>>
    {
        #[cfg(feature = "events")]
        let debug_session_id_owned = params.debug_session_id.to_owned();

        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Debug",
                "terminateDebugSession",
                params,
                &[],
                crate::RpcProtocol::Http,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<
                                    response::TerminateDebugSessionResult,
                                    response::TerminateDebugSessionError,
                                >,
                                crate::DecthingsRpcError<TerminateDebugSessionError>,
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

    pub async fn call_create_model_state<D>(
        &self,
        params: CallCreateModelStateParams<'_>,
    ) -> Result<CallCreateModelStateResult, crate::DecthingsRpcError<CallCreateModelStateError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let serialized = crate::serialize_parameter_provider_list(params.params.iter());
        self.rpc
            .raw_method_call(
                "Debug",
                "callCreateModelState",
                params,
                serialized,
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
                    response::CallCreateModelStateResult,
                    response::CallCreateModelStateError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn call_instantiate_model(
        &self,
        params: CallInstantiateModelParams<'_, impl AsRef<[u8]>>,
    ) -> Result<CallInstantiateModelResult, crate::DecthingsRpcError<CallInstantiateModelError>>
    {
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
                    response::CallInstantiateModelResult,
                    response::CallInstantiateModelError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn call_train<D>(
        &self,
        params: CallTrainParams<'_>,
    ) -> Result<CallTrainResult, crate::DecthingsRpcError<CallTrainError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let serialized = crate::serialize_parameter_provider_list(params.params.iter());
        self.rpc
            .raw_method_call(
                "Debug",
                "callTrain",
                params,
                serialized,
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
                let res: super::Response<response::CallTrainResult, response::CallTrainError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn get_training_status(
        &self,
        params: DebugGetTrainingStatusParams<'_>,
    ) -> Result<DebugGetTrainingStatusResult, crate::DecthingsRpcError<DebugGetTrainingStatusError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Debug",
                "getTrainingStatus",
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
                let res: super::Response<
                    response::DebugGetTrainingStatusResult,
                    response::DebugGetTrainingStatusError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn get_training_metrics(
        &self,
        params: DebugGetTrainingMetricsParams<'_>,
    ) -> Result<DebugGetTrainingMetricsResult, crate::DecthingsRpcError<DebugGetTrainingMetricsError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Debug",
                "getTrainingMetrics",
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
                let res: super::Response<
                    response::DebugGetTrainingMetricsResult,
                    response::DebugGetTrainingMetricsError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(mut val) => {
                        if val.metrics.iter().map(|x| x.entries.len()).sum::<usize>() != x.1.len() {
                            return Err(crate::DecthingsClientError::InvalidMessage.into());
                        }
                        for (entry, data) in
                            val.metrics.iter_mut().flat_map(|x| &mut x.entries).zip(x.1)
                        {
                            entry.data = OwnedDecthingsTensor::from_bytes(data)
                                .map_err(|_| crate::DecthingsClientError::InvalidMessage)?;
                        }
                        Ok(val)
                    }
                    super::Response::Error(val) => Err(crate::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn cancel_training_session(
        &self,
        params: DebugCancelTrainingSessionParams<'_>,
    ) -> Result<
        DebugCancelTrainingSessionResult,
        crate::DecthingsRpcError<DebugCancelTrainingSessionError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Debug",
                "cancelTrainingSession",
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
                let res: super::Response<
                    response::DebugCancelTrainingSessionResult,
                    response::DebugCancelTrainingSessionError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn call_evaluate(
        &self,
        params: CallEvaluateParams<'_>,
    ) -> Result<CallEvaluateResult, crate::DecthingsRpcError<CallEvaluateError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let serialized = crate::serialize_parameter_provider_list(params.params.iter());
        self.rpc
            .raw_method_call(
                "Debug",
                "callEvaluate",
                params,
                serialized,
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
                    response::CallEvaluateResult,
                    response::CallEvaluateError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(mut val) => {
                        if val.output.len() != x.1.len() {
                            return Err(crate::DecthingsClientError::InvalidMessage.into());
                        }
                        for (entry, data) in val.output.iter_mut().zip(x.1) {
                            entry.data = OwnedDecthingsTensor::many_from_bytes(data)
                                .map_err(|_| crate::DecthingsClientError::InvalidMessage)?;
                        }
                        Ok(val)
                    }
                    super::Response::Error(val) => Err(crate::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn call_get_model_state(
        &self,
        params: CallGetModelStateParams<'_>,
    ) -> Result<CallGetModelStateResult, crate::DecthingsRpcError<CallGetModelStateError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Debug",
                "callGetModelState",
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
                let res: super::Response<
                    response::CallGetModelStateResult,
                    response::CallGetModelStateError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn download_state_data(
        &self,
        params: DownloadStateDataParams<'_, impl AsRef<str>>,
    ) -> Result<DownloadStateDataResult, crate::DecthingsRpcError<DownloadStateDataError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Debug",
                "downloadStateData",
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
                    super::Response::Error(val) => Err(crate::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn send_to_remote_inspector(
        &self,
        params: SendToRemoteInspectorParams<'_, impl AsRef<[u8]>>,
    ) -> Result<SendToRemoteInspectorResult, crate::DecthingsRpcError<SendToRemoteInspectorError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call(
                "Debug",
                "sendToRemoteInspector",
                &params,
                [&params.data],
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
                    response::SendToRemoteInspectorResult,
                    response::SendToRemoteInspectorError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    #[cfg(feature = "events")]
    pub async fn subscribe_to_events(
        &self,
        params: DebugSubscribeToEventsParams<'_>,
    ) -> Result<DebugSubscribeToEventsResult, crate::DecthingsRpcError<DebugSubscribeToEventsError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let debug_session_id_owned = params.debug_session_id.to_owned();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Debug",
                "subscribeToEvents",
                params,
                &[],
                crate::RpcProtocol::Ws,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<
                                    response::DebugSubscribeToEventsResult,
                                    response::DebugSubscribeToEventsError,
                                >,
                                crate::DecthingsRpcError<DebugSubscribeToEventsError>,
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
        params: DebugUnsubscribeFromEventsParams<'_>,
    ) -> Result<
        DebugUnsubscribeFromEventsResult,
        crate::DecthingsRpcError<DebugUnsubscribeFromEventsError>,
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
                crate::RpcProtocol::WsIfAvailableOtherwiseNone,
                move |x| {
                    match x {
                        Ok(val) => {
                            let res: Result<
                                super::Response<
                                    response::DebugUnsubscribeFromEventsResult,
                                    response::DebugUnsubscribeFromEventsError,
                                >,
                                crate::DecthingsRpcError<DebugUnsubscribeFromEventsError>,
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
                DebugUnsubscribeFromEventsError::NotSubscribed,
            ));
        }
        rx.await.unwrap()
    }
}
