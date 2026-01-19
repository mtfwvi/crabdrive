use axum::Json;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use crabdrive_common::payloads::node::request::node::{
    DeleteNodeRequest, PatchNodeRequest, PathConstraints, PostMoveNodeOutOfTrashRequest,
    PostMoveNodeRequest, PostMoveNodeToTrashRequest,
};
use crabdrive_common::payloads::node::response::node::{
    DeleteNodeResponse, GetNodeChildrenResponse, GetNodeResponse, GetPathBetweenNodesResponse,
    NodeInfo, PatchNodeResponse, PostMoveNodeOutOfTrashResponse, PostMoveNodeResponse,
    PostMoveNodeToTrashResponse,
};

use crabdrive_common::iv::IV;
use crabdrive_common::storage::NodeId;
use crabdrive_common::storage::NodeType;

pub fn get_example_node_info() -> NodeInfo {
    NodeInfo {
        id: NodeId::random(),
        change_count: 0,
        parent_id: NodeId::random(),
        owner_id: NodeId::random(),
        deleted_on: None,
        node_type: NodeType::Folder,
        current_revision: None,
        encrypted_metadata: vec![],
        metadata_iv: IV::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    }
}

pub async fn delete_node(
    Path(_node_id): Path<NodeId>,
    Json(_payload): Json<DeleteNodeRequest>,
) -> (StatusCode, Json<DeleteNodeResponse>) {
    // (StatusCode::CONFLICT, Json(DeleteNodeResponse::Conflict))
    // (StatusCode::NOT_FOUND, Json(DeleteNodeResponse::NotFound))

    //TODO implement
    (StatusCode::OK, Json(DeleteNodeResponse::Ok))
}

pub async fn get_node(Path(_node_id): Path<NodeId>) -> (StatusCode, Json<GetNodeResponse>) {
    //(StatusCode::NOT_FOUND, Json(GetNodeResponse::NotFound))

    //TODO implement
    (
        StatusCode::OK,
        Json(GetNodeResponse::Ok(get_example_node_info())),
    )
}

pub async fn patch_node(
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
    Path(_node_id): Path<NodeId>,
    Json(_payload): Json<PostMoveNodeRequest>,
) -> (StatusCode, Json<PostMoveNodeResponse>) {
    //(StatusCode::NOT_FOUND, Json(PostMoveNodeResponse::NotFound))
    //(StatusCode::CONFLICT, Json(PostMoveNodeResponse::Conflict))

    //TODO implement
    (StatusCode::OK, Json(PostMoveNodeResponse::Ok))
}

pub async fn post_move_node_to_trash(
    Path(_node_id): Path<NodeId>,
    Json(_payload): Json<PostMoveNodeToTrashRequest>,
) -> (StatusCode, Json<PostMoveNodeToTrashResponse>) {
    //(StatusCode::NOT_FOUND, Json(PostMoveNodeToTrashResponse::NotFound))
    //(StatusCode::CONFLICT, Json(PostMoveNodeToTrashResponse::Conflict))

    //TODO implement
    (StatusCode::OK, Json(PostMoveNodeToTrashResponse::Ok))
}

pub async fn post_move_node_out_of_trash(
    Path(_node_id): Path<NodeId>,
    Json(_payload): Json<PostMoveNodeOutOfTrashRequest>,
) -> (StatusCode, Json<PostMoveNodeOutOfTrashResponse>) {
    //(StatusCode::NOT_FOUND, Json(PostMoveNodeOutOfTrashResponse::NotFound))
    //(StatusCode::CONFLICT, Json(PostMoveNodeOutOfTrashResponse::Conflict))

    //TODO implement
    (StatusCode::OK, Json(PostMoveNodeOutOfTrashResponse::Ok))
}

pub async fn get_path_between_nodes(
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
    Path(_node_id): Path<NodeId>,
) -> (StatusCode, Json<GetNodeChildrenResponse>) {
    //(StatusCode::NOT_FOUND, Json(GetNodeResponse::NotFound))

    //TODO implement
    (StatusCode::OK, Json(GetNodeChildrenResponse::Ok(vec![])))
}
