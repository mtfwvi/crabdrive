use crate::storage::file::model::{FileChunk, TransferSession};
use anyhow::Result;

// TODO: Async, Streams?
pub(crate) trait FileRepository {
    // Meta-operations
    /// Checks if the key exists
    fn exists(&self, key: &str) -> bool;
    /// Check if the session exists
    fn session_exists(&self, session: &TransferSession) -> bool;
    /// Estimates the number of chunks. Ultimately, this is however managed by the client.
    fn estimate_chunks(&self, c_size: u64) -> u64;

    // Writing files
    /// Initiates a new transfer by allocating all necessary temporary resources.
    ///
    /// The returned `TransferSession` acts as a handle and is needed for all subsequent operations
    /// (upload, end, abort). The file (and chunks) can be retrieved using the supplied key after
    /// the upload is finished.
    ///
    /// Under the hood, it stores all uploaded chunks in a sepearte staging area until the upload
    /// is finished.
    fn start_transfer(&self, key: &str) -> Result<TransferSession>;
    /// Uploads a chunk of data to the active transfer. Not order-sensitive.
    fn write_chunk(&self, session: &TransferSession, chunk: FileChunk) -> Result<()>;
    /// Finalizes ("commit") the transfer and persists it.
    /// **This will invalidate the `TransferSession`**.
    fn end_transfer(&self, session: TransferSession) -> Result<()>;
    /// Cancels the transfer and cleans up temporary resources.
    /// **This will invalidate the `TransferSession`**.
    fn abort_transfer(&self, session: TransferSession) -> Result<()>;

    // Read files

    // TODO: The behavior of this method in multithreaded environment should be configurable (f.e. max.
    //       amount of readers) <-> HDD vs. SSD?

    /// Get a chunk from a file (`key`) with the given index and chunk size.
    ///
    /// Example:
    /// ```
    /// let storage = C3::new();
    /// // Retrieve the 13th chunk of the file
    /// storage.get_chunk("MyFile123", 13, 16_000_000);
    /// ```
    fn get_chunk(&self, key: &str, c_index: u64, c_size: u64) -> Result<FileChunk>;
}
