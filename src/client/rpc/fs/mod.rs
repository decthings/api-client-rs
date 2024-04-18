mod request;
mod response;

pub use request::*;
pub use response::*;

use crate::client::StateModification;

pub struct FsRpc {
    rpc: crate::client::DecthingsClientRpc,
}

impl FsRpc {
    pub(crate) fn new(rpc: crate::client::DecthingsClientRpc) -> Self {
        Self { rpc }
    }

    pub async fn lookup(
        &self,
        params: LookupParams<'_, impl AsRef<[u8]>>,
    ) -> Result<LookupResult, crate::client::DecthingsRpcError<LookupError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "FS",
                "lookup",
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
                let res: super::Response<LookupResult, LookupError> = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn getattr(
        &self,
        params: GetattrParams<'_>,
    ) -> Result<GetattrResult, crate::client::DecthingsRpcError<GetattrError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "FS",
                "getattr",
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
                let res: super::Response<GetattrResult, GetattrError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn setattr(
        &self,
        params: SetattrParams<'_>,
    ) -> Result<SetattrResult, crate::client::DecthingsRpcError<SetattrError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "FS",
                "setattr",
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
                let res: super::Response<SetattrResult, SetattrError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn mknod(
        &self,
        params: MknodParams<'_, impl AsRef<[u8]>>,
    ) -> Result<MknodResult, crate::client::DecthingsRpcError<MknodError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "FS",
                "mknod",
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
                let res: super::Response<MknodResult, MknodError> = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn read(
        &self,
        params: ReadParams<'_>,
    ) -> Result<ReadResult, crate::client::DecthingsRpcError<ReadError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "FS",
                "read",
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
                let res: super::Response<ReadResult, ReadError> = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(ReadResult { .. }) => {
                        if x.1.is_empty() {
                            return Err(crate::client::DecthingsClientError::InvalidMessage.into());
                        }
                        Ok(ReadResult {
                            data: x.1.remove(0),
                        })
                    }
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn write(
        &self,
        params: WriteParams<'_>,
    ) -> Result<WriteResult, crate::client::DecthingsRpcError<WriteError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call(
                "FS",
                "write",
                &params,
                &[params.data],
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
                let res: super::Response<WriteResult, WriteError> = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn symlink(
        &self,
        params: SymlinkParams<'_, impl AsRef<[u8]>, impl AsRef<[u8]>>,
    ) -> Result<SymlinkResult, crate::client::DecthingsRpcError<SymlinkError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "FS",
                "symlink",
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
                let res: super::Response<SymlinkResult, SymlinkError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn readlink(
        &self,
        params: ReadlinkParams<'_>,
    ) -> Result<ReadlinkResult, crate::client::DecthingsRpcError<ReadlinkError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "FS",
                "readlink",
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
                let res: super::Response<ReadlinkResult, ReadlinkError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(ReadlinkResult { .. }) => {
                        if x.1.is_empty() {
                            return Err(crate::client::DecthingsClientError::InvalidMessage.into());
                        }
                        Ok(ReadlinkResult {
                            link: x.1.remove(0),
                        })
                    }
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn mkdir(
        &self,
        params: MkdirParams<'_, impl AsRef<[u8]>>,
    ) -> Result<MkdirResult, crate::client::DecthingsRpcError<MkdirError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "FS",
                "mkdir",
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
                let res: super::Response<MkdirResult, MkdirError> = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn unlink(
        &self,
        params: UnlinkParams<'_, impl AsRef<[u8]>>,
    ) -> Result<UnlinkResult, crate::client::DecthingsRpcError<UnlinkError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "FS",
                "unlink",
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
                let res: super::Response<UnlinkResult, UnlinkError> = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn rmdir(
        &self,
        params: RmdirParams<'_, impl AsRef<[u8]>>,
    ) -> Result<RmdirResult, crate::client::DecthingsRpcError<RmdirError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "FS",
                "rmdir",
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
                let res: super::Response<RmdirResult, RmdirError> = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn rename(
        &self,
        params: RenameParams<'_, impl AsRef<[u8]>, impl AsRef<[u8]>>,
    ) -> Result<RenameResult, crate::client::DecthingsRpcError<RenameError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "FS",
                "rename",
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
                let res: super::Response<RenameResult, RenameError> = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn link(
        &self,
        params: LinkParams<'_, impl AsRef<[u8]>>,
    ) -> Result<LinkResult, crate::client::DecthingsRpcError<LinkError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "FS",
                "link",
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
                let res: super::Response<LinkResult, LinkError> = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn readdir(
        &self,
        params: ReaddirParams<'_>,
    ) -> Result<ReaddirResult, crate::client::DecthingsRpcError<ReaddirError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "FS",
                "readdir",
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
                let res: super::Response<ReaddirResult, ReaddirError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn rmdir_all(
        &self,
        params: RmdirAllParams<'_, impl AsRef<[u8]>>,
    ) -> Result<RmdirAllResult, crate::client::DecthingsRpcError<RmdirAllError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "FS",
                "rmdirAll",
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
                let res: super::Response<RmdirAllResult, RmdirAllError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn copy(
        &self,
        params: CopyParams<'_, impl AsRef<[u8]>>,
    ) -> Result<CopyResult, crate::client::DecthingsRpcError<CopyError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "FS",
                "copy",
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
                let res: super::Response<CopyResult, CopyError> = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }
}
