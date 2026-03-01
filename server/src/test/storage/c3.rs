use crate::storage::vfs::backend::c3::model::{FileTransfer, FileTransferState};
use crate::storage::vfs::backend::c3::*;
use crate::storage::vfs::{FileChunk, FileRepository, FileStatus, FileSystemError};
use crate::test::utils::TestContext;

use bytes::Bytes;
use crabdrive_common::uuid::UUID;
use std::path::PathBuf;

#[tokio::test]
async fn test_file_transfer_state_machine() {
    let transfer = FileTransfer::new(PathBuf::from("/tmp/dummy"));

    assert_eq!(transfer.get_state(), FileTransferState::Ready);

    assert!(transfer.try_set_state(FileTransferState::Writing).is_ok());
    assert_eq!(transfer.get_state(), FileTransferState::Writing);

    assert!(transfer.try_set_state(FileTransferState::Writing).is_ok());
    assert_eq!(transfer.get_state(), FileTransferState::Writing);

    assert!(transfer.try_set_state(FileTransferState::Ready).is_ok());
    assert_eq!(transfer.get_state(), FileTransferState::Writing);

    assert!(transfer.try_set_state(FileTransferState::Ready).is_ok());
    assert_eq!(transfer.get_state(), FileTransferState::Ready);

    assert!(transfer.try_set_state(FileTransferState::Comitting).is_ok());
    assert_eq!(transfer.get_state(), FileTransferState::Comitting);

    assert!(transfer.try_set_state(FileTransferState::Writing).is_err());
}

#[tokio::test]
async fn test_c3_lifecycle() {
    let ctx = TestContext::new(0).await;
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");

    let mut c3 = C3::new(temp_dir.path().to_path_buf(), ctx.state.db_pool, 10, 5).await;

    let key = UUID::random();

    assert_eq!(c3.file_status(&key).await, FileStatus::NotFound);

    c3.create_file(&key).await.expect("Failed to create file");
    assert_eq!(c3.file_status(&key).await, FileStatus::Staged);

    let data = Bytes::from("crabdrive_rocks");
    let chunk = FileChunk {
        index: 0,
        data: data.clone(),
    };
    c3.write_chunk(&key, chunk)
        .await
        .expect("Failed to write chunk");

    assert!(c3.chunk_exists(&key, 0).await);
    assert!(!c3.chunk_exists(&key, 1).await);

    c3.commit_file(&key).await.expect("Failed to commit file");
    assert_eq!(c3.file_status(&key).await, FileStatus::Persisted);

    assert!(!c3.chunk_exists(&key, 0).await);

    let read_chunk = c3.read_chunk(&key, 0).await.expect("Failed to read chunk");
    assert_eq!(read_chunk.data, data);

    c3.delete_file(&key).await.expect("Failed to delete file");
    assert_eq!(c3.file_status(&key).await, FileStatus::NotFound);
}

#[tokio::test]
async fn test_c3_abort_transfer() {
    let ctx = TestContext::new(0).await;
    let temp_dir = tempfile::tempdir().unwrap();

    let mut c3 = C3::new(
        temp_dir.path().to_path_buf(),
        ctx.state.db_pool.clone(),
        10,
        5,
    )
    .await;

    let key = UUID::random();
    c3.create_file(&key).await.unwrap();

    let chunk = FileChunk {
        index: 0,
        data: Bytes::from("abort"),
    };
    c3.write_chunk(&key, chunk).await.unwrap();

    assert_eq!(c3.file_status(&key).await, FileStatus::Staged);
    assert!(c3.chunk_exists(&key, 0).await);

    c3.abort(&key).await.expect("Failed to abort transfer");

    assert_eq!(c3.file_status(&key).await, FileStatus::NotFound);
    assert!(!c3.chunk_exists(&key, 0).await);
}

#[tokio::test]
async fn test_c3_already_exists() {
    let ctx = TestContext::new(0).await;
    let temp_dir = tempfile::tempdir().unwrap();
    let mut c3 = C3::new(
        temp_dir.path().to_path_buf(),
        ctx.state.db_pool.clone(),
        10,
        5,
    )
    .await;

    let key = UUID::random();
    c3.create_file(&key).await.unwrap();

    let err = c3.create_file(&key).await.unwrap_err();
    assert!(matches!(err, FileSystemError::AlreadyExists));
}

#[tokio::test]
async fn test_c3_delete_staged_fails() {
    let ctx = TestContext::new(0).await;
    let temp_dir = tempfile::tempdir().unwrap();
    let mut c3 = C3::new(
        temp_dir.path().to_path_buf(),
        ctx.state.db_pool.clone(),
        10,
        5,
    )
    .await;

    let key = UUID::random();
    c3.create_file(&key).await.unwrap();

    let err = c3.delete_file(&key).await.unwrap_err();
    assert!(matches!(err, FileSystemError::NotFound));
}

#[tokio::test]
async fn test_c3_abort_persisted_fails() {
    let ctx = TestContext::new(0).await;
    let temp_dir = tempfile::tempdir().unwrap();
    let mut c3 = C3::new(
        temp_dir.path().to_path_buf(),
        ctx.state.db_pool.clone(),
        10,
        5,
    )
    .await;

    let key = UUID::random();
    c3.create_file(&key).await.unwrap();
    c3.commit_file(&key).await.unwrap();

    let err = c3.abort(&key).await.unwrap_err();
    assert!(matches!(err, FileSystemError::NotFound));
}
