mod test;

use crate::storage::vfs::{
    FileChunk, FileKey, FileRepository, TransferSessionId, model::FileError,
};
use bytes::BytesMut;
use crabdrive_common::{da, storage::ChunkIndex};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
};
use tempfile::TempDir;
use tracing::{debug, debug_span, error, info};

use std::{collections::HashMap, path::PathBuf};

/// S(tupid)imple File System
pub struct Sfs {
    // Internal guard object, which drops the directory
    _temp_dir: Option<TempDir>,
    storage_dir: PathBuf,
    sessions: HashMap<TransferSessionId, PathBuf>,
}

impl Sfs {
    pub fn new(storage_dir: &String) -> Self {
        let _span = debug_span!("Sfs::new").entered();
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
            let mut directory = std::path::PathBuf::new();

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

impl FileRepository for Sfs {
    fn exists(&self, key: &FileKey) -> bool {
        let mut pathbuf = self.storage_dir.clone();
        pathbuf.push(key);
        pathbuf.exists()
    }

    fn chunk_exists(&self, key: &FileKey, chunk_index: ChunkIndex) -> bool {
        let mut pathbuf = self.storage_dir.clone();
        pathbuf.push(key);
        pathbuf.push(chunk_index.to_string());
        pathbuf.set_extension("bin");
        pathbuf.exists()
    }

    fn session_exists(&self, session: &TransferSessionId) -> bool {
        self.sessions.contains_key(session)
    }

    fn estimate_chunks(&self, _chunk_size: crabdrive_common::data::DataAmount) -> ChunkIndex {
        unimplemented!("SFS does not implement this functionality.")
    }

    fn start_transfer(&mut self, key: FileKey) -> Result<TransferSessionId, FileError> {
        let session = key.clone();

        let _s = debug_span!(
            "StartTransfer",
            key = key.to_string(),
            session = session.to_string()
        )
        .entered();

        let mut pathbuf = self.storage_dir.clone();
        pathbuf.push(&key);
        std::fs::create_dir_all(&pathbuf)?;
        debug!("Chunks will be stored in {}", pathbuf.display());
        self.sessions.insert(session.clone(), pathbuf);
        Ok(session)
    }

    fn write_chunk(&self, session: &TransferSessionId, chunk: FileChunk) -> Result<(), FileError> {
        let _s = debug_span!("WriteChunk", session = session.to_string()).entered();

        if !self.sessions.contains_key(session) {
            error!("Invalid session");
            return Err(FileError::InvalidSession);
        }

        let path = self.sessions.get(session).unwrap();

        let mut pathbuf = path.clone();
        pathbuf.push(chunk.index.to_string());
        pathbuf.set_extension("bin");
        let mut file_handle = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&pathbuf)?;
        file_handle.write_all(&chunk.data)?;
        debug!(
            "Wrote chunk {} (Size: {}) to {}",
            chunk.index,
            da!(chunk.data.len()),
            pathbuf.display()
        );

        Ok(())
    }

    fn end_transfer(&mut self, session: &TransferSessionId) -> Result<(), FileError> {
        let _s = debug_span!("EndTransfer", session = session.to_string()).entered();
        if self.session_exists(session) {
            self.sessions.remove(session);
            debug!("Session {} removed", session);
            Ok(())
        } else {
            error!("Invalid session");
            Err(FileError::InvalidSession)
        }
    }

    fn abort_transfer(&mut self, _session: TransferSessionId) -> Result<(), FileError> {
        unimplemented!("SFS does not support this functionality.")
    }

    fn get_chunk(
        &self,
        key: FileKey,
        chunk_index: ChunkIndex,
        _chunk_size: crabdrive_common::data::DataAmount,
    ) -> Result<FileChunk, FileError> {
        let _s = debug_span!("GetChunk", key = key).entered();

        if !self.exists(&key) {
            return Err(FileError::KeyNotFound);
        }

        let mut pathbuf = self.storage_dir.clone();
        pathbuf.push(&key);
        pathbuf.push(chunk_index.to_string());
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
            index: chunk_index,
            data: bytes.freeze(),
        })
    }
}
