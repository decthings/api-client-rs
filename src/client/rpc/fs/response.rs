use base64::Engine;
use serde::{de::Visitor, Deserialize, Deserializer};

fn deserialize_base64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
    struct Base64Visitor;

    impl<'de> Visitor<'de> for Base64Visitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string containing json data")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            base64::engine::general_purpose::STANDARD
                .decode(v)
                .map_err(E::custom)
        }
    }

    deserializer.deserialize_str(Base64Visitor)
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    pub mode: u32,
    pub nlink: u64,
    pub rdev: u64,
    pub size: u64,
    pub blksize: u64,
    pub blocks: u64,
    pub atime: i64,
    pub atime_nsec: i64,
    pub mtime: i64,
    pub mtime_nsec: i64,
    pub ctime: i64,
    pub ctime_nsec: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LookupResult {
    pub inode: u64,
    pub stat: Stat,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum LookupError {
    ModelNotFound,
    SnapshotNotFound,
    #[serde(rename = "ESTALE")]
    ESTALE,
    #[serde(rename = "ESTALE")]
    ENOENT,
    #[serde(rename = "ESTALE")]
    ENOTDIR,
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetattrResult {
    pub stat: Stat,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum GetattrError {
    ModelNotFound,
    SnapshotNotFound,
    #[serde(rename = "ESTALE")]
    ESTALE,
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetattrResult {
    pub stat: Stat,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum SetattrError {
    ModelNotFound,
    AccessDenied,
    #[serde(rename = "ESTALE")]
    ESTALE,
    #[serde(rename = "ESTALE")]
    EFBIG,
    #[serde(rename = "ESTALE")]
    EISDIR,
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MknodResult {
    pub inode: u64,
    pub stat: Stat,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum MknodError {
    ModelNotFound,
    AccessDenied,
    #[serde(rename = "ESTALE")]
    ESTALE,
    #[serde(rename = "ESTALE")]
    ENOTDIR,
    #[serde(rename = "ESTALE")]
    EEXIST,
    #[serde(rename = "ESTALE")]
    ENOSPC,
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadResult {
    #[serde(skip_deserializing)]
    pub data: bytes::Bytes,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum ReadError {
    ModelNotFound,
    SnapshotNotFound,
    #[serde(rename = "ESTALE")]
    ESTALE,
    #[serde(rename = "ESTALE")]
    EISDIR,
    #[serde(rename = "ESTALE")]
    EINVAL,
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteResult {
    pub bytes_written: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum WriteError {
    ModelNotFound,
    AccessDenied,
    #[serde(rename = "ESTALE")]
    ESTALE,
    #[serde(rename = "ESTALE")]
    EISDIR,
    #[serde(rename = "ESTALE")]
    ENOSPC,
    #[serde(rename = "ESTALE")]
    EINVAL,
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymlinkResult {
    pub inode: u64,
    pub stat: Stat,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum SymlinkError {
    ModelNotFound,
    AccessDenied,
    #[serde(rename = "ESTALE")]
    ESTALE,
    #[serde(rename = "ESTALE")]
    ENOTDIR,
    #[serde(rename = "ESTALE")]
    EEXIST,
    #[serde(rename = "ESTALE")]
    ENOSPC,
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadlinkResult {
    #[serde(skip_deserializing)]
    pub link: bytes::Bytes,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum ReadlinkError {
    ModelNotFound,
    SnapshotNotFound,
    #[serde(rename = "ESTALE")]
    ESTALE,
    #[serde(rename = "ESTALE")]
    EINVAL,
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MkdirResult {
    pub inode: u64,
    pub stat: Stat,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum MkdirError {
    ModelNotFound,
    AccessDenied,
    #[serde(rename = "ESTALE")]
    ESTALE,
    #[serde(rename = "ESTALE")]
    ENOTDIR,
    #[serde(rename = "ESTALE")]
    EEXIST,
    #[serde(rename = "ESTALE")]
    ENOSPC,
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnlinkResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum UnlinkError {
    ModelNotFound,
    AccessDenied,
    #[serde(rename = "ESTALE")]
    ESTALE,
    #[serde(rename = "ESTALE")]
    ENOTDIR,
    #[serde(rename = "ESTALE")]
    ENOENT,
    #[serde(rename = "ESTALE")]
    EISDIR,
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RmdirResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum RmdirError {
    ModelNotFound,
    AccessDenied,
    #[serde(rename = "ESTALE")]
    ESTALE,
    #[serde(rename = "ESTALE")]
    ENOTDIR,
    #[serde(rename = "ESTALE")]
    ENOENT,
    #[serde(rename = "ESTALE")]
    ENOTEMPTY,
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum RenameError {
    ModelNotFound,
    AccessDenied,
    #[serde(rename = "ESTALE")]
    ESTALE,
    #[serde(rename = "ESTALE")]
    ENOTDIR,
    #[serde(rename = "ESTALE")]
    ENOENT,
    #[serde(rename = "ESTALE")]
    ENOTEMPTY,
    #[serde(rename = "ESTALE")]
    EEXIST,
    #[serde(rename = "ESTALE")]
    EISDIR,
    #[serde(rename = "ESTALE")]
    ENOSPC,
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkResult {
    pub stat: Stat,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum LinkError {
    ModelNotFound,
    AccessDenied,
    #[serde(rename = "ESTALE")]
    ESTALE,
    #[serde(rename = "ESTALE")]
    ENOTDIR,
    #[serde(rename = "ESTALE")]
    EPERM,
    #[serde(rename = "ESTALE")]
    EEXIST,
    #[serde(rename = "ESTALE")]
    ENOSPC,
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReaddirEntry {
    /// Filename
    #[serde(deserialize_with = "deserialize_base64")]
    pub basename: Vec<u8>,
    /// File mode
    pub filetype: u32,
    /// Inode number
    pub ino: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReaddirResult {
    pub entries: Vec<ReaddirEntry>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum ReaddirError {
    ModelNotFound,
    SnapshotNotFound,
    #[serde(rename = "ESTALE")]
    ESTALE,
    #[serde(rename = "ESTALE")]
    ENOTDIR,
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RmdirAllResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum RmdirAllError {
    ModelNotFound,
    AccessDenied,
    #[serde(rename = "ESTALE")]
    ESTALE,
    #[serde(rename = "ESTALE")]
    ENOTDIR,
    #[serde(rename = "ESTALE")]
    ENOENT,
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyResult {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum CopyError {
    ModelNotFound,
    AccessDenied,
    #[serde(rename = "ESTALE")]
    ESTALE,
    #[serde(rename = "ESTALE")]
    ENOTDIR,
    #[serde(rename = "ESTALE")]
    EEXIST,
    #[serde(rename = "ESTALE")]
    ENOSPC,
    BadCredentials,
    TooManyRequests,
    PaymentRequired,
    Unknown,
    #[serde(rename_all = "camelCase")]
    InvalidParameter {
        parameter_name: String,
        reason: String,
    },
}
