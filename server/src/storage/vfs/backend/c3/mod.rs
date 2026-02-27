mod model;
#[cfg(test)]
mod test;
mod utils;

use crate::db::connection::DbPool;
use crate::db::operations::revision::get_all_uncommitted_revisions;
use crate::storage::vfs::{FileChunk, FileKey, FileRepository, FileStatus, FileSystemError};
use model::{CachedChunk, FileTransfer, FileTransferState};

use crabdrive_common::storage::ChunkIndex;
use crabdrive_common::uuid::UUID;

use std::sync::Arc;
use std::{path::PathBuf, time::Duration};
use tokio::fs;

use async_trait::async_trait;
use dashmap::DashMap;
use moka::future::Cache;
use tracing::instrument;

/// Crabdrive Storage Service (name definetly not stolen from Amazon S3)
#[derive(Clone)]
pub struct C3 {
    /// The base path for the staging directory
    staging_path: PathBuf,
    /// The base path for the persitent directory
    persistent_path: PathBuf,
    /// List of all active transfers
    transfers: Arc<DashMap<UUID, FileTransfer>>,
    cache: Arc<Cache<(UUID, ChunkIndex), CachedChunk>>,
    // Uses the database to track active transfers
    db_pool: DbPool,
    /// Configuration option for caching behavior.
    cache_ahead: u8,
}

impl C3 {
    pub async fn new(storage_directory: PathBuf, db_pool: DbPool) -> Self {
        if !storage_directory.exists() {
            panic!("Storage directory does not exist!");
        }

        let mut staging_path = storage_directory.clone();
        staging_path.push("stage");

        let mut persistent_path = storage_directory.clone();
        persistent_path.push("pers");

        tokio::fs::create_dir(&persistent_path)
            .await
            .expect("Failed to create persistent directory");
        tokio::fs::create_dir(&staging_path)
            .await
            .expect("Failed to create staging directory");

        tracing::info!("C3 will store files inside {}", storage_directory.display());

        let transfers = DashMap::new();

        tracing::info!("Checking for unfinished transfers");
        // If some transfers are lost (for example during restart), recreate them
        let open_transfers =
            get_all_uncommitted_revisions(&mut db_pool.get().expect("Failed to get DB Connection"))
                .inspect_err(|e| tracing::warn!("Failed to recreate open file transfers ({e})"))
                .ok();

        if let Some(open_transfers) = open_transfers {
            tracing::info!("Re-opened {} open transfers", open_transfers.len());
            for id in open_transfers {
                let path = utils::shard_path(id, &staging_path);
                let transfer = FileTransfer::new(path);
                transfers.insert(id, transfer);
            }
        }

        let cache: Cache<(UUID, ChunkIndex), CachedChunk> = Cache::builder()
            .max_capacity(20) // 20 * 17MiB = ca. 350 MB
            // In the current cache strategy, chunks are cached sequentially. This means, if client requests CHUNK A,
            // C3 will start to load CHUNK B, CHUNK C, and so on into the cache. If a chunk is not accessed during
            // these 60 seconds, the client has either a horrible bandwith or simply abandoned the download.
            .time_to_idle(Duration::from_secs(60))
            .support_invalidation_closures()
            .build();

        let c3 = C3 {
            staging_path,
            persistent_path,
            transfers: Arc::new(transfers),
            cache: Arc::new(cache),
            db_pool,
            cache_ahead: 4,
        };

        c3.spawn_gc();
        c3
    }

