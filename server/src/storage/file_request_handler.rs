use crate::storage::node_request_handler::{get_example_node_info};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use crabdrive_common::payloads::node::request::file::{PostCommitFileRequest, PostCreateFileRequest, PostUpdateFileRequest};
use crabdrive_common::payloads::node::response::file::{PostCommitFileResponse, PostCreateFileResponse, PostUpdateFileResponse};
use uuid::Uuid;
use crabdrive_common::payloads::node::response::node::{FileRevision, NodeInfo};

//TODO fix this
pub fn get_example_revision_info() -> FileRevision {
    todo!()
}

pub async fn post_create_file(Path(_parent_id): Path<Uuid>, Json(_payload): Json<PostCreateFileRequest>) -> (StatusCode, Json<PostCreateFileResponse>) {
    //(StatusCode::NOT_FOUND, Json(PostCreateFileResponse::NotFound))
    //(StatusCode::CONFLICT, Json(PostCreateFileResponse::Conflict))
    //(StatusCode::BAD_REQUEST, Json(PostCreateFileResponse::BadRequest))

    //TODO implement
    (StatusCode::CREATED, Json(PostCreateFileResponse::Created(get_example_node_info())))
}

pub async fn post_update_file(Path(_file_id): Path<Uuid>, Json(_payload): Json<PostUpdateFileRequest>) -> (StatusCode, Json<PostUpdateFileResponse>) {
    //(StatusCode::NOT_FOUND, Json(PostUpdateFileResponse::NotFound))
    //(StatusCode::BAD_REQUEST, Json(PostUpdateFileResponse::BadRequest))

    //TODO implement
    (StatusCode::OK, Json(PostUpdateFileResponse::Ok(get_example_revision_info())))
}

pub async fn post_commit_file(Path((_file_id, _revision_id)): Path<(Uuid, Uuid)>, Json(_payload): Json<PostCommitFileRequest>) -> (StatusCode, Json<PostCommitFileResponse>) {
    //(StatusCode::NOT_FOUND, Json(PostCommitFileResponse::NotFound))
    //(StatusCode::BAD_REQUEST, Json(PostCommitFileResponse::BadRequest(vec![1,2,3])))

    //TODO implement
    (StatusCode::OK, Json(PostCommitFileResponse::Ok(get_example_node_info())))
}