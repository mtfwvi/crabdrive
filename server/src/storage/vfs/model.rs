use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use crabdrive_common::storage::{ChunkIndex, RevisionId};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum FileSystemError {
    #[error("File not found.")]
    NotFound,

    #[error("File already exists.")]
    AlreadyExists,

    #[error("IO Error: {1} ({0})")]
    Io(std::io::ErrorKind, String),
}

impl From<std::io::Error> for FileSystemError {
    fn from(value: std::io::Error) -> Self {
        FileSystemError::Io(value.kind(), value.to_string())
    }
}

impl From<std::sync::Arc<FileSystemError>> for FileSystemError {
    fn from(arc_err: std::sync::Arc<FileSystemError>) -> Self {
        (*arc_err).clone()
    }
}

impl IntoResponse for FileSystemError {
    fn into_response(self) -> Response {
        match self {
            FileSystemError::NotFound => (StatusCode::NOT_FOUND, Json(())),
            FileSystemError::AlreadyExists => (StatusCode::CONFLICT, Json(())),
            FileSystemError::Io(_, _) => (StatusCode::INTERNAL_SERVER_ERROR, Json(())),
        }
        .into_response()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum FileStatus {
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
pub type FileKey = RevisionId;

pub struct FileChunk {
    pub index: ChunkIndex,
    /// The chunk contents.
    /// The size of the file chunk can be accessed using `FileChunk::data::len()`. It is
    /// usually 16MB, however it may be smaller if this is the last (or only) chunk.
    pub data: Bytes,
}
