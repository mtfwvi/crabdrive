use bytes::Bytes;

pub(crate) type TransferSession = u128;

pub(crate) struct FileChunk {
    /// The ID of the chunk
    pub id: i32,
    /// The chunk contents.
    /// The size of the file chunk can be accessed using `FileChunk::data::len()`. It is
    /// usually 16MB, however it may be smaller if this is the last (or only) chunk.
    pub data: Bytes,
}
