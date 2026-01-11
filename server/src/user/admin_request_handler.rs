use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use crabdrive_common::payloads::admin::request::user::{
    DeleteUserRequest, PatchUserRequest, PostUserRequest,
};
use crabdrive_common::payloads::admin::response::user::{
    DeleteUserResponse, GetUserResponse, PatchUserResponse, PostUserResponse, UserInfo,
};
use uuid::Uuid;

//TODO remove this
pub fn get_example_user_info() -> UserInfo {
    todo!()
}

pub async fn get_user(Path(_user_id): Path<Uuid>) -> (StatusCode, Json<GetUserResponse>) {
    //TODO implement
    (
        StatusCode::OK,
        Json(GetUserResponse::Ok(get_example_user_info())),
    )
}

pub async fn delete_user(
    Path(_user_id): Path<Uuid>,
    Json(_payload): Json<DeleteUserRequest>,
) -> (StatusCode, Json<DeleteUserResponse>) {
    //TODO implement
    (
        StatusCode::OK,
        Json(DeleteUserResponse::Ok(get_example_user_info())),
    )
}

pub async fn patch_user(
    Path(_user_id): Path<Uuid>,
    Json(_payload): Json<PatchUserRequest>,
) -> (StatusCode, Json<PatchUserResponse>) {
    //TODO implement
    (
        StatusCode::OK,
        Json(PatchUserResponse::Ok(get_example_user_info())),
    )
}

pub async fn post_user(
    Json(_payload): Json<PostUserRequest>,
) -> (StatusCode, Json<PostUserResponse>) {
    //TODO implement
    (
        StatusCode::CREATED,
        Json(PostUserResponse::Created(get_example_user_info())),
    )
}