    /// Garbage-Collects all staled transfers (received no uploads in the last 10 minutes) and
    /// updates the reference counter on each transfer.
    fn spawn_gc(&self) {
        let mut this = self.clone(); // Welcome back Java :D

        tokio::spawn(async move {
            // Run every minute
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                tracing::info!("Checking for staled transfers");

                // Sadly, cannot iterate over the map while also deleting files (as it locks the map too long)
                let mut stale_keys = Vec::new();
                for entry in this.transfers.iter() {
                    let transfer = entry.value();
                    if transfer.get_state() == FileTransferState::Ready {
                        let last = transfer
                            .reference
                            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if last > 10 {
                            stale_keys.push(*entry.key());
                        }
                    }
                }

                for key in stale_keys {
                    tracing::info!("Aborting transfer for {key} (staled)");
                    this.abort(&key).await.ok();
                }
            }
        });
    }

    fn spawn_prefetch(&self, key: UUID, original_index: i64) {
        let cache = Arc::clone(&self.cache);
        let cache_ahead = self.cache_ahead;
        let persistent_path = self.persistent_path.clone();

        tokio::spawn(async move {
            let mut byte_chunks = Vec::with_capacity(cache_ahead as usize);

            for advance in 0..cache_ahead {
                let current_index = original_index + advance as i64;
                let path = utils::shard_path_with_index(key, &persistent_path, current_index);

                match utils::read_chunk(&path).await {
                    Ok(c) => byte_chunks.push(c),
                    Err(FileSystemError::NotFound) => break,
                    Err(e) => {
                        tracing::error!(%key, index = current_index, "Prefetch: {e}");
                        break;
                    }
                }
            }

            tracing::trace!(
                "Prefetched {} chunks (seeked {})",
                byte_chunks.len(),
                cache_ahead
            );

            let is_eof = byte_chunks.len() < cache_ahead as usize;

            for (advance, bytes) in byte_chunks.into_iter().enumerate() {
                let current_index = original_index + advance as i64;

                let cached_at = if is_eof {
                    // If we reached EOF (no more chunks), indicate this to future tasks attempting to prefetch chunks
                    -1
                } else {
                    (cache_ahead as i8) - (advance as i8)
                };

                cache
                    .insert((key, current_index), CachedChunk { cached_at, bytes })
                    .await;
            }
        });
    }
}

#[async_trait]
impl FileRepository for C3 {
    #[instrument(skip(self))]
    async fn chunk_exists(&self, key: &FileKey, index: ChunkIndex) -> bool {
        fs::try_exists(utils::shard_path_with_index(
            *key,
            &self.staging_path,
            index,
        ))
        .await
        .unwrap_or(false)
    }

    #[instrument(skip(self))]
    async fn file_status(&self, key: &FileKey) -> FileStatus {
        if let Some(transfer) = self.transfers.get(key) {
            if transfer
                .reference
                .load(std::sync::atomic::Ordering::Relaxed)
                > 5
            {
                // The file received no more chunks in the last 5 minutes
                FileStatus::Stale
            } else {
                FileStatus::Staged
            }
        } else {
            let persisted_path = utils::shard_path(*key, &self.persistent_path);
            if fs::try_exists(&persisted_path).await.unwrap_or(false) {
                FileStatus::Persisted
            } else {
                FileStatus::NotFound
            }
        }
    }

    #[instrument(skip(self), err)]
    async fn create_file(&mut self, key: &FileKey) -> Result<(), FileSystemError> {
        let FileStatus::NotFound = self.file_status(key).await else {
            tracing::warn!("Revision already stored in VFS!");
            return Err(FileSystemError::AlreadyExists);
        };

        let path = utils::shard_path(*key, &self.staging_path);
        fs::create_dir_all(&path).await?;

        tracing::debug!("File chunks will be staged in {}", path.display());

        let transfer = FileTransfer::new(path);
        self.transfers.insert(*key, transfer);

        Ok(())
    }

    #[instrument(skip(self, contents), err)]
    async fn write_chunk(
        &mut self,
        key: &FileKey,
        contents: FileChunk,
    ) -> Result<(), FileSystemError> {
        let transfer = self.transfers.get(key).ok_or(FileSystemError::NotFound)?;
        transfer.set_state(FileTransferState::Writing).await;

        let mut path = transfer.path.clone();
        utils::push_index(&mut path, contents.index);

        transfer
            .reference
            .store(0, std::sync::atomic::Ordering::SeqCst);

        let result = fs::write(path, contents.data).await;

        transfer.try_set_state(FileTransferState::Ready).ok();
        result.map_err(|e| e.into())
    }

