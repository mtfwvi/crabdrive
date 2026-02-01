use crate::http::AppState;
use crate::storage::vfs::FileChunk;
use crate::storage::vfs::model::{FileError, new_filekey};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use crabdrive_common::data::DataAmount;
use crabdrive_common::storage::{ChunkIndex, NodeId, RevisionId};

pub async fn post_chunk(
    State(state): State<AppState>,
    Path((node_id, revision_id, chunk_index)): Path<(NodeId, RevisionId, ChunkIndex)>,
    chunk: axum::body::Bytes,
) -> (StatusCode, Json<()>) {
    let file_chunk = FileChunk {
        index: chunk_index,
        data: chunk,
    };

    let revision = state
        .revision_repository
        .get_revision(revision_id)
        .expect("db error");

    if revision.is_none() {
        return (StatusCode::NOT_FOUND, Json(()));
    }

    let revision = revision.unwrap();

    if revision.chunk_count < chunk_index || chunk_index <= 0 {
        return (StatusCode::BAD_REQUEST, Json(()));
    }

    let file_key = new_filekey(node_id, revision_id);

    if state
        .vfs
        .read()
        .unwrap()
        .chunk_exists(&file_key, chunk_index)
    {
        return (StatusCode::BAD_REQUEST, Json(()));
    }

    let result = state.vfs.read().unwrap().write_chunk(&file_key, file_chunk);

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
