use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use crabdrive_common::payloads::admin::request::user::PostUserRequest;
use crabdrive_common::payloads::admin::response::user::{
    DeleteUserResponse, GetUserResponse, PostUserResponse, UserInfo,
};
use crabdrive_common::user::{UserId, UserType};

//TODO remove this
pub fn get_example_user_info() -> UserInfo {
    UserInfo {
        username: "admin".to_string(),
        user_type: UserType::User,
        storage_limit: None,
        created_on: Default::default(),
        updated_on: Default::default(),
    }
}

pub async fn get_user(Path(_user_id): Path<UserId>) -> (StatusCode, Json<GetUserResponse>) {
    //TODO implement
    (
        StatusCode::OK,
        Json(GetUserResponse::Ok(get_example_user_info())),
    )
}

pub async fn delete_user(Path(_user_id): Path<UserId>) -> (StatusCode, Json<DeleteUserResponse>) {
    //TODO implement
    (
        StatusCode::OK,
        Json(DeleteUserResponse::Ok(get_example_user_info())),
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
