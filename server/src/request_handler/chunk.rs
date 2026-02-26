use crate::http::AppState;
use crate::storage::vfs::FileChunk;
use crate::storage::vfs::model::FileSystemError;
use crate::user::persistence::model::user_entity::UserEntity;
use axum::Json;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use crabdrive_common::data::{DataAmount, DataUnit};
use crabdrive_common::storage::{ChunkIndex, NodeId, RevisionId};
use std::ops::Add;

pub async fn post_chunk(
    current_user: UserEntity,
    State(state): State<AppState>,
    Path((node_id, revision_id, chunk_index)): Path<(NodeId, RevisionId, ChunkIndex)>,
    chunk: axum::body::Bytes,
) -> (StatusCode, Json<()>) {
    let size = chunk.len() as f64;

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

    let Some(mut owning_user) = state.user_repository.get_user(node_entity.owner_id).expect("db error") else {
        panic!("db constraints not respected");
    };

    let owning_user_new_storage_used = owning_user.storage_used
        .add(DataAmount::new(size, DataUnit::Byte));

    if owning_user_new_storage_used > owning_user.storage_limit {
        return (StatusCode::INSUFFICIENT_STORAGE, Json(()));
    }


    if node_entity.owner_id != current_user.id || node_entity.id != revision_entity.file_id {
        return (StatusCode::NOT_FOUND, Json(()));
    }

    if revision_entity.chunk_count < chunk_index || chunk_index <= 0 {
        return (StatusCode::BAD_REQUEST, Json(()));
    }

    if state
        .vfs
        .read()
        .await
        .chunk_exists(&revision_id, chunk_index)
        .await
    {
        return (StatusCode::BAD_REQUEST, Json(()));
    }

    let result = state
        .vfs
        .write()
        .await
        .write_chunk(&revision_id, file_chunk)
        .await;

    match result {
        Ok(_) => {
            owning_user.storage_used = owning_user_new_storage_used;
            state
                .user_repository
                .update_user(owning_user)
                .expect("db error");

            (StatusCode::CREATED, Json(()))
        }
        Err(FileSystemError::AlreadyCommitted) => (StatusCode::BAD_REQUEST, Json(())),
        Err(FileSystemError::NotFound) => (StatusCode::NOT_FOUND, Json(())),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(())),
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

    if node_entity.owner_id != current_user.id || node_entity.id != revision_entity.file_id {
        return StatusCode::NOT_FOUND.into_response();
    }

    let result = state
        .vfs
        .read()
        .await
        .read_chunk(&revision_id, chunk_index)
        .await;

    if let Ok(data) = result {
        return (StatusCode::OK, data.data).into_response();
    }

    match result.err().unwrap() {
        FileSystemError::AlreadyCommitted => StatusCode::BAD_REQUEST.into_response(),
        FileSystemError::NotFound => StatusCode::NOT_FOUND.into_response(),
        _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
