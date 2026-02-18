use crate::storage::vfs::model::{FileChunk, FileKey, FileStatus, FileSystemError};
use crabdrive_common::storage::ChunkIndex;

#[async_trait::async_trait]
pub(crate) trait FileRepository {
    async fn chunk_exists(&self, key: &FileKey, index: ChunkIndex) -> bool;
    /// Check if a file exists
    async fn exists(&self, key: &FileKey) -> FileStatus;
    /// Create a new file with the given key in the staging area.
    async fn create(&mut self, key: &FileKey) -> Result<(), FileSystemError>;
    /// Write a new chunk into a file key
    async fn write(&mut self, key: &FileKey, contents: FileChunk) -> Result<(), FileSystemError>;
    /// Transfer a file from the staging area into the permanent storage area
    async fn commit(&mut self, key: &FileKey) -> Result<(), FileSystemError>;
    /// Abort the file upload of a file
    async fn abort(&mut self, key: &FileKey) -> Result<(), FileSystemError>;
    /// Delete a file and all it's chunk contents permanently
    async fn delete(&mut self, key: &FileKey) -> Result<(), FileSystemError>;
    /// Get the contents of a file
    async fn read(&self, key: &FileKey, index: ChunkIndex) -> Result<FileChunk, FileSystemError>;
}
