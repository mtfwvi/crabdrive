use crate::http::AppState;
use crate::request_handler::node::{entity_to_encrypted_node, entity_to_file_revision};
use crate::storage::node::persistence::model::node_entity::NodeEntity;
use crate::storage::vfs::model::new_filekey;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use chrono::Utc;
use crabdrive_common::iv::IV;
use crabdrive_common::payloads::node::request::file::{
    PostCreateFileRequest, PostUpdateFileRequest,
};
use crabdrive_common::payloads::node::response::file::{
    GetVersionsResponse, PostCommitFileResponse, PostCreateFileResponse, PostUpdateFileResponse,
};
use crabdrive_common::storage::{FileRevision, NodeType};
use crabdrive_common::storage::{NodeId, RevisionId};
use crabdrive_common::uuid::UUID;

//TODO remove this
pub fn get_example_revision_info() -> FileRevision {
    FileRevision {
        id: UUID::random(),
        upload_ended_on: None,
        upload_started_on: Default::default(),
        iv: IV::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        chunk_count: 0,
    }
}

pub async fn post_create_file(
    State(state): State<AppState>,
    Path(parent_id): Path<NodeId>,
    Json(payload): Json<PostCreateFileRequest>,
) -> (StatusCode, Json<PostCreateFileResponse>) {
    let parent_node = state.node_repository.get_node(parent_id).expect("db error");

    if parent_node.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(PostCreateFileResponse::NotFound),
        );
    }

    let parent_node = parent_node.unwrap();

    // a file cannot have children
    if parent_node.node_type != NodeType::Folder {
        return (
            StatusCode::BAD_REQUEST,
            Json(PostCreateFileResponse::BadRequest),
        );
    }

    //TODO this is not thread safe
    if parent_node.metadata_change_counter != payload.parent_metadata_version {
        return (StatusCode::CONFLICT, Json(PostCreateFileResponse::Conflict));
    }

    //update the parent
    state
        .node_repository
        .update_node(&NodeEntity {
            metadata: payload.parent_metadata,
            metadata_change_counter: 0,
            ..parent_node
        })
        .expect("db error");

    //create the node
    let node = state
        .node_repository
        .create_node(
            Some(parent_id),
            payload.node_metadata,
            UUID::nil(),
            NodeType::File,
            payload.node_id,
        )
        .expect("db error");

    let revision = state
        .revision_repository
        .create_revision(
            node.id,
            Utc::now().naive_utc(),
            payload.file_iv,
            payload.chunk_count,
        )
        .expect("db error");

    let node_with_revision = NodeEntity {
        current_revision: Some(revision.id),
        ..node
    };

    state
        .node_repository
        .update_node(&node_with_revision)
        .expect("db error");

    let response_node =
        entity_to_encrypted_node(node_with_revision.clone(), &state).expect("db error");
    (
        StatusCode::CREATED,
        Json(PostCreateFileResponse::Created(response_node)),
    )
}

pub async fn post_update_file(
    State(state): State<AppState>,
    Path(file_id): Path<NodeId>,
    Json(payload): Json<PostUpdateFileRequest>,
) -> (StatusCode, Json<PostUpdateFileResponse>) {
    //(StatusCode::NOT_FOUND, Json(PostUpdateFileResponse::NotFound))
    //(StatusCode::BAD_REQUEST, Json(PostUpdateFileResponse::BadRequest))

    let node_entity = state.node_repository.get_node(file_id).expect("db error");

    if node_entity.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(PostUpdateFileResponse::NotFound),
        );
    }

    let node_entity = node_entity.unwrap();

    if node_entity.node_type != NodeType::Folder {
        return (
            StatusCode::BAD_REQUEST,
            Json(PostUpdateFileResponse::BadRequest),
        );
    }

    let revision = state
        .revision_repository
        .create_revision(
            file_id,
            Utc::now().naive_utc(),
            payload.file_iv,
            payload.chunk_count,
        )
        .expect("db error");

    (
        StatusCode::OK,
        Json(PostUpdateFileResponse::Ok(entity_to_file_revision(
            revision,
        ))),
    )
}

pub async fn post_commit_file(
    State(state): State<AppState>,
    Path((file_id, revision_id)): Path<(NodeId, RevisionId)>,
) -> (StatusCode, Json<PostCommitFileResponse>) {
    let revision = state
        .revision_repository
        .get_revision(revision_id)
        .expect("db error");

    let node_entity = state.node_repository.get_node(file_id).expect("db error");
    if revision.is_none() || node_entity.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(PostCommitFileResponse::NotFound),
        );
    }

    let (mut revision, mut node_entity) = (revision.unwrap(), node_entity.unwrap());

    let file_key = new_filekey(file_id, revision.id);

    let mut missing_chunks = vec![];
    for i in 1..revision.chunk_count {
        if !state.vfs.read().unwrap().exists(&file_key) {
            missing_chunks.push(i);
        }
    }
    if !missing_chunks.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(PostCommitFileResponse::BadRequest(missing_chunks)),
        );
    }

    revision.upload_ended_on = Some(Utc::now().naive_utc());

    node_entity.current_revision = Some(revision.id);

    state
        .revision_repository
        .update_revision(revision)
        .expect("db error");

    state
        .node_repository
        .update_node(&node_entity)
        .expect("db error");

    let node = entity_to_encrypted_node(node_entity, &state).expect("db error");

    (StatusCode::OK, Json(PostCommitFileResponse::Ok(node)))
}

pub async fn get_file_versions(
    State(state): State<AppState>,
    Path(file_id): Path<NodeId>,
) -> (StatusCode, Json<GetVersionsResponse>) {
    let node_entity = state.node_repository.get_node(file_id).expect("db error");

    if node_entity.is_none() {
        return (StatusCode::NOT_FOUND, Json(GetVersionsResponse::NotFound));
    }

    let version_entities = state
        .revision_repository
        .get_all_revisions_by_node(file_id)
        .expect("db error");

    let versions = version_entities
        .iter()
        .map(|&entity| entity_to_file_revision(entity))
        .collect();

    (StatusCode::OK, Json(GetVersionsResponse::Ok(versions)))
}
