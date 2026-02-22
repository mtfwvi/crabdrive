use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use crabdrive_common::storage::{ChunkIndex, RevisionId};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileSystemError {
    #[error("File not found.")]
    NotFound,

    #[error("File already committed.")]
    AlreadyCommitted,

    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

impl IntoResponse for FileSystemError {
    fn into_response(self) -> Response {
        match self {
            FileSystemError::NotFound => (StatusCode::NOT_FOUND, Json(())),
            FileSystemError::AlreadyCommitted => (StatusCode::BAD_REQUEST, Json(())),
            FileSystemError::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(())),
        }
        .into_response()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum FileStatus {
    /// Reserved for future usage
    Stale,
    /// File is currently being uploaded
    Staged,
    /// File is stored on permanent, persistent storage
    Persisted,
    /// The file has not been found (neither Staged nor Persisted)
    NotFound,
}

/// Internal storage key for a file
pub(crate) type FileKey = RevisionId;

pub(crate) struct FileChunk {
    pub index: ChunkIndex,
    /// The chunk contents.
    /// The size of the file chunk can be accessed using `FileChunk::data::len()`. It is
    /// usually 16MB, however it may be smaller if this is the last (or only) chunk.
    pub data: Bytes,
}
