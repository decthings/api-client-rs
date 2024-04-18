mod request;
mod response;

pub use request::*;
pub use response::*;

use crate::{client::StateModification, tensor::OwnedDecthingsTensor};

pub struct ModelRpc {
    rpc: crate::client::DecthingsClientRpc,
}

impl ModelRpc {
    pub(crate) fn new(rpc: crate::client::DecthingsClientRpc) -> Self {
        Self { rpc }
    }

    pub async fn create_model(
        &self,
        params: CreateModelParams<'_, impl AsRef<[u8]>>,
    ) -> Result<CreateModelResult, crate::client::DecthingsRpcError<CreateModelError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        match &params.options {
            CreateModelOptions::BasedOnModelSnapshot {
                model_id: _,
                snapshot_id: _,
                initial_state: CreateModelInitialState::Copy,
            }
            | CreateModelOptions::Code { .. }
            | CreateModelOptions::FromExisting { .. } => {
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
            }
            CreateModelOptions::Upload {
                tags: _,
                parameter_definitions: _,
                format: _,
                data,
            } => {
                self.rpc
                    .raw_method_call::<_, _, &[u8]>(
                        "Model",
                        "createModel",
                        &params,
                        &[*data],
                        crate::client::RpcProtocol::Http,
                        |x| {
                            tx.send(x).ok();
                            StateModification::empty()
                        },
                    )
                    .await;
            }
            CreateModelOptions::BasedOnModelSnapshot {
                model_id: _,
                snapshot_id: _,
                initial_state: CreateModelInitialState::Upload { name: _, data },
            } => {
                self.rpc
                    .raw_method_call(
                        "Model",
                        "createModel",
                        &params,
                        data.iter().map(|x| &x.data).collect::<Vec<_>>(),
                        crate::client::RpcProtocol::Http,
                        |x| {
                            tx.send(x).ok();
                            StateModification::empty()
                        },
                    )
                    .await;
            }
            CreateModelOptions::BasedOnModelSnapshot {
                model_id: _,
                snapshot_id: _,
                initial_state:
                    CreateModelInitialState::Create {
                        name: _,
                        params,
                        launcher_spec: _,
                    },
            } => {
                let serialized = crate::client::serialize_parameter_provider_list(params.iter());
                self.rpc
                    .raw_method_call(
                        "Model",
                        "createModel",
                        params,
                        serialized,
                        crate::client::RpcProtocol::Http,
                        |x| {
                            tx.send(x).ok();
                            StateModification::empty()
                        },
                    )
                    .await;
            }
        };
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

    pub async fn wait_for_model_to_be_created(
        &self,
        params: WaitForModelToBeCreatedParams<'_>,
    ) -> Result<
        WaitForModelToBeCreatedResult,
        crate::client::DecthingsRpcError<WaitForModelToBeCreatedError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "waitForModelToBeCrated",
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
                    WaitForModelToBeCreatedResult,
                    WaitForModelToBeCreatedError,
                > = serde_json::from_slice(&x.0)?;
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

