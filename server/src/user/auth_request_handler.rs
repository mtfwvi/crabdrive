use axum::Json;
use axum::http::StatusCode;
use crabdrive_common::payloads::auth::request::login::PostLoginRequest;
use crabdrive_common::payloads::auth::request::register::PostRegisterRequest;
use crabdrive_common::payloads::auth::response::login::{LoginSuccess, PostLoginResponse};
use crabdrive_common::payloads::auth::response::register::PostRegisterResponse;

pub fn get_example_login_success() -> LoginSuccess {
    LoginSuccess::new(
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        false,
    )
}

pub async fn post_login(
    Json(_payload): Json<PostLoginRequest>,
) -> (StatusCode, Json<PostLoginResponse>) {
    //TODO implement
    (
        StatusCode::OK,
        Json(PostLoginResponse::Ok(get_example_login_success())),
    )
}

pub async fn post_register(
    Json(_payload): Json<PostRegisterRequest>,
) -> (StatusCode, Json<PostRegisterResponse>) {
    //TODO implement
    (StatusCode::CREATED, Json(PostRegisterResponse::Created))
}

pub async fn post_logout() -> StatusCode {
    //TODO implement (token blacklisting?)
    StatusCode::OK
}
