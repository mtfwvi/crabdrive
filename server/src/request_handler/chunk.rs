use crate::http::AppState;
use crate::storage::vfs::model::{new_filekey, FileError};
use crate::storage::vfs::FileChunk;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use crabdrive_common::data::DataAmount;
use crabdrive_common::storage::{ChunkIndex, NodeId, RevisionId};
use crabdrive_common::uuid::UUID;

pub async fn post_chunk(
    State(state): State<AppState>,
    Path((node_id, revision_id, chunk_index)): Path<(NodeId, RevisionId, ChunkIndex)>,
    chunk: axum::body::Bytes,
) -> (StatusCode, Json<()>) {
    let file_chunk = FileChunk {
        index: chunk_index,
        data: chunk,
    };

    let file_key = new_filekey(node_id, revision_id);
    let transfer_session_id = UUID::from_string(&file_key);
    let result = state
        .vfs
        .read()
        .unwrap()
        .write_chunk(&transfer_session_id, file_chunk);

    match result {
        Ok(_) => (StatusCode::CREATED, Json(())),
        Err(FileError::InvalidLength) => (StatusCode::BAD_REQUEST, Json(())),
        Err(FileError::InvalidSession) => (StatusCode::BAD_REQUEST, Json(())),
        Err(FileError::Io(_err)) => (StatusCode::INTERNAL_SERVER_ERROR, Json(())),
        Err(FileError::KeyNotFound) => (StatusCode::NOT_FOUND, Json(())),
    }
}

pub async fn get_chunk(
    State(state): State<AppState>,
    Path((node_id, revision_id, chunk_index)): Path<(NodeId, RevisionId, ChunkIndex)>,
) -> (StatusCode, Vec<u8>) {
    let file_key = new_filekey(node_id, revision_id);

    let chunk_size = DataAmount::zero(); // Inferred from actual file for now, may be set manually later

    let result = state
        .vfs
        .read()
        .expect("someone panicked while holding vfs?")
        .get_chunk(file_key, chunk_index, chunk_size);

    if let Ok(data) = result {
        return (StatusCode::CREATED, data.data.to_vec());
    }

    match result.err().unwrap() {
        FileError::InvalidLength => (StatusCode::BAD_REQUEST, vec![]),
        FileError::InvalidSession => (StatusCode::BAD_REQUEST, vec![]),
        FileError::Io(_err) => (StatusCode::INTERNAL_SERVER_ERROR, vec![]),
        FileError::KeyNotFound => (StatusCode::NOT_FOUND, vec![]),
    }
}
