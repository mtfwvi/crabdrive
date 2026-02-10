use std::vec;

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use crabdrive_common::payloads::node::request::node::{
    DeleteNodeRequest, PatchNodeRequest, PathConstraints, PostMoveNodeOutOfTrashRequest,
    PostMoveNodeRequest, PostMoveNodeToTrashRequest,
};
use crabdrive_common::payloads::node::response::node::{
    DeleteNodeResponse, GetNodeChildrenResponse, GetNodeResponse, GetPathBetweenNodesResponse,
    PatchNodeResponse, PostMoveNodeOutOfTrashResponse, PostMoveNodeResponse,
    PostMoveNodeToTrashResponse,
};

use crate::http::AppState;
use crate::storage::node::persistence::model::node_entity::NodeEntity;
use crate::storage::revision::persistence::model::revision_entity::RevisionEntity;
use crate::user::persistence::model::user_entity::UserEntity;
use crabdrive_common::storage::FileRevision;
use crabdrive_common::storage::{EncryptedNode, NodeId};

pub async fn delete_node(
    State(_state): State<AppState>,
    Path(_node_id): Path<NodeId>,
    Json(_payload): Json<DeleteNodeRequest>,
) -> (StatusCode, Json<DeleteNodeResponse>) {
    // (StatusCode::CONFLICT, Json(DeleteNodeResponse::Conflict))
    // (StatusCode::NOT_FOUND, Json(DeleteNodeResponse::NotFound))

    //TODO implement
    (StatusCode::OK, Json(DeleteNodeResponse::Ok))
}

pub async fn get_node(
    current_user: UserEntity,
    State(state): State<AppState>,
    Path(node_id): Path<NodeId>,
) -> (StatusCode, Json<GetNodeResponse>) {
    let node_entity = state.node_repository.get_node(node_id).expect("db error");

    if node_entity.is_none() {
        return (StatusCode::NOT_FOUND, Json(GetNodeResponse::NotFound));
    }
    let node_entity = node_entity.unwrap();

    if node_entity.owner_id != current_user.id {
        return (StatusCode::NOT_FOUND, Json(GetNodeResponse::NotFound));
    }

    let node = entity_to_encrypted_node(node_entity, &state).unwrap();

    (StatusCode::OK, Json(GetNodeResponse::Ok(node)))
}

pub async fn patch_node(
    current_user: UserEntity,
    State(state): State<AppState>,
    Path(node_id): Path<NodeId>,
    Json(payload): Json<PatchNodeRequest>,
) -> (StatusCode, Json<PatchNodeResponse>) {
    // TODO very janky

    let node = state.node_repository.get_node(node_id).expect("db error");

    if node.is_none() {
        return (StatusCode::NOT_FOUND, Json(PatchNodeResponse::NotFound));
    }

    let node_entity = node.unwrap();

    if node_entity.owner_id != current_user.id {
        return (StatusCode::NOT_FOUND, Json(PatchNodeResponse::NotFound));
    }

    // TODO this should happen in one transaction as it it could lead to updates being lost
    // (which is bad)
    if node_entity.metadata_change_counter != payload.node_change_count {
        return (StatusCode::CONFLICT, Json(PatchNodeResponse::Conflict));
    }

    let updated_node = NodeEntity {
        metadata: payload.node_metadata,
        metadata_change_counter: node_entity.metadata_change_counter + 1,
        ..node_entity
    };

    let updated_node_entity = state
        .node_repository
        .update_node(&updated_node)
        .expect("db error");

    let updated_node = entity_to_encrypted_node(updated_node_entity, &state).expect("db error");
    (StatusCode::OK, Json(PatchNodeResponse::Ok(updated_node)))
}

pub async fn post_move_node(
    current_user: UserEntity,
    State(state): State<AppState>,
    Path(node_id): Path<NodeId>,
    Json(payload): Json<PostMoveNodeRequest>,
) -> (StatusCode, Json<PostMoveNodeResponse>) {
    let node = state.node_repository.get_node(node_id).expect("db error");
    if node.is_none() {
        return (StatusCode::NOT_FOUND, Json(PostMoveNodeResponse::NotFound));
    }
    let node = node.unwrap();

    if node.owner_id != current_user.id {
        return (StatusCode::NOT_FOUND, Json(PostMoveNodeResponse::NotFound));
    }

    let to_node = state
        .node_repository
        .get_node(payload.to_node_id)
        .expect("db error");
    let from_node = state
        .node_repository
        .get_node(node.parent_id.expect("node to be moved has no parent"))
        .expect("db error");

    if to_node.is_none() || from_node.is_none() {
        return (StatusCode::NOT_FOUND, Json(PostMoveNodeResponse::NotFound));
    }
    let (to_node, from_node) = (to_node.unwrap(), from_node.unwrap());

    if to_node.owner_id != current_user.id || from_node.owner_id != current_user.id {
        return (StatusCode::NOT_FOUND, Json(PostMoveNodeResponse::NotFound));
    }

    //TODO check version (in one transaction)

    state
        .node_repository
        .move_node(
            node_id,
            from_node.id,
            payload.from_node_metadata,
            to_node.id,
            payload.to_node_metadata,
        )
        .expect("db error");
    (StatusCode::OK, Json(PostMoveNodeResponse::Ok))
}

