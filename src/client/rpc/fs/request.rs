use base64::Engine;
use serde::{Serialize, Serializer};

fn serialize_base64<T: AsRef<[u8]>, S: Serializer>(t: &T, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&base64::engine::general_purpose::STANDARD.encode(t.as_ref()))
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LookupParams<'a, T: AsRef<[u8]>> {
    /// The model's id.
    pub model_id: &'a str,
    /// If provided, the filesystem of the snapshot will be used. Otherwise, the filesystem of the model will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<&'a str>,
    /// Inode number of the parent directory.
    pub parent: u64,
    /// Filename within the parent directory.
    #[serde(serialize_with = "serialize_base64")]
    pub name: T,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetattrParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// If provided, the filesystem of the snapshot will be used. Otherwise, the filesystem of the model will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<&'a str>,
    /// Inode number of file.
    pub inode: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTime {
    pub sec: i64,
    pub nsec: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetattrParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// Inode number of file.
    pub inode: u64,
    /// If specified, file mode to set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<u32>,
    /// If specified the file will be resized to this size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    /// If specified, set access time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atime: Option<SetTime>,
    /// If specified, set modified time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtime: Option<SetTime>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MknodParams<'a, T: AsRef<[u8]>> {
    /// The model's id.
    pub model_id: &'a str,
    /// Inode number of the parent directory.
    pub parent: u64,
    /// Filename within the parent directory.
    #[serde(serialize_with = "serialize_base64")]
    pub name: T,
    /// File mode.
    pub mode: u32,
    /// Device number (for character or block device files).
    pub dev: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// If provided, the filesystem of the snapshot will be used. Otherwise, the filesystem of the model will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<&'a str>,
    /// Inode number of file.
    pub inode: u64,
    /// Where in the file to start reading.
    pub offset: u64,
    /// Number of bytes to read.
    pub count: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// Inode number of file.
    pub inode: u64,
    #[serde(skip_serializing)]
    pub data: &'a [u8],
    /// Where in the file to start writing.
    pub offset: u64,
    /// If true, the file will be truncate to zero length before writing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncate: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SymlinkParams<'a, T1: AsRef<[u8]>, T2: AsRef<[u8]>> {
    /// The model's id.
    pub model_id: &'a str,
    /// Inode number of the parent directory.
    pub parent: u64,
    /// Filename within the parent directory.
    #[serde(serialize_with = "serialize_base64")]
    pub name: T1,
    /// Target name.
    #[serde(serialize_with = "serialize_base64")]
    pub link: T2,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadlinkParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// If provided, the filesystem of the snapshot will be used. Otherwise, the filesystem of the model will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<&'a str>,
    /// Inode number of file.
    pub inode: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MkdirParams<'a, T: AsRef<[u8]>> {
    /// The model's id.
    pub model_id: &'a str,
    /// Inode number of the parent directory.
    pub parent: u64,
    /// Filename within the parent directory.
    #[serde(serialize_with = "serialize_base64")]
    pub name: T,
    /// File mode.
    pub mode: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnlinkParams<'a, T: AsRef<[u8]>> {
    /// The model's id.
    pub model_id: &'a str,
    /// Inode number of the parent directory.
    pub parent: u64,
    /// Filename within the parent directory.
    #[serde(serialize_with = "serialize_base64")]
    pub name: T,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RmdirParams<'a, T: AsRef<[u8]>> {
    /// The model's id.
    pub model_id: &'a str,
    /// Inode number of the parent directory.
    pub parent: u64,
    /// Filename within the parent directory.
    #[serde(serialize_with = "serialize_base64")]
    pub name: T,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameParams<'a, T1: AsRef<[u8]>, T2: AsRef<[u8]>> {
    /// The model's id.
    pub model_id: &'a str,
    /// Inode number of the parent directory.
    pub parent: u64,
    /// Filename within the parent directory.
    #[serde(serialize_with = "serialize_base64")]
    pub name: T1,
    /// Inode number of the new parent directory.
    pub newparent: u64,
    /// Filename within the new parent directory.
    #[serde(serialize_with = "serialize_base64")]
    pub newname: T2,
    /// Optional rename flags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkParams<'a, T: AsRef<[u8]>> {
    /// The model's id.
    pub model_id: &'a str,
    /// Inode number of file.
    pub inode: u64,
    /// Inode number of the new parent directory.
    pub newparent: u64,
    /// Filename within the new parent directory.
    #[serde(serialize_with = "serialize_base64")]
    pub newname: T,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReaddirParams<'a> {
    /// The model's id.
    pub model_id: &'a str,
    /// If provided, the filesystem of the snapshot will be used. Otherwise, the filesystem of the model will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<&'a str>,
    /// Inode number of directory.
    pub inode: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RmdirAllParams<'a, T: AsRef<[u8]>> {
    /// The model's id.
    pub model_id: &'a str,
    /// Inode number of the parent directory.
    pub parent: u64,
    /// Filename within the parent directory.
    #[serde(serialize_with = "serialize_base64")]
    pub name: T,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyParams<'a, T: AsRef<[u8]>> {
    /// The model's id.
    pub model_id: &'a str,
    /// Inode number of file.
    pub inode: u64,
    /// Inode number of the new parent directory.
    pub newparent: u64,
    /// Filename within the new parent directory.
    #[serde(serialize_with = "serialize_base64")]
    pub newname: T,
}
