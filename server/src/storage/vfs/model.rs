use bytes::Bytes;
use crabdrive_common::storage::ChunkIndex;
use crabdrive_common::uuid::UUID;

#[derive(Debug)]
pub(crate) enum FileError {
    KeyNotFound,
    InvalidSession,
    InvalidLength,
    Io(std::io::Error),
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::KeyNotFound => write!(f, "Key not found"),
            FileError::InvalidSession => write!(f, "Invalid session key"),
            FileError::InvalidLength => write!(f, "Invalid length"),
            FileError::Io(error_kind) => write!(f, "IO ({})", error_kind),
        }?;
        Ok(())
    }
}

impl From<std::io::Error> for FileError {
    fn from(value: std::io::Error) -> Self {
        FileError::Io(value)
    }
}

impl std::error::Error for FileError {}

/// Internal storage key for a file
pub(crate) type FileKey = String;

/// Crated when starting a transfer, this acts as a handle and is needed for all subsequent operations
/// (upload, end, abort).
pub(crate) type TransferSessionId = UUID;

pub(crate) struct FileChunk {
    pub index: ChunkIndex,
    /// The chunk contents.
    /// The size of the file chunk can be accessed using `FileChunk::data::len()`. It is
    /// usually 16MB, however it may be smaller if this is the last (or only) chunk.
    pub data: Bytes,
}
