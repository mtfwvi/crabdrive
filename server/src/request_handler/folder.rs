use crate::request_handler::node::get_example_node_info;
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use crabdrive_common::payloads::node::request::folder::PostCreateFolderRequest;
use crabdrive_common::payloads::node::response::folder::PostCreateFolderResponse;
use crabdrive_common::storage::NodeId;

pub async fn post_create_folder(
    Path(_parent_id): Path<NodeId>,
    Json(_payload): Json<PostCreateFolderRequest>,
) -> (StatusCode, Json<PostCreateFolderResponse>) {
    //(StatusCode::NOT_FOUND, Json(PostCreateFolderResponse::NotFound))
    //(StatusCode::CONFLICT, Json(PostCreateFolderResponse::Conflict))
    //(StatusCode::BAD_REQUEST, Json(PostCreateFolderResponse::BadRequest))

    //TODO implement
    (
        StatusCode::CREATED,
        Json(PostCreateFolderResponse::Created(get_example_node_info())),
    )
}