    pub async fn snapshot_model(
        &self,
        params: SnapshotModelParams<'_>,
    ) -> Result<SnapshotModelResult, crate::client::DecthingsRpcError<SnapshotModelError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "snapshotModel",
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
                let res: super::Response<SnapshotModelResult, SnapshotModelError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn update_snapshot(
        &self,
        params: UpdateSnapshotParams<'_>,
    ) -> Result<UpdateSnapshotResult, crate::client::DecthingsRpcError<UpdateSnapshotError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "updateSnapshot",
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
                let res: super::Response<UpdateSnapshotResult, UpdateSnapshotError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn delete_snapshot(
        &self,
        params: DeleteSnapshotParams<'_>,
    ) -> Result<DeleteSnapshotResult, crate::client::DecthingsRpcError<DeleteSnapshotError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "deleteSnapshot",
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
                let res: super::Response<DeleteSnapshotResult, DeleteSnapshotError> =
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
    pub async fn get_filesystem_usage(
        &self,
        params: GetFilesystemUsageParams<'_>,
    ) -> Result<GetFilesystemUsageResult, crate::client::DecthingsRpcError<GetFilesystemUsageError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "getFilesystemUsage",
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
                let res: super::Response<GetFilesystemUsageResult, GetFilesystemUsageError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }
    pub async fn create_state(
        &self,
        params: CreateStateParams<'_>,
    ) -> Result<CreateStateResult, crate::client::DecthingsRpcError<CreateStateError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let serialized = crate::client::serialize_parameter_provider_list(params.params.iter());
        self.rpc
            .raw_method_call(
                "Model",
                "createState",
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
                let res: super::Response<CreateStateResult, CreateStateError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn upload_state(
        &self,
        params: UploadStateParams<'_, impl AsRef<str>, impl AsRef<[u8]>>,
    ) -> Result<UploadStateResult, crate::client::DecthingsRpcError<UploadStateError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let data = params.data.iter().map(|x| &x.data).collect::<Vec<_>>();
        self.rpc
            .raw_method_call(
                "Model",
                "uploadState",
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
                let res: super::Response<UploadStateResult, UploadStateError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn cancel_create_state(
        &self,
        params: CancelCreateStateParams<'_>,
    ) -> Result<CancelCreateStateResult, crate::client::DecthingsRpcError<CancelCreateStateError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "cancelCreateState",
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
                let res: super::Response<CancelCreateStateResult, CancelCreateStateError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn get_creating_states(
        &self,
        params: GetCreatingStatesParams<'_>,
    ) -> Result<GetCreatingStatesResult, crate::client::DecthingsRpcError<GetCreatingStatesError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "getCreatingStates",
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
                let res: super::Response<GetCreatingStatesResult, GetCreatingStatesError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn wait_for_state_to_be_created(
        &self,
        params: WaitForStateToBeCreatedParams<'_>,
    ) -> Result<
        WaitForStateToBeCreatedResult,
        crate::client::DecthingsRpcError<WaitForStateToBeCreatedError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "waitForStateToBeCreated",
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
                    WaitForStateToBeCreatedResult,
                    WaitForStateToBeCreatedError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn update_model_state(
        &self,
        params: UpdateModelStateParams<'_>,
    ) -> Result<UpdateModelStateResult, crate::client::DecthingsRpcError<UpdateModelStateError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "updateModelState",
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
                let res: super::Response<UpdateModelStateResult, UpdateModelStateError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn set_active_model_state(
        &self,
        params: SetActiveModelStateParams<'_>,
    ) -> Result<SetActiveModelStateResult, crate::client::DecthingsRpcError<SetActiveModelStateError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "setActiveModelState",
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
                let res: super::Response<SetActiveModelStateResult, SetActiveModelStateError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn delete_model_state(
        &self,
        params: DeleteModelStateParams<'_>,
    ) -> Result<DeleteModelStateResult, crate::client::DecthingsRpcError<DeleteModelStateError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "deleteModelState",
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
                let res: super::Response<DeleteModelStateResult, DeleteModelStateError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn get_model_state(
        &self,
        params: GetModelStateParams<'_, impl AsRef<str>>,
    ) -> Result<GetModelStateResult, crate::client::DecthingsRpcError<GetModelStateError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "getModelState",
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
                let res: super::Response<GetModelStateResult, GetModelStateError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(GetModelStateResult {
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

    pub async fn get_snapshot_state(
        &self,
        params: GetSnapshotStateParams<'_, impl AsRef<str>>,
    ) -> Result<GetSnapshotStateResult, crate::client::DecthingsRpcError<GetSnapshotStateError>>
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Model",
                "getSnapshotState",
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
                let res: super::Response<GetSnapshotStateResult, GetSnapshotStateError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(GetSnapshotStateResult {
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
                    super::Response::Result(EvaluateResult::Success {
                        total_duration,
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
                        Ok(EvaluateResult::Success {
                            total_duration,
                            durations,
                            executed_on_launcher,
                            output,
                        })
                    }
                    super::Response::Result(val) => Ok(val),
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
                    super::Response::Result(GetFinishedEvaluationResultResult::Success {
                        total_duration,
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
                        >(GetFinishedEvaluationResultResult::Success {
                            total_duration,
                            durations,
                            executed_on_launcher,
                            output,
                        })
                    }
                    super::Response::Result(val) => Ok(val),
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
