use crate::request_handler::node::get_example_node_info;
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use crabdrive_common::iv::IV;
use crabdrive_common::payloads::node::request::file::{
    PostCreateFileRequest, PostUpdateFileRequest,
};
use crabdrive_common::payloads::node::response::file::{
    GetVersionsResponse, PostCommitFileResponse, PostCreateFileResponse, PostUpdateFileResponse,
};
use crabdrive_common::storage::FileRevision;
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
    Path(_parent_id): Path<NodeId>,
    Json(_payload): Json<PostCreateFileRequest>,
) -> (StatusCode, Json<PostCreateFileResponse>) {
    //(StatusCode::NOT_FOUND, Json(PostCreateFileResponse::NotFound))
    //(StatusCode::CONFLICT, Json(PostCreateFileResponse::Conflict))
    //(StatusCode::BAD_REQUEST, Json(PostCreateFileResponse::BadRequest))

    //TODO implement
    (
        StatusCode::CREATED,
        Json(PostCreateFileResponse::Created(get_example_node_info())),
    )
}

pub async fn post_update_file(
    Path(_file_id): Path<NodeId>,
    Json(_payload): Json<PostUpdateFileRequest>,
) -> (StatusCode, Json<PostUpdateFileResponse>) {
    //(StatusCode::NOT_FOUND, Json(PostUpdateFileResponse::NotFound))
    //(StatusCode::BAD_REQUEST, Json(PostUpdateFileResponse::BadRequest))

    //TODO implement
    (
        StatusCode::OK,
        Json(PostUpdateFileResponse::Ok(get_example_revision_info())),
    )
}

pub async fn post_commit_file(
    Path((_file_id, _revision_id)): Path<(NodeId, RevisionId)>,
) -> (StatusCode, Json<PostCommitFileResponse>) {
    //(StatusCode::NOT_FOUND, Json(PostCommitFileResponse::NotFound))
    //(StatusCode::BAD_REQUEST, Json(PostCommitFileResponse::BadRequest(vec![1,2,3])))

    //TODO implement
    (
        StatusCode::OK,
        Json(PostCommitFileResponse::Ok(get_example_node_info())),
    )
}

pub async fn get_file_versions(
    Path(_file_id): Path<NodeId>,
) -> (StatusCode, Json<GetVersionsResponse>) {
    //TODO implement
    (StatusCode::OK, Json(GetVersionsResponse::Ok(vec![])))
}