    #[instrument(skip(self), err)]
    async fn commit_file(&mut self, key: &FileKey) -> Result<(), FileSystemError> {
        let transfer = self.transfers.get(key).ok_or(FileSystemError::NotFound)?;
        transfer.set_state(FileTransferState::Comitting).await;

        transfer
            .reference
            .store(0, std::sync::atomic::Ordering::Relaxed);

        let persistent_path = utils::shard_path(*key, &self.persistent_path);

        fs::create_dir_all(&persistent_path).await?;

        fs::rename(&transfer.path, &persistent_path)
            .await
            .inspect_err(|e| {
                tracing::error!("Failed to persist file: {e}");
                tracing::info!(
                    "Operation that failed was rename {} to {}",
                    transfer.path.display(),
                    persistent_path.display()
                );
                tracing::info!(
                    "Folder {} exists: {}",
                    transfer.path.display(),
                    transfer.path.exists()
                );
                transfer.try_set_state(FileTransferState::Ready).ok(); // Reset state if failed
            })?;

        tracing::debug!(
            "File chunks are now persisted in: {}",
            persistent_path.display()
        );

        drop(transfer);
        self.transfers.remove(key);

        Ok(())
    }

    #[instrument(skip(self), err)]
    async fn abort(&mut self, key: &FileKey) -> Result<(), FileSystemError> {
        if !matches!(
            self.file_status(key).await,
            FileStatus::Staged | FileStatus::Stale
        ) {
            tracing::warn!("Attempted to abort non-existing transfer!");
            return Err(FileSystemError::NotFound);
        };

        let transfer = self.transfers.get(key).ok_or(FileSystemError::NotFound)?;
        transfer.set_state(FileTransferState::Aborting).await;
        transfer
            .reference
            .store(0, std::sync::atomic::Ordering::Relaxed);

        fs::remove_dir_all(&transfer.path).await.inspect_err(|_| {
            transfer.try_set_state(FileTransferState::Ready).ok(); // Reset state if failed. This should not fail.
        })?;

        drop(transfer);
        self.transfers.remove(key);

        Ok(())
    }

    #[instrument(skip(self), err)]
    async fn delete_file(&mut self, key: &FileKey) -> Result<(), FileSystemError> {
        if self.file_status(key).await != FileStatus::Persisted {
            tracing::warn!("Attempted to delete non-existing file!");
            return Err(FileSystemError::NotFound);
        };

        let original_path = utils::shard_path(*key, &self.persistent_path);
        let mut safe_path = original_path.clone();
        // Rename the dictionary, so a call to read() directly returns Not Found
        if let Some(name) = original_path.file_name() {
            let mut last_shard = name.to_os_string();
            last_shard.push("-i");
            safe_path.set_file_name(last_shard);
        }

        // Try 3 times to rename the folder and (if not successful) finally give up
        let mut c = 0;
        while let Err(e) = fs::rename(&original_path, &safe_path).await {
            if c > 2 {
                return Err(e.into());
            }
            c += 1;
            tokio::task::yield_now().await;
        }

        fs::remove_dir_all(&safe_path).await?;

        // Invalidate all cache entries
        let key = *key;
        self.cache
            .invalidate_entries_if(move |(cached_id, _), _| cached_id == &key)
            .ok();

        Ok(())
    }

    #[instrument(skip(self), err)]
    async fn read_chunk(
        &self,
        key: &FileKey,
        index: ChunkIndex,
    ) -> Result<FileChunk, FileSystemError> {
        let cache_key = (*key, index);

        // Check for a cache hit
        let bytes = self
            .cache
            .try_get_with(cache_key, async {
                tracing::debug!("Cache miss");
                // If the chunk was not cached, read it from disk
                let path = utils::shard_path_with_index(*key, &self.persistent_path, index);
                let bytes = utils::read_chunk(&path).await?;
                Ok(CachedChunk {
                    cached_at: 0,
                    bytes,
                })
            })
            .await
            .inspect_err(|e| tracing::error!("An error occurred while reading the bytes: {e}"))?;

        if bytes.cached_at < 1 && bytes.cached_at != -1 {
            // If the next chunks are not cached & the following chunks are not cached
            self.spawn_prefetch(*key, index);
        }

        Ok(FileChunk {
            index,
            data: bytes.bytes,
        })
    }
}
