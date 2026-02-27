use std::path::PathBuf;
use std::sync::atomic::AtomicU8;

use bytes::Bytes;

// There is no way of using atomics and enums representations, so this is just a "lookup"
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FileTransferState {
    /// The file is currently idle and no operations are running on it
    Ready = 0,
    /// The file is currently being committed
    Comitting = 1,
    /// The transfer is aborting and the file is currently deleting
    Aborting = 2,
    /// Anything above 3 is the number of (concurrent) chunk writers
    Writing = 3,
}

impl TryFrom<u8> for FileTransferState {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FileTransferState::Ready),
            1 => Ok(FileTransferState::Comitting),
            2 => Ok(FileTransferState::Aborting),
            _ => Ok(FileTransferState::Writing),
        }
    }
}

/// Internal representation of transfers.
pub struct FileTransfer {
    /// The current state of the transfer. See [`FileTransferStatus`] for possible values. Never access this
    /// directly, instead use [`FileTransfer::get_state()`] and [`FileTransfer::set_state()`]!
    // There may be a race condition, where the gc is deleting a chunk, even though a chunk is currently getting
    // written. To prevent this, use a simple spin lock, which waits until the writer finishes.
    state: AtomicU8,
    /// Reference counter for checking if an upload has staled and is to be garbage collected.
    /// Every minute, a task goes over all active transfers, and increments `reference` by one. If
    /// the reference reaches 5 minutes, it is considered stale, if it reaches 10 minutes, the
    /// transfer is aborted.
    ///
    /// If a file receives a new chunk, the reference byte is reset to 0.
    pub reference: AtomicU8,
    /// The sharded base path in the (staging) directory, where chunks are stored inside
    pub path: PathBuf,
}

impl FileTransfer {
    pub fn new(path: PathBuf) -> Self {
        FileTransfer {
            state: AtomicU8::from(FileTransferState::Ready as u8),
            reference: AtomicU8::from(0),
            path,
        }
    }

    pub fn get_state(&self) -> FileTransferState {
        self
            .state
            .load(std::sync::atomic::Ordering::Relaxed)
            .try_into()
            .expect("also impossible")
    }

    pub fn try_set_state(&self, target: FileTransferState) -> Result<(), ()> {
        // The lockfree state machine is inspired by https://mara.nl/atomics/atomics.html#example-handle-overflow
        let mut current = self.state.load(std::sync::atomic::Ordering::Acquire);

        loop {
            let current_state = FileTransferState::try_from(current).unwrap();

            let next_val = match (current_state, target) {
                // if setting from Ready -> Anything: Succeed
                (FileTransferState::Ready, FileTransferState::Comitting) => 1,
                (FileTransferState::Ready, FileTransferState::Aborting) => 2,
                (FileTransferState::Ready, FileTransferState::Writing) => 3, // First writer
                // if setting from Writing -> Writing: Increase writers by one and succeed
                (FileTransferState::Writing, FileTransferState::Writing) => {
                    tracing::trace!("Incrementing writers by one");
                    if current == 255 {
                        tracing::warn!("Maximum concurrent chunk writers (252) exceeded");
                        return Err(());
                    }
                    current + 1
                }

                // if setting from Writing -> Ready: Decrease writers by one and succeed
                (FileTransferState::Writing, FileTransferState::Ready) => {
                    if current == 3 {
                        // Last writer checking out. State becomes Ready.
                        0
                    } else {
                        // Still other writers active. Decrement count.
                        current - 1
                    }
                }

                _ => return Err(()),
            };

            match self.state.compare_exchange_weak(
                current,
                next_val,
                // Needs to be consistent across tasks / threads
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::Acquire,
            ) {
                Ok(_) => return Ok(()),
                Err(actual) => current = actual,
            }
        }
    }

    pub async fn set_state(&self, target: FileTransferState) {
        while let Err(()) = self.try_set_state(target) {
            tokio::task::yield_now().await;
        }
    }
}

#[derive(Clone)]
pub struct CachedChunk {
    /// Information about when the chunk was cached. If for example [`C3::cache_ahead`] is set to 5
    /// and Chunk A is loaded, a task is spawned to load Chunk B - F (5 in advance). To prevent the
    /// task in B running from Chunk C - G (where 4 out 5 chunks are already chunked), this counter
    /// allows for periodically skipping loading chunks into cache.
    ///
    /// If this is set to `-1`, all chunks of this file have already been cached.
    pub cached_at: i8,
    pub bytes: Bytes,
}
