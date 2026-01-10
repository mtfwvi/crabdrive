use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::Json;
use crabdrive_common::payloads::node::request::node::{DeleteNodeRequest, PatchNodeRequest, PathConstraints, PostMoveNodeOutOfTrashRequest, PostMoveNodeRequest, PostMoveNodeToTrashRequest};
use crabdrive_common::payloads::node::response::node::{DeleteNodeResponse, GetNodeChildrenResponse, GetNodeResponse, GetPathBetweenNodesResponse, NodeInfo, PatchNodeResponse, PostMoveNodeOutOfTrashResponse, PostMoveNodeResponse, PostMoveNodeToTrashResponse};
use uuid::Uuid;

//TODO fix this
pub fn get_example_node_info() -> NodeInfo {
    todo!()
}

pub async fn delete_node(Path(_node_id): Path<Uuid>, Json(_payload): Json<DeleteNodeRequest>) -> (StatusCode, Json<DeleteNodeResponse>) {
    // (StatusCode::CONFLICT, Json(DeleteNodeResponse::Conflict))
    // (StatusCode::NOT_FOUND, Json(DeleteNodeResponse::NotFound))

    //TODO implement
    (StatusCode::OK, Json(DeleteNodeResponse::Ok))
}

pub async fn get_node(Path(_node_id): Path<Uuid>) -> (StatusCode, Json<GetNodeResponse>) {
    //(StatusCode::NOT_FOUND, Json(GetNodeResponse::NotFound))

    //TODO implement
    (StatusCode::OK, Json(GetNodeResponse::Ok(get_example_node_info())))
}

pub async fn patch_node(Path(_node_id): Path<Uuid>, Json(_payload): Json<PatchNodeRequest>) -> (StatusCode, Json<PatchNodeResponse>) {
    //(StatusCode::NOT_FOUND, Json(PatchNodeResponse::NotFound))
    //(StatusCode::CONFLICT, Json(PatchNodeResponse::Conflict))

    //TODO implement
    (StatusCode::OK, Json(PatchNodeResponse::Ok(get_example_node_info())))
}

pub async fn post_move_node(Path(_node_id): Path<Uuid>, Json(_payload): Json<PostMoveNodeRequest>) -> (StatusCode, Json<PostMoveNodeResponse>) {
    //(StatusCode::NOT_FOUND, Json(PostMoveNodeResponse::NotFound))
    //(StatusCode::CONFLICT, Json(PostMoveNodeResponse::Conflict))

    //TODO implement
    (StatusCode::OK, Json(PostMoveNodeResponse::Ok))
}

pub async fn post_move_node_to_trash(Path(_node_id): Path<Uuid>, Json(_payload): Json<PostMoveNodeToTrashRequest>) -> (StatusCode, Json<PostMoveNodeToTrashResponse>) {
    //(StatusCode::NOT_FOUND, Json(PostMoveNodeToTrashResponse::NotFound))
    //(StatusCode::CONFLICT, Json(PostMoveNodeToTrashResponse::Conflict))

    //TODO implement
    (StatusCode::OK, Json(PostMoveNodeToTrashResponse::Ok))
}

pub async fn post_move_node_out_of_trash(Path(_node_id): Path<Uuid>, Json(_payload): Json<PostMoveNodeOutOfTrashRequest>) -> (StatusCode, Json<PostMoveNodeOutOfTrashResponse>) {
    //(StatusCode::NOT_FOUND, Json(PostMoveNodeOutOfTrashResponse::NotFound))
    //(StatusCode::CONFLICT, Json(PostMoveNodeOutOfTrashResponse::Conflict))

    //TODO implement
    (StatusCode::OK, Json(PostMoveNodeOutOfTrashResponse::Ok))
}

//TODO add to openapi
pub async fn get_path_between_nodes(_path_constraints: Query<PathConstraints>) -> (StatusCode, Json<GetPathBetweenNodesResponse>) {
    //(StatusCode::NO_CONTENT, Json(GetPathBetweenNodesResponse::NoContent))

    //TODO implement
    (StatusCode::OK, Json(GetPathBetweenNodesResponse::Ok(vec![])))
}

//TODO add to openapi
pub async fn get_node_children(Path(_node_id): Path<Uuid>) -> (StatusCode, Json<GetNodeChildrenResponse>) {
    //(StatusCode::NOT_FOUND, Json(GetNodeResponse::NotFound))

    //TODO implement
    (StatusCode::OK, Json(GetNodeChildrenResponse::Ok(vec![])))
}