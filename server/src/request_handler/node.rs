use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use crabdrive_common::encrypted_metadata::EncryptedMetadata;
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
use crabdrive_common::iv::IV;
use crabdrive_common::storage::{EncryptedNode, NodeId};
use crabdrive_common::storage::{FileRevision, NodeType};

pub fn get_example_node_info() -> EncryptedNode {
    EncryptedNode {
        id: NodeId::random(),
        change_count: 0,
        parent_id: Some(NodeId::random()),
        owner_id: NodeId::random(),
        deleted_on: None,
        node_type: NodeType::Folder,
        current_revision: None,
        encrypted_metadata: EncryptedMetadata::new(
            vec![],
            IV::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        ),
    }
}

pub async fn delete_node(
    State(state): State<AppState>,
    Path(_node_id): Path<NodeId>,
    Json(_payload): Json<DeleteNodeRequest>,
) -> (StatusCode, Json<DeleteNodeResponse>) {
    // (StatusCode::CONFLICT, Json(DeleteNodeResponse::Conflict))
    // (StatusCode::NOT_FOUND, Json(DeleteNodeResponse::NotFound))

    //TODO implement
    (StatusCode::OK, Json(DeleteNodeResponse::Ok))
}

pub async fn get_node(State(state): State<AppState>, Path(node_id): Path<NodeId>) -> (StatusCode, Json<GetNodeResponse>) {
    let node_entity = state.node_repository.get_node(node_id);

    if node_entity.as_ref().unwrap().is_none() {
        return (StatusCode::NOT_FOUND, Json(GetNodeResponse::NotFound));
    }

    let node = entity_to_encrypted_node(node_entity.unwrap().unwrap(), State(&state)).unwrap();

    (StatusCode::OK, Json(GetNodeResponse::Ok(node)))
}

pub async fn patch_node(
    State(state): State<AppState>,
    Path(_node_id): Path<NodeId>,
    Json(_payload): Json<PatchNodeRequest>,
) -> (StatusCode, Json<PatchNodeResponse>) {
    //(StatusCode::NOT_FOUND, Json(PatchNodeResponse::NotFound))
    //(StatusCode::CONFLICT, Json(PatchNodeResponse::Conflict))

    //TODO implement
    (
        StatusCode::OK,
        Json(PatchNodeResponse::Ok(get_example_node_info())),
    )
}

pub async fn post_move_node(
    State(state): State<AppState>,
    Path(_node_id): Path<NodeId>,
    Json(_payload): Json<PostMoveNodeRequest>,
) -> (StatusCode, Json<PostMoveNodeResponse>) {
    //(StatusCode::NOT_FOUND, Json(PostMoveNodeResponse::NotFound))
    //(StatusCode::CONFLICT, Json(PostMoveNodeResponse::Conflict))

    //TODO implement
    (StatusCode::OK, Json(PostMoveNodeResponse::Ok))
}

pub async fn post_move_node_to_trash(
    State(state): State<AppState>,
    Path(_node_id): Path<NodeId>,
    Json(_payload): Json<PostMoveNodeToTrashRequest>,
) -> (StatusCode, Json<PostMoveNodeToTrashResponse>) {
    //(StatusCode::NOT_FOUND, Json(PostMoveNodeToTrashResponse::NotFound))
    //(StatusCode::CONFLICT, Json(PostMoveNodeToTrashResponse::Conflict))

    //TODO implement
    (StatusCode::OK, Json(PostMoveNodeToTrashResponse::Ok))
}

pub async fn post_move_node_out_of_trash(
    State(state): State<AppState>,
    Path(_node_id): Path<NodeId>,
    Json(_payload): Json<PostMoveNodeOutOfTrashRequest>,
) -> (StatusCode, Json<PostMoveNodeOutOfTrashResponse>) {
    //(StatusCode::NOT_FOUND, Json(PostMoveNodeOutOfTrashResponse::NotFound))
    //(StatusCode::CONFLICT, Json(PostMoveNodeOutOfTrashResponse::Conflict))

    //TODO implement
    (StatusCode::OK, Json(PostMoveNodeOutOfTrashResponse::Ok))
}

pub async fn get_path_between_nodes(
    State(state): State<AppState>,
    _path_constraints: Query<PathConstraints>,
) -> (StatusCode, Json<GetPathBetweenNodesResponse>) {
    //(StatusCode::NO_CONTENT, Json(GetPathBetweenNodesResponse::NoContent))

    //TODO implement
    (
        StatusCode::OK,
        Json(GetPathBetweenNodesResponse::Ok(vec![])),
    )
}

pub async fn get_node_children(
    State(state): State<AppState>,
    Path(parent_id): Path<NodeId>,
) -> (StatusCode, Json<GetNodeChildrenResponse>) {
    let node = state.node_repository.get_node(parent_id);

    if node.unwrap().is_none() {
        return (StatusCode::NOT_FOUND, Json(GetNodeChildrenResponse::NotFound));
    }

    let children = state.node_repository.get_children(parent_id).unwrap();

    let children = children.iter().map(|entity| {
        let node = entity_to_encrypted_node(entity.clone(), State(&state)).unwrap();
        node
    });

    (StatusCode::OK, Json(GetNodeChildrenResponse::Ok(children.collect())))
}


fn entity_to_encrypted_node(node: NodeEntity, State(state): State<&AppState>) -> anyhow::Result<EncryptedNode> {
    let current_revision = match node.current_revision {
        Some(id) => {
            let entity = state.revision_repository.get_revision(id)?.expect("data is not consistent");
            Some(entity_to_file_revision(entity, State(state))?)
        }
        None => None
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
fn entity_to_file_revision(revision: RevisionEntity, State(state): State<&AppState>) -> anyhow::Result<FileRevision> {
    //TODO there is no way to retrieve the chunk count

    Ok(FileRevision {
        id: revision.id,
        upload_ended_on: revision.upload_ended_on,
        upload_started_on: revision.upload_started_on,
        iv: revision.iv,
        //TODO fix
        chunk_count: 1,
    })
}

