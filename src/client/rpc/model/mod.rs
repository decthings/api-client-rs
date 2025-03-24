mod request;
mod response;

use crate::{client::StateModification, tensor::OwnedDecthingsTensor};

pub use request::*;
pub use response::*;

pub struct ModelRpc {
    rpc: crate::client::DecthingsClientRpc,
}

impl ModelRpc {
    pub(crate) fn new(rpc: crate::client::DecthingsClientRpc) -> Self {
        Self { rpc }
    }

    pub async fn create_model(
        &self,
        params: CreateModelParams<'_>,
    ) -> Result<CreateModelResult, crate::client::DecthingsRpcError<CreateModelError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "createModel",
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
                let res: super::Response<CreateModelResult, CreateModelError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn delete_model(
        &self,
        params: DeleteModelParams<'_>,
    ) -> Result<DeleteModelResult, crate::client::DecthingsRpcError<DeleteModelError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "deleteModel",
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
                let res: super::Response<DeleteModelResult, DeleteModelError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn update_model(
        &self,
        params: UpdateModelParams<'_>,
    ) -> Result<UpdateModelResult, crate::client::DecthingsRpcError<UpdateModelError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "updateModel",
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
                let res: super::Response<UpdateModelResult, UpdateModelError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn get_models(
        &self,
        params: GetModelsParams<'_, impl AsRef<str>>,
    ) -> Result<GetModelsResult, crate::client::DecthingsRpcError<GetModelsError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "getModels",
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
                let res: super::Response<GetModelsResult, GetModelsError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn set_filesystem_size(
        &self,
        params: SetFilesystemSizeParams<'_>,
    ) -> Result<SetFilesystemSizeResult, crate::client::DecthingsRpcError<SetFilesystemSizeError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "setFilesystemSize",
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
                let res: super::Response<SetFilesystemSizeResult, SetFilesystemSizeError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn create_model_version(
        &self,
        params: CreateModelVersionParams<'_>,
    ) -> Result<CreateModelVersionResult, crate::client::DecthingsRpcError<CreateModelVersionError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let serialized = crate::client::serialize_parameter_provider_list(params.params.iter());
        self.rpc
            .raw_method_call(
                "Model",
                "createModelVersion",
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
                let res: super::Response<CreateModelVersionResult, CreateModelVersionError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn create_model_version_upload_weights(
        &self,
        params: CreateModelVersionUploadWeightsParams<'_, impl AsRef<[u8]>>,
    ) -> Result<
        CreateModelVersionUploadWeightsResult,
        crate::client::DecthingsRpcError<CreateModelVersionUploadWeightsError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let data = params.data.iter().map(|x| &x.data).collect::<Vec<_>>();
        self.rpc
            .raw_method_call(
                "Model",
                "createModelVersionUploadWeights",
                params,
                data,
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
                    CreateModelVersionUploadWeightsResult,
                    CreateModelVersionUploadWeightsError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn update_model_version(
        &self,
        params: UpdateModelVersionParams<'_>,
    ) -> Result<UpdateModelVersionResult, crate::client::DecthingsRpcError<UpdateModelVersionError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "updateModelVersion",
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
                let res: super::Response<UpdateModelVersionResult, UpdateModelVersionError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn get_weights(
        &self,
        params: GetWeightsParams<'_, impl AsRef<str>>,
    ) -> Result<GetWeightsResult, crate::client::DecthingsRpcError<GetWeightsError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "getWeights",
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
                let res: super::Response<GetWeightsResult, GetWeightsError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(GetWeightsResult {
                        data: val
                            .data
                            .into_iter()
                            .zip(x.1)
                            .map(|(key, data)| super::WeightKeyData { key: key.key, data })
                            .collect(),
                    }),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn delete_model_version(
        &self,
        params: DeleteModelVersionParams<'_>,
    ) -> Result<DeleteModelVersionResult, crate::client::DecthingsRpcError<DeleteModelVersionError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "deleteModelVersion",
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
                let res: super::Response<DeleteModelVersionResult, DeleteModelVersionError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn train(
        &self,
        params: TrainParams<'_>,
    ) -> Result<TrainResult, crate::client::DecthingsRpcError<TrainError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let serialized = crate::client::serialize_parameter_provider_list(params.params.iter());
        self.rpc
            .raw_method_call(
                "Model",
                "train",
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
                let res: super::Response<TrainResult, TrainError> = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn get_training_status(
        &self,
        params: GetTrainingStatusParams<'_>,
    ) -> Result<GetTrainingStatusResult, crate::client::DecthingsRpcError<GetTrainingStatusError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
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
                let res: super::Response<GetTrainingStatusResult, GetTrainingStatusError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn get_training_metrics(
        &self,
        params: GetTrainingMetricsParams<'_>,
    ) -> Result<GetTrainingMetricsResult, crate::client::DecthingsRpcError<GetTrainingMetricsError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
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
                let res: super::Response<GetTrainingMetricsResult, GetTrainingMetricsError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(mut val) => {
                        if val.metrics.iter().map(|x| x.entries.len()).sum::<usize>() != x.1.len() {
                            return Err(crate::client::DecthingsClientError::InvalidMessage.into());
                        }
                        for (entry, data) in val
                            .metrics
                            .iter_mut()
                            .flat_map(|x| x.entries.iter_mut())
                            .zip(x.1)
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
        params: CancelTrainingSessionParams<'_>,
    ) -> Result<
        CancelTrainingSessionResult,
        crate::client::DecthingsRpcError<CancelTrainingSessionError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
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
                let res: super::Response<CancelTrainingSessionResult, CancelTrainingSessionError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn clear_previous_training_session(
        &self,
        params: ClearPreviousTrainingSessionParams<'_>,
    ) -> Result<
        ClearPreviousTrainingSessionResult,
        crate::client::DecthingsRpcError<ClearPreviousTrainingSessionError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "clearPreviousTrainingSession",
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
                    ClearPreviousTrainingSessionResult,
                    ClearPreviousTrainingSessionError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn evaluate<'a>(
        &self,
        params: EvaluateParams<'a>,
    ) -> Result<EvaluateResult, crate::client::DecthingsRpcError<EvaluateError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let serialized = crate::client::serialize_parameter_provider_list(params.params.iter());
        self.rpc
            .raw_method_call(
                "Model",
                "evaluate",
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
                let res: super::Response<EvaluateResult, EvaluateError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(EvaluateResult {
                        durations,
                        executed_on_launcher,
                        mut output,
                    }) => {
                        if output.len() != x.1.len() {
                            return Err(crate::client::DecthingsClientError::InvalidMessage.into());
                        }
                        for (entry, data) in output.iter_mut().zip(x.1) {
                            entry.data = super::many_decthings_tensors_from_bytes(data)
                                .map_err(|_| crate::client::DecthingsClientError::InvalidMessage)?;
                        }
                        Ok(EvaluateResult {
                            durations,
                            executed_on_launcher,
                            output,
                        })
                    }
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn get_evaluations(
        &self,
        params: GetEvaluationsParams<'_>,
    ) -> Result<GetEvaluationsResult, crate::client::DecthingsRpcError<GetEvaluationsError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "getEvaluations",
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
                let res: super::Response<GetEvaluationsResult, GetEvaluationsError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn get_finished_evaluation_result(
        &self,
        params: GetFinishedEvaluationResultParams<'_>,
    ) -> Result<
        GetFinishedEvaluationResultResult,
        crate::client::DecthingsRpcError<GetFinishedEvaluationResultError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "getFinishedEvaluationResult",
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
                    GetFinishedEvaluationResultResult,
                    GetFinishedEvaluationResultError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(GetFinishedEvaluationResultResult {
                        durations,
                        executed_on_launcher,
                        mut output,
                    }) => {
                        if output.len() != x.1.len() {
                            return Err(crate::client::DecthingsClientError::InvalidMessage.into());
                        }
                        for (entry, data) in output.iter_mut().zip(x.1) {
                            entry.data = super::many_decthings_tensors_from_bytes(data)
                                .map_err(|_| crate::client::DecthingsClientError::InvalidMessage)?;
                        }
                        Ok::<
                            GetFinishedEvaluationResultResult,
                            crate::client::DecthingsRpcError<GetFinishedEvaluationResultError>,
                        >(GetFinishedEvaluationResultResult {
                            durations,
                            executed_on_launcher,
                            output,
                        })
                    }
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn cancel_evaluation(
        &self,
        params: CancelEvaluationParams<'_>,
    ) -> Result<CancelEvaluationResult, crate::client::DecthingsRpcError<CancelEvaluationError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "cancelEvaluation",
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
                let res: super::Response<CancelEvaluationResult, CancelEvaluationError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn set_used_persistent_launchers_for_evaluate(
        &self,
        params: SetUsedPersistentLaunchersForEvaluateParams<'_>,
    ) -> Result<
        SetUsedPersistentLaunchersForEvaluateResult,
        crate::client::DecthingsRpcError<SetUsedPersistentLaunchersForEvaluateError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "setUsedPersistentLaunchersForEvaluate",
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
                    SetUsedPersistentLaunchersForEvaluateResult,
                    SetUsedPersistentLaunchersForEvaluateError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn get_used_persistent_launchers_for_evaluate(
        &self,
        params: GetUsedPersistentLaunchersForEvaluateParams<'_>,
    ) -> Result<
        GetUsedPersistentLaunchersForEvaluateResult,
        crate::client::DecthingsRpcError<GetUsedPersistentLaunchersForEvaluateError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "getUsedPersistentLaunchersForEvaluate",
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
                    GetUsedPersistentLaunchersForEvaluateResult,
                    GetUsedPersistentLaunchersForEvaluateError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }
}
