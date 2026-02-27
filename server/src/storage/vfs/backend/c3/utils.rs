use bytes::{Bytes, BytesMut};
use crabdrive_common::{storage::ChunkIndex, uuid::UUID};
use tracing::instrument;
use std::path::PathBuf;
use tokio::io::AsyncReadExt;
use uuid::Uuid;

use crate::storage::vfs::FileSystemError;

/// Generates a nested directory path by sharding a UUID.
///
/// The resulting path structure is:
/// `[base_path]/[chars 0-1]/[chars 2-3]/[chars 4-5]/[chars 6-31]`
///
// The idea is stolen from git :)
#[instrument(ret(level = "trace"))]
pub fn shard_path(id: UUID, base_path: &PathBuf) -> PathBuf {
    let mut buffer = Uuid::encode_buffer();
    let hex = id.get().simple().encode_lower(&mut buffer);
    // Reserve length of base path + 40 bytes. At least 36 are required for the UUID
    // (including path seperators), and another 5 (plus one seperator) are reserved
    // for the chunk name.
    let mut path = PathBuf::with_capacity(base_path.as_os_str().len() + 42);
    path.push(base_path);
    path.push(&hex[0..2]);
    path.push(&hex[2..4]);
    path.push(&hex[4..6]);
    path.push(&hex[6..]);
    path
}

pub fn shard_path_with_index(id: UUID, base_path: &PathBuf, index: ChunkIndex) -> PathBuf {
    let mut pathbuf = shard_path(id, base_path);
    push_index(&mut pathbuf, index);
    pathbuf
}

pub fn push_index(path: &mut PathBuf, index: ChunkIndex) {
    path.push(index.to_string());
    path.set_extension("dat");
}

pub async fn read_chunk(path: &PathBuf) -> Result<Bytes, FileSystemError> {
    let mut file = tokio::fs::File::open(path)
        .await
        .map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => FileSystemError::NotFound,
            _ => {
                tracing::info!("Failed to read path: {e}");
                e.into()
            },
        })?;

    let size = file.metadata().await?.len();
    assert!(size < crabdrive_common::da!(20 MiB).as_bytes());

    let mut buffer = BytesMut::with_capacity(size as usize);

    while buffer.len() < size as usize {
        let bytes_read = file.read_buf(&mut buffer).await.inspect_err(|e| {
            tracing::error!("Failed to fill buffer: {e}");
        })?;

        if bytes_read == 0 {
            tracing::error!("Unexpected EOF before chunk was fully read");
            break;
        }
    }

    tracing::warn!("Buffer: L {} C {}", buffer.len(), buffer.capacity());
    tracing::info!("Reading {} into buffer", crabdrive_common::da!(size));

    Ok(buffer.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use std::path::PathBuf;
    use test_case::test_case;

    #[test_case(
        "4de6ed4b-01af-4f0f-a76a-39d62258a3c0",
        "4d/e6/ed/4b01af4f0fa76a39d62258a3c0"
    )]
    #[test_case(
        "4b037ba3-8309-4bfd-9c54-e8d4c66bb260",
        "4b/03/7b/a383094bfd9c54e8d4c66bb260"
    )]
    #[test_case(
        "b2af44f2-9814-4b03-a45e-24d3dd5e7e01",
        "b2/af/44/f298144b03a45e24d3dd5e7e01"
    )]
    #[test_case(
        "58d0de47-8ea5-4d05-b742-0d3c30a6503e",
        "58/d0/de/478ea54d05b7420d3c30a6503e"
    )]
    #[test_case(
        "9f54e7c1-b61c-4322-8fd1-09848f22d59a",
        "9f/54/e7/c1b61c43228fd109848f22d59a"
    )]
    #[test_case(
        "45422a90-75d5-400e-ab8d-8991329492b4",
        "45/42/2a/9075d5400eab8d8991329492b4"
    )]
    #[test_case(
        "3355cce4-0b31-4a4c-a1bd-f88b67bf4b2f",
        "33/55/cc/e40b314a4ca1bdf88b67bf4b2f"
    )]
    fn test_shard_path(id: &str, expected: &str) {
        let id = UUID::parse_string(id).expect("Failed to parse UUID!");
        let base_path = PathBuf::from("/test/");
        let result = shard_path(id, &base_path).display().to_string();
        #[cfg(not(unix))] // Normalize strings for non-UNIX
        let result = result.replace("\\", "/");
        assert_eq!(result, "/test/".to_string() + expected);
    }
}
