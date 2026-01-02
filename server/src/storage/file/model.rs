use bytes::Bytes;
use crabdrive_common::storage::ChunkIndex;
use uuid::Uuid;

/// Internal storage key for a file
pub(crate) type FileKey = String;

/// Crated when starting a transfer, this acts as a handle and is needed for all subsequent operations
/// (upload, end, abort).
pub(crate) type TransferSessionId = Uuid;

pub(crate) struct FileChunk {
    pub id: ChunkIndex,
    /// The chunk contents.
    /// The size of the file chunk can be accessed using `FileChunk::data::len()`. It is
    /// usually 16MB, however it may be smaller if this is the last (or only) chunk.
    pub data: Bytes,
}
