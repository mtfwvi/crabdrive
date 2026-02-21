mod test;

use crate::storage::vfs::{FileChunk, FileKey, FileRepository};
use bytes::BytesMut;
use crabdrive_common::{da, storage::ChunkIndex};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
};
use tempfile::TempDir;
use tracing::{debug, error, info, instrument};

use crate::storage::vfs::model::{FileStatus, FileSystemError};
use crabdrive_common::storage::RevisionId;
use std::{collections::HashMap, path::PathBuf};

/// S(tupid)imple File System
pub struct Sfs {
    // Internal guard object, which drops the directory
    _temp_dir: Option<TempDir>,
    storage_dir: PathBuf,
    sessions: HashMap<RevisionId, PathBuf>,
}

impl Sfs {
    #[instrument]
    pub fn new(storage_dir: &String) -> Self {
        let temp_dir = if storage_dir == ":temp:" {
            debug!("Configuration requests temporary directory");
            let directory = tempfile::tempdir().expect("Unable to create temporary directory!");
            info!(
                "Temporary Directory: {} (auto-deleted upon server stop)",
                &directory.path().display()
            );
            // Altough it seems tempting, the method `.keep()` (which returns a `PathBuf`), actually
            // disables automatic deletion of the directoy.
            Some(directory)
        } else {
            None
        };

        let storage_dir = if let Some(temp_dir) = &temp_dir {
            temp_dir.path().to_path_buf()
        } else {
            let mut directory = PathBuf::new();

            directory.push(storage_dir);
            if !directory.exists() || !directory.is_dir() {
                error!(
                    "Storage directory {} either does not exist, or is no directory",
                    &storage_dir
                );
                panic!("Invalid storage directory!");
            }
            directory
        };

        Self {
            _temp_dir: temp_dir,
            storage_dir,
            sessions: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl FileRepository for Sfs {
    async fn chunk_exists(&self, key: &FileKey, index: ChunkIndex) -> bool {
        let mut pathbuf = self.storage_dir.clone();
        pathbuf.push(key.to_string());
        pathbuf.push(index.to_string());
        pathbuf.set_extension("bin");
        pathbuf.as_path().exists()
    }

    async fn file_exists(&self, key: &FileKey) -> FileStatus {
        let mut pathbuf = self.storage_dir.clone();
        pathbuf.push(key.to_string());
        if pathbuf.exists() {
            // If path exists and in current session map -> Staged
            if self.sessions.contains_key(key) {
                FileStatus::Staged
            } else {
                FileStatus::Persisted
            }
        } else {
            FileStatus::NotFound
        }
    }

    #[instrument(skip(self), fields(key = %key))]
    async fn create_file(&mut self, key: &FileKey) -> Result<(), FileSystemError> {
        let session = *key;
        let mut pathbuf = self.storage_dir.clone();
        pathbuf.push(key.to_string());
        std::fs::create_dir_all(&pathbuf)?;
        debug!("Chunks will be stored in {}", pathbuf.display());
        self.sessions.insert(session, pathbuf);
        Ok(())
    }

    #[instrument(skip(self, contents), fields(key = %key))]
    async fn write_chunk(
        &mut self,
        key: &FileKey,
        contents: FileChunk,
    ) -> Result<(), FileSystemError> {
        if !self.sessions.contains_key(key) {
            error!("Invalid session");
            return Err(FileSystemError::NotFound);
        }

        let path = self.sessions.get(key).unwrap();

        let mut pathbuf = path.clone();
        pathbuf.push(contents.index.to_string());
        pathbuf.set_extension("bin");
        let mut file_handle = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&pathbuf)?;
        file_handle.write_all(&contents.data)?;

        debug!(
            "Wrote chunk {} (Size: {}) to {}",
            contents.index,
            da!(contents.data.len()),
            pathbuf.display()
        );

        Ok(())
    }

    #[instrument(skip(self), fields(key = %key))]
    async fn commit_file(&mut self, key: &FileKey) -> Result<(), FileSystemError> {
        if self.sessions.contains_key(key) {
            self.sessions.remove(key);
            debug!("Session {} removed", key);
            Ok(())
        } else {
            error!("Invalid session");
            Err(FileSystemError::NotFound)
        }
    }

    async fn abort(&mut self, _: &FileKey) -> Result<(), FileSystemError> {
        unimplemented!("SFS does not implement this functionality.")
    }
    async fn delete_file(&mut self, key: &FileKey) -> Result<(), FileSystemError> {
        if self.file_exists(key).await != FileStatus::Persisted {
            return Err(FileSystemError::NotFound);
        }
        let mut path_buf = self.storage_dir.clone();
        path_buf.push(key.to_string());
        Ok(std::fs::remove_dir_all(&path_buf)?)
    }

    #[instrument(skip(self), fields(key = %key))]
    async fn read_chunk(
        &self,
        key: &FileKey,
        index: ChunkIndex,
    ) -> Result<FileChunk, FileSystemError> {
        if self.file_exists(key).await != FileStatus::Persisted {
            return Err(FileSystemError::NotFound);
        }

        let mut pathbuf = self.storage_dir.clone();
        pathbuf.push(key.to_string());
        pathbuf.push(index.to_string());
        pathbuf.set_extension("bin");

        let mut file_handle = OpenOptions::new().read(true).open(&pathbuf)?;
        debug!("Creating zeroed buffer");

        let chunk_size = file_handle.metadata()?.len();

        let mut bytes = BytesMut::zeroed(chunk_size as usize);

        debug!(
            "Reading {} from {} into buffer",
            &chunk_size,
            &pathbuf.display()
        );

        file_handle.read_exact(&mut bytes)?;

        Ok(FileChunk {
            index,
            data: bytes.freeze(),
        })
    }
}
