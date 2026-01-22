use crate::storage::vfs::model::{FileChunk, FileError, FileKey, TransferSessionId};
use crabdrive_common::data::DataAmount;
use crabdrive_common::storage::ChunkIndex;

// TODO: Async, Streams?
pub(crate) trait FileRepository {
    // Meta-operations
    /// Checks if the key exists
    fn exists(&self, key: &FileKey) -> bool;
    /// Check if the session exists
    fn session_exists(&self, session: &TransferSessionId) -> bool;
    /// Estimates the number of chunks. Ultimately, however, this is managed by the client.
    fn estimate_chunks(&self, chunk_size: DataAmount) -> ChunkIndex;

    // Writing files
    /// Initiates a new transfer by allocating all necessary temporary resources.
    ///
    /// The returned `TransferSessionId` acts as a handle and is needed for all subsequent operations
    /// (upload, end, abort). The file (and chunks) can be retrieved using the supplied key after
    /// the upload is finished.
    ///
    /// Under the hood, it stores all uploaded chunks in a separate staging area until the upload
    /// is finished.
    fn start_transfer(&mut self, key: FileKey) -> Result<TransferSessionId, FileError>;
    /// Uploads a chunk of data to the active transfer. Not order-sensitive.
    fn write_chunk(&self, session: &TransferSessionId, chunk: FileChunk) -> Result<(), FileError>;
    /// Finalizes ("commit") the transfer and persists it.
    /// **This will invalidate the `TransferSessionId`**.
    fn end_transfer(&mut self, session: TransferSessionId) -> Result<(), FileError>;
    /// Cancels the transfer and cleans up temporary resources.
    /// **This will invalidate the `TransferSessionId`**.
    fn abort_transfer(&mut self, session: TransferSessionId) -> Result<(), FileError>;

    // Read files

    // TODO: The behavior of this method in multithreaded environment should be configurable (f.e. max.
    //       amount of readers) <-> HDD vs. SSD?

    /// Get a chunk from a file (`key`) with the given index and chunk size.
    ///
    /// Example:
    /// ```
    /// let storage = C3::new();
    /// // Retrieve the 13th chunk of the file
    /// storage.get_chunk("MyFile123", 12, 16_000_000);
    /// ```
    fn get_chunk(
        &self,
        key: FileKey,
        chunk_index: ChunkIndex,
        chunk_size: DataAmount,
    ) -> Result<FileChunk, FileError>;
}
