use crate::http::AppState;
use crate::request_handler::node::entity_to_encrypted_node;
use crate::storage::node::persistence::model::node_entity::NodeEntity;
use crate::user::persistence::model::user_entity::UserEntity;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use crabdrive_common::payloads::node::request::folder::PostCreateFolderRequest;
use crabdrive_common::payloads::node::response::folder::PostCreateFolderResponse;
use crabdrive_common::storage::{NodeId, NodeType};

pub async fn post_create_folder(
    current_user: UserEntity,
    State(state): State<AppState>,
    Path(parent_id): Path<NodeId>,
    Json(payload): Json<PostCreateFolderRequest>,
) -> (StatusCode, Json<PostCreateFolderResponse>) {
    let parent_node = state.node_repository.get_node(parent_id).expect("db error");

    if parent_node.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(PostCreateFolderResponse::NotFound),
        );
    }

    let parent_node = parent_node.unwrap();

    if !state.node_repository.has_access(parent_node.id, current_user.id).expect("db error") {
        return (
            StatusCode::NOT_FOUND,
            Json(PostCreateFolderResponse::NotFound),
        );
    }

    // a file cannot have children
    if parent_node.node_type != NodeType::Folder {
        return (
            StatusCode::BAD_REQUEST,
            Json(PostCreateFolderResponse::BadRequest),
        );
    }

    //TODO this is not thread safe
    if parent_node.metadata_change_counter != payload.parent_metadata_version {
        return (
            StatusCode::CONFLICT,
            Json(PostCreateFolderResponse::Conflict),
        );
    }

    //update the parent
    state
        .node_repository
        .update_node(&NodeEntity {
            metadata: payload.parent_metadata,
            metadata_change_counter: parent_node.metadata_change_counter,
            ..parent_node
        })
        .expect("db error");

    //create the node
    let node = state
        .node_repository
        .create_node(
            Some(parent_id),
            payload.node_metadata,
            current_user.id,
            NodeType::Folder,
            payload.node_id,
        )
        .expect("db error");

    let response_node = entity_to_encrypted_node(node, &state).expect("db error");
    (
        StatusCode::CREATED,
        Json(PostCreateFolderResponse::Created(response_node)),
    )
}
