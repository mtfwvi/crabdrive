use anyhow::Result;
use crabdrive_common::payloads::auth::{
    request::{login::PostLoginRequest, register::PostRegisterRequest},
    response::{login::PostLoginResponse, register::PostRegisterResponse},
};

use crate::{
    api::requests::{RequestBody, RequestMethod, request, string_from_response},
    utils,
};

pub async fn post_login(body: PostLoginRequest) -> Result<PostLoginResponse> {
    let url = crabdrive_common::routes::auth::login();
    let body = RequestBody::Json(serde_json::to_string(&body)?);

    let response = request(&url, RequestMethod::POST, body, None, true).await?;

    let response_string = string_from_response(response).await?;
    let response_object = serde_json::from_str(&response_string)?;

    Ok(response_object)
}

pub async fn post_register(body: PostRegisterRequest) -> Result<PostRegisterResponse> {
    let url = crabdrive_common::routes::auth::register();
    let body = RequestBody::Json(serde_json::to_string(&body)?);

    let response = request(&url, RequestMethod::POST, body, None, true).await?;

    let response_string = string_from_response(response).await?;
    let response_object = serde_json::from_str(&response_string)?;

    Ok(response_object)
}

pub async fn post_logout() -> Result<()> {
    let url = crabdrive_common::routes::auth::logout();
    let token = utils::auth::get_token()?;
    let _ = request(
        &url,
        RequestMethod::POST,
        RequestBody::Empty,
        Some(&token),
        true,
    )
    .await?;
    Ok(())
}
