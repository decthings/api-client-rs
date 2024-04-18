mod request;
mod response;

use std::collections::HashSet;

use crate::{client::StateModification, tensor::OwnedDecthingsTensor};
pub use request::*;
pub use response::*;
use serde::Serialize;

/// *data* has one element per dataset key, where each of these elements contains a list of
/// DecthingsTensor. We first assert that the keys are valid (more than one and unique), and that
/// the same number of elements are to be added to each key. Then, the data is serialized by first
/// sorting the data by key (this allows for optimization in the server). Then, the data is
/// serialized using *.serialize()* on the DecthingsTensor. The data is serialized in order by
/// sorted key, and grouped by element. That is, one element for each key is serialized and added
/// after each other, before moving on to the next element. Each element is returned as a separate
/// vec.
fn serialize_add_dataset_data(data: &[DataToAddForKey<'_>]) -> Result<Vec<Vec<u8>>, String> {
    if data.is_empty() {
        return Err(
            "Failed to serialize data: Got zero keys, but a dataset always has at least one key."
                .to_string(),
        );
    }

    let num_entries = data[0].data.len();
    for x in data.iter() {
        if x.data.len() != num_entries {
            return Err(format!(
                    "Failed to serialize data: All keys must contain the same amount of data. Key {} had {num_entries} elements, but key {} had {} elements.",
                    data[0].key,
                    x.key,
                    x.data.len()
                ));
        }
    }

    let mut sorted_keys: Vec<_> = data.iter().map(|x| x.key).collect();
    sorted_keys.sort();

    {
        let mut uniq = HashSet::new();
        if !sorted_keys.iter().all(|x| uniq.insert(x)) {
            return Err(format!(
                "Failed to serialize data: Got duplicate keys. Keys were: {:?}",
                data.iter().map(|x| x.key).collect::<Vec<_>>()
            ));
        }
    }

    let mut res = Vec::with_capacity(num_entries * sorted_keys.len());

    for i in 0..num_entries {
        for &key in &sorted_keys {
            let element = &data.iter().find(|x| x.key == key).unwrap().data[i];
            res.push(element.serialize());
        }
    }

    Ok(res)
}

pub struct DatasetRpc {
    rpc: crate::client::DecthingsClientRpc,
}

impl DatasetRpc {
    pub(crate) fn new(rpc: crate::client::DecthingsClientRpc) -> Self {
        Self { rpc }
    }

    pub async fn create_dataset(
        &self,
        params: CreateDatasetParams<'_>,
    ) -> Result<CreateDatasetResult, crate::client::DecthingsRpcError<CreateDatasetError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Dataset",
                "createDataset",
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
                let res: super::Response<CreateDatasetResult, CreateDatasetError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn update_dataset(
        &self,
        params: UpdateDatasetParams<'_>,
    ) -> Result<UpdateDatasetResult, crate::client::DecthingsRpcError<UpdateDatasetError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Dataset",
                "updateDataset",
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
                let res: super::Response<UpdateDatasetResult, UpdateDatasetError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn delete_dataset(
        &self,
        params: DeleteDatasetParams<'_>,
    ) -> Result<DeleteDatasetResult, crate::client::DecthingsRpcError<DeleteDatasetError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Dataset",
                "deleteDataset",
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
                let res: super::Response<DeleteDatasetResult, DeleteDatasetError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn get_datasets(
        &self,
        params: GetDatasetsParams<'_, impl AsRef<str> + Serialize>,
    ) -> Result<GetDatasetsResult, crate::client::DecthingsRpcError<GetDatasetsError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Dataset",
                "getDatasets",
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
                let res: super::Response<GetDatasetsResult, GetDatasetsError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn add_entries(
        &self,
        params: AddEntriesParams<'_>,
    ) -> Result<AddEntriesResult, crate::client::DecthingsRpcError<AddEntriesError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let serialized = serialize_add_dataset_data(&params.keys).map_err(|e| {
            crate::client::DecthingsRpcError::Rpc(AddEntriesError::InvalidParameter {
                parameter_name: "params.keys".to_string(),
                reason: e,
            })
        })?;
        self.rpc
            .raw_method_call(
                "Dataset",
                "addEntries",
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
                let res: super::Response<AddEntriesResult, AddEntriesError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn add_entries_to_needs_review(
        &self,
        params: AddEntriesToNeedsReviewParams<'_>,
    ) -> Result<
        AddEntriesToNeedsReviewResult,
        crate::client::DecthingsRpcError<AddEntriesToNeedsReviewError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let serialized = serialize_add_dataset_data(&params.keys).map_err(|e| {
            crate::client::DecthingsRpcError::Rpc(AddEntriesToNeedsReviewError::InvalidParameter {
                parameter_name: "params.keys".to_string(),
                reason: e,
            })
        })?;
        self.rpc
            .raw_method_call(
                "Dataset",
                "addEntriesToNeedsReview",
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
                    AddEntriesToNeedsReviewResult,
                    AddEntriesToNeedsReviewError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn finalize_needs_review_entries(
        &self,
        params: FinalizeNeedsReviewEntriesParams<'_>,
    ) -> Result<
        FinalizeNeedsReviewEntriesResult,
        crate::client::DecthingsRpcError<FinalizeNeedsReviewEntriesError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let serialized = serialize_add_dataset_data(&params.keys).map_err(|e| {
            crate::client::DecthingsRpcError::Rpc(
                FinalizeNeedsReviewEntriesError::InvalidParameter {
                    parameter_name: "params.keys".to_string(),
                    reason: e,
                },
            )
        })?;
        if params.indexes.len() != params.keys[0].data.len() {
            return Err(crate::client::DecthingsRpcError::Rpc(
                FinalizeNeedsReviewEntriesError::InvalidParameter {
                    parameter_name: "params.keys".to_string(),
                    reason: format!(
                        "The number of indexes to remove must equal the number of elements to add. Attempted to remove {} indexes and add {} elements.",
                        params.indexes.len(),
                        params.keys[0].data.len()
                    )
                },
            ));
        }
        self.rpc
            .raw_method_call(
                "Dataset",
                "finalizeNeedsReviewEntries",
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
                    FinalizeNeedsReviewEntriesResult,
                    FinalizeNeedsReviewEntriesError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn get_entries(
        &self,
        params: GetEntriesParams<'_>,
    ) -> Result<GetEntriesResult, crate::client::DecthingsRpcError<GetEntriesError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Dataset",
                "getEntries",
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
            .and_then(|mut x| {
                let inner_res: super::Response<InnerGetEntriesResult, GetEntriesError> =
                    serde_json::from_slice(&x.0)?;
                match inner_res {
                    super::Response::Result(val) => {
                        if x.1.len() != val.indexes.len() * val.keys.len() {
                            return Err(crate::client::DecthingsClientError::InvalidMessage.into());
                        }
                        let mut res = GetEntriesResult {
                            keys: val
                                .keys
                                .into_iter()
                                .map(|x| KeyData {
                                    name: x,
                                    data: Vec::with_capacity(val.indexes.len()),
                                })
                                .collect(),
                        };

                        for index in val.indexes {
                            for key in res.keys.iter_mut() {
                                key.data.push(FetchedEntry {
                                    index,
                                    data: OwnedDecthingsTensor::from_bytes(x.1.remove(0)).map_err(
                                        |_| crate::client::DecthingsClientError::InvalidMessage,
                                    )?,
                                });
                            }
                        }
                        Ok(res)
                    }
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn get_needs_review_entries(
        &self,
        params: GetNeedsReviewEntriesParams<'_>,
    ) -> Result<
        GetNeedsReviewEntriesResult,
        crate::client::DecthingsRpcError<GetNeedsReviewEntriesError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Dataset",
                "getNeedsReviewEntries",
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
            .and_then(|mut x| {
                let res: super::Response<
                    InnerGetNeedsReviewEntriesResult,
                    GetNeedsReviewEntriesError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => {
                        if x.1.len() != val.indexes.len() * val.keys.len() {
                            return Err(crate::client::DecthingsClientError::InvalidMessage.into());
                        }
                        let mut res = GetNeedsReviewEntriesResult {
                            keys: val
                                .keys
                                .into_iter()
                                .map(|x| KeyData {
                                    name: x,
                                    data: Vec::with_capacity(val.indexes.len()),
                                })
                                .collect(),
                        };
                        for index in val.indexes {
                            for key in res.keys.iter_mut() {
                                key.data.push(FetchedEntry {
                                    index,
                                    data: OwnedDecthingsTensor::from_bytes(x.1.remove(0)).map_err(
                                        |_| crate::client::DecthingsClientError::InvalidMessage,
                                    )?,
                                });
                            }
                        }
                        Ok(res)
                    }
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn remove_entries(
        &self,
        params: RemoveEntriesParams<'_>,
    ) -> Result<RemoveEntriesResult, crate::client::DecthingsRpcError<RemoveEntriesError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Dataset",
                "removeEntries",
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
                let res: super::Response<RemoveEntriesResult, RemoveEntriesError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn remove_needs_review_entries(
        &self,
        params: RemoveNeedsReviewEntriesParams<'_>,
    ) -> Result<
        RemoveNeedsReviewEntriesResult,
        crate::client::DecthingsRpcError<RemoveNeedsReviewEntriesError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "Dataset",
                "removeNeedsReviewEntries",
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
                    RemoveNeedsReviewEntriesResult,
                    RemoveNeedsReviewEntriesError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }
}
