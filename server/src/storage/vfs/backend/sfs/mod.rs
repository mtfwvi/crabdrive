mod test;

use crate::storage::vfs::{FileChunk, FileRepository, TransferSessionId, model::FileError};
use bytes::BytesMut;
use crabdrive_common::{da, storage::ChunkIndex, uuid::UUID};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
};
use tracing::{debug, debug_span, error};

use std::{collections::HashMap, path::PathBuf};

/// S(tupid)imple File System
pub struct Sfs {
    storage_dir: std::path::PathBuf,
    sessions: HashMap<UUID, PathBuf>,
}

impl Sfs {
    pub fn new(storage_dir: std::path::PathBuf) -> Self {
        if !storage_dir.exists() || !storage_dir.is_dir() {
            panic!("Invalid storage directory!");
        }
        Self {
            storage_dir,
            sessions: HashMap::new(),
        }
    }
}

impl FileRepository for Sfs {
    fn exists(&self, key: &crate::storage::vfs::FileKey) -> bool {
        let mut pathbuf = self.storage_dir.clone();
        pathbuf.push(key);
        pathbuf.exists()
    }

    fn session_exists(&self, session: &crate::storage::vfs::TransferSessionId) -> bool {
        self.sessions.contains_key(session)
    }

    fn estimate_chunks(&self, _chunk_size: crabdrive_common::data::DataAmount) -> ChunkIndex {
        unimplemented!("SFS does not implement this functionality.")
    }

    fn start_transfer(
        &mut self,
        key: crate::storage::vfs::FileKey,
    ) -> Result<TransferSessionId, FileError> {
        let session = UUID::random();

        let _s = debug_span!(
            "StartTransfer",
            key = key.to_string(),
            session = session.to_string()
        )
        .entered();

        let mut pathbuf = self.storage_dir.clone();
        pathbuf.push(&key);
        std::fs::create_dir(&pathbuf)?;
        debug!("Chunks will be stored in {}", pathbuf.display());
        self.sessions.insert(session, pathbuf);
        Ok(session)
    }

    fn write_chunk(
        &self,
        session: &crate::storage::vfs::TransferSessionId,
        chunk: crate::storage::vfs::FileChunk,
    ) -> Result<(), FileError> {
        let _s = debug_span!("WriteChunk", session = session.to_string()).entered();
        if let Some(path) = self.sessions.get(session) {
            let mut path = path.clone();
            path.push(chunk.id.to_string());
            path.set_extension("bin");
            let mut file_handle = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)?;
            file_handle.write_all(&chunk.data)?;
            debug!(
                "Wrote chunk {} (Size: {}) to {}",
                chunk.id,
                da!(chunk.data.len()),
                path.display()
            );
            Ok(())
        } else {
            error!("Invalid session");
            Err(FileError::InvalidSession)
        }
    }

    fn end_transfer(
        &mut self,
        session: crate::storage::vfs::TransferSessionId,
    ) -> Result<(), FileError> {
        let _s = debug_span!("EndTransfer", session = session.to_string()).entered();
        if self.session_exists(&session) {
            self.sessions.remove(&session);
            debug!("Session {} removed", session);
            Ok(())
        } else {
            error!("Invalid session");
            Err(FileError::InvalidSession)
        }
    }

    fn abort_transfer(
        &mut self,
        _session: crate::storage::vfs::TransferSessionId,
    ) -> Result<(), FileError> {
        unimplemented!("SFS does not support this functionality.")
    }

    fn get_chunk(
        &self,
        key: crate::storage::vfs::FileKey,
        chunk_index: crabdrive_common::storage::ChunkIndex,
        chunk_size: crabdrive_common::data::DataAmount,
    ) -> Result<FileChunk, FileError> {
        let _s = debug_span!("GetChunk", key = key).entered();
        if self.exists(&key) {
            let mut path = self.storage_dir.clone();
            path.push(&key);
            path.push(chunk_index.to_string());
            path.set_extension("bin");
            let mut file_handle = OpenOptions::new().read(true).open(&path)?;
            debug!("Creating zeroed buffer");
            let mut bytes = BytesMut::zeroed(chunk_size.as_bytes() as usize);
            debug!(
                "Reading {} from {} into buffer",
                &chunk_size,
                &path.display()
            );
            file_handle.read_exact(&mut bytes)?;
            Ok(FileChunk {
                id: chunk_index,
                data: bytes.freeze(),
            })
        } else {
            Err(FileError::KeyNotFound)
        }
    }
}
