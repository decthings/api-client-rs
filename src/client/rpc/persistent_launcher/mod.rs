mod request;
mod response;

pub use request::*;
pub use response::*;
use serde::Serialize;

use crate::client::StateModification;

pub struct PersistentLauncherRpc {
    rpc: crate::client::DecthingsClientRpc,
}

impl PersistentLauncherRpc {
    pub(crate) fn new(rpc: crate::client::DecthingsClientRpc) -> Self {
        Self { rpc }
    }

    pub async fn create_persistent_launcher(
        &self,
        params: CreatePersistentLauncherParams<'_>,
    ) -> Result<
        CreatePersistentLauncherResult,
        crate::client::DecthingsRpcError<CreatePersistentLauncherError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "PersistentLauncher",
                "createPersistentLauncher",
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
                    CreatePersistentLauncherResult,
                    CreatePersistentLauncherError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn get_persistent_launchers(
        &self,
        params: GetPersistentLaunchersParams<'_, impl AsRef<str> + Serialize>,
    ) -> Result<
        GetPersistentLaunchersResult,
        crate::client::DecthingsRpcError<GetPersistentLaunchersError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "PersistentLaunchers",
                "getPersistentLaunchers",
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
                    GetPersistentLaunchersResult,
                    GetPersistentLaunchersError,
                > = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn get_sysinfo(
        &self,
        params: GetSysinfoParams<'_>,
    ) -> Result<GetSysinfoResult, crate::client::DecthingsRpcError<GetSysinfoError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "PersistentLaunchers",
                "getSysinfo",
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
                let res: super::Response<GetSysinfoResult, GetSysinfoError> =
                    serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }

    pub async fn delete_persistent_launcher(
        &self,
        params: DeletePersistentLauncherParams<'_>,
    ) -> Result<
        DeletePersistentLauncherResult,
        crate::client::DecthingsRpcError<DeletePersistentLauncherError>,
    > {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.rpc
            .raw_method_call::<_, _, &[u8]>(
                "PersistentLaunchers",
                "deletePersistentLauncher",
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
                let res = serde_json::from_slice(&x.0)?;
                match res {
                    super::Response::Result(val) => Ok(val),
                    super::Response::Error(val) => Err(crate::client::DecthingsRpcError::Rpc(val)),
                }
            })
    }
}
