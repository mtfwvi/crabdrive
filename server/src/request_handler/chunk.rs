use crate::http::AppState;
use crate::storage::vfs::FileChunk;
use crate::storage::vfs::model::{FileError, new_filekey};
use crate::user::persistence::model::user_entity::UserEntity;
use axum::Json;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use crabdrive_common::data::DataAmount;
use crabdrive_common::storage::{ChunkIndex, NodeId, RevisionId};

pub async fn post_chunk(
    current_user: UserEntity,
    State(state): State<AppState>,
    Path((node_id, revision_id, chunk_index)): Path<(NodeId, RevisionId, ChunkIndex)>,
    chunk: axum::body::Bytes,
) -> (StatusCode, Json<()>) {
    let file_chunk = FileChunk {
        index: chunk_index,
        data: chunk,
    };

    let node_entity = state.node_repository.get_node(node_id).expect("db error");
    let revision_entity = state
        .revision_repository
        .get_revision(revision_id)
        .expect("db error");
    if revision_entity.is_none() || node_entity.is_none() {
        return (StatusCode::NOT_FOUND, Json(()));
    }

    let (revision_entity, node_entity) = (revision_entity.unwrap(), node_entity.unwrap());

    if node_entity.id != revision_entity.file_id {
        return (StatusCode::NOT_FOUND, Json(()));
    }

    if !state
        .node_repository
        .has_access(node_entity.id, current_user.id)
        .expect("db error")
    {
        return (StatusCode::NOT_FOUND, Json(()));
    }

    if revision_entity.chunk_count < chunk_index || chunk_index <= 0 {
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
    current_user: UserEntity,
    State(state): State<AppState>,
    Path((node_id, revision_id, chunk_index)): Path<(NodeId, RevisionId, ChunkIndex)>,
) -> Response<Body> {
    let node_entity = state.node_repository.get_node(node_id).expect("db error");
    let revision_entity = state
        .revision_repository
        .get_revision(revision_id)
        .expect("db error");
    if revision_entity.is_none() || node_entity.is_none() {
        return StatusCode::NOT_FOUND.into_response();
    }

    let (revision_entity, node_entity) = (revision_entity.unwrap(), node_entity.unwrap());

    if node_entity.id != revision_entity.file_id {
        return StatusCode::NOT_FOUND.into_response();
    }

    if !state
        .node_repository
        .has_access(node_entity.id, current_user.id)
        .expect("db error")
    {
        return StatusCode::NOT_FOUND.into_response();
    }

    let file_key = new_filekey(node_id, revision_id);

    let chunk_size = DataAmount::zero(); // Inferred from actual file for now, may be set manually later

    let result = state
        .vfs
        .read()
        .expect("someone panicked while holding vfs?")
        .get_chunk(file_key, chunk_index, chunk_size);

    if let Ok(data) = result {
        return (StatusCode::OK, data.data).into_response();
    }

    match result.err().unwrap() {
        FileError::InvalidLength => StatusCode::BAD_REQUEST.into_response(),
        FileError::InvalidSession => StatusCode::BAD_REQUEST.into_response(),
        FileError::Io(_err) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        FileError::KeyNotFound => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
