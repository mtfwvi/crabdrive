use crate::storage::node_request_handler::{get_example_node_info};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use crabdrive_common::payloads::node::request::folder::PostCreateFolderRequest;
use crabdrive_common::payloads::node::response::folder::PostCreateFolderResponse;
use uuid::Uuid;

pub async fn post_create_folder(Path(_parent_id): Path<Uuid>, Json(_payload): Json<PostCreateFolderRequest>) -> (StatusCode, Json<PostCreateFolderResponse>) {
    //(StatusCode::NOT_FOUND, Json(PostCreateFolderResponse::NotFound))
    //(StatusCode::CONFLICT, Json(PostCreateFolderResponse::Conflict))
    //(StatusCode::BAD_REQUEST, Json(PostCreateFolderResponse::BadRequest))

    //TODO implement
    (StatusCode::CREATED, Json(PostCreateFolderResponse::Created(get_example_node_info())))
}
