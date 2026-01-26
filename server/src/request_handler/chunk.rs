use crate::http::AppState;
use crate::storage::vfs::model::FileError;
use crate::storage::vfs::{FileChunk, FileKey, TransferSessionId};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use crabdrive_common::data::DataAmount;
use crabdrive_common::storage::{NodeId, RevisionId};

pub async fn post_chunk(
    State(state): State<AppState>,
    Path((_node_id, _revision_id, chunk_index)): Path<(NodeId, RevisionId, u64)>,
    chunk: axum::body::Bytes,
) -> (StatusCode, Json<()>) {
    let file_chunk = FileChunk {
        index: chunk_index,
        data: chunk,
    };
    let transfer_session_id = TransferSessionId::nil(); // TODO: Generate from _node_id and _revision_id

    let result = state.vfs.write_chunk(&transfer_session_id, file_chunk);

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
    Path((_node_id, _revision_id, chunk_index)): Path<(NodeId, RevisionId, u64)>,
) -> (StatusCode, Vec<u8>) {
    let file_key = FileKey::new(); // TODO: Generate from _node_id and _revision_id

    let chunk_size = DataAmount::zero(); // Inferred from actual file for now, may be set manually later

    let result = state.vfs.get_chunk(file_key, chunk_index, chunk_size);

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