pub async fn post_move_node_to_trash(
    State(_state): State<AppState>,
    Path(_node_id): Path<NodeId>,
    Json(_payload): Json<PostMoveNodeToTrashRequest>,
) -> (StatusCode, Json<PostMoveNodeToTrashResponse>) {
    //(StatusCode::NOT_FOUND, Json(PostMoveNodeToTrashResponse::NotFound))
    //(StatusCode::CONFLICT, Json(PostMoveNodeToTrashResponse::Conflict))

    //TODO implement
    (StatusCode::OK, Json(PostMoveNodeToTrashResponse::Ok))
}

pub async fn post_move_node_out_of_trash(
    State(_state): State<AppState>,
    Path(_node_id): Path<NodeId>,
    Json(_payload): Json<PostMoveNodeOutOfTrashRequest>,
) -> (StatusCode, Json<PostMoveNodeOutOfTrashResponse>) {
    //(StatusCode::NOT_FOUND, Json(PostMoveNodeOutOfTrashResponse::NotFound))
    //(StatusCode::CONFLICT, Json(PostMoveNodeOutOfTrashResponse::Conflict))

    //TODO implement
    (StatusCode::OK, Json(PostMoveNodeOutOfTrashResponse::Ok))
}

pub async fn get_path_between_nodes(
    current_user: UserEntity,
    State(state): State<AppState>,
    path_constraints: Query<PathConstraints>,
) -> (StatusCode, Json<GetPathBetweenNodesResponse>) {
    //TODO maybe write recursive sql

    let to_node_id = path_constraints.0.to_id;
    let from_node_id = path_constraints.0.from_id;
    let to_node = state
        .node_repository
        .get_node(to_node_id)
        .expect("db error");

    if to_node.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(GetPathBetweenNodesResponse::NotFound),
        );
    }
    let mut path = vec![to_node.unwrap()];

    loop {
        if path.last().unwrap().id == from_node_id {
            break;
        }

        let new_parent_id = path.last().unwrap().parent_id;
        if new_parent_id.is_none() {
            // reached a node with no parent that is not the node we are looking for -> the path
            // does not exist
            return (
                StatusCode::NO_CONTENT,
                Json(GetPathBetweenNodesResponse::NoContent),
            );
        }

        let parent = state
            .node_repository
            .get_node(new_parent_id.unwrap())
            .unwrap()
            .unwrap();
        path.push(parent);
    }

    let path: Vec<EncryptedNode> = path
        .into_iter()
        .map(|node_entity| entity_to_encrypted_node(node_entity.clone(), &state).unwrap())
        .rev()
        .collect();

    if path[0].owner_id != current_user.id || path.last().unwrap().owner_id != current_user.id {
        return (
            StatusCode::NOT_FOUND,
            Json(GetPathBetweenNodesResponse::NotFound),
        );
    }

    (StatusCode::OK, Json(GetPathBetweenNodesResponse::Ok(path)))
}

pub async fn get_node_children(
    current_user: UserEntity,
    State(state): State<AppState>,
    Path(parent_id): Path<NodeId>,
) -> (StatusCode, Json<GetNodeChildrenResponse>) {
    let node = state.node_repository.get_node(parent_id).expect("db error");

    if node.as_ref().is_none() || node.unwrap().owner_id != current_user.id {
        return (
            StatusCode::NOT_FOUND,
            Json(GetNodeChildrenResponse::NotFound),
        );
    }

    let children = state.node_repository.get_children(parent_id).unwrap();

    let children = children
        .iter()
        .map(|entity| entity_to_encrypted_node(entity.clone(), &state).unwrap());

    (
        StatusCode::OK,
        Json(GetNodeChildrenResponse::Ok(children.collect())),
    )
}

pub fn entity_to_encrypted_node(
    node: NodeEntity,
    state: &AppState,
) -> anyhow::Result<EncryptedNode> {
    let current_revision = match node.current_revision {
        Some(id) => {
            let entity = state
                .revision_repository
                .get_revision(id)?
                .expect("data is not consistent");
            Some(entity_to_file_revision(entity))
        }
        None => None,
    };
    Ok(EncryptedNode {
        id: node.id,
        change_count: node.metadata_change_counter,
        parent_id: node.parent_id,
        owner_id: node.owner_id,
        deleted_on: node.deleted_on,
        node_type: node.node_type,
        current_revision,
        encrypted_metadata: node.metadata,
    })
}
pub fn entity_to_file_revision(revision: RevisionEntity) -> FileRevision {
    FileRevision {
        id: revision.id,
        upload_ended_on: revision.upload_ended_on,
        upload_started_on: revision.upload_started_on,
        iv: revision.iv,
        chunk_count: revision.chunk_count,
    }
}
