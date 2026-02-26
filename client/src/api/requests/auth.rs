use crate::api::requests::{RequestBody, RequestMethod, request, string_from_response};
use crate::utils::auth::get_token;
use anyhow::Result;
use crabdrive_common::payloads::auth::response::info::SelfUserInfo;
use crabdrive_common::payloads::auth::{
    request::{login::PostLoginRequest, register::PostRegisterRequest},
    response::{login::PostLoginResponse, register::PostRegisterResponse},
};
use crabdrive_common::routes;

pub async fn post_login(body: PostLoginRequest) -> Result<PostLoginResponse> {
    let url = crabdrive_common::routes::auth::login();
    let body = RequestBody::Json(serde_json::to_string(&body).unwrap());

    let response = request(url, RequestMethod::POST, body, vec![], None, true).await?;

    let response_string = string_from_response(response).await?;
    let response_object = serde_json::from_str(&response_string).unwrap();

    Ok(response_object)
}

pub async fn post_register(body: PostRegisterRequest) -> Result<PostRegisterResponse> {
    let url = crabdrive_common::routes::auth::register();
    let body = RequestBody::Json(serde_json::to_string(&body).unwrap());

    let response = request(url, RequestMethod::POST, body, vec![], None, true).await?;

    let response_string = string_from_response(response).await?;
    let response_object = serde_json::from_str(&response_string).unwrap();

    Ok(response_object)
}

pub async fn post_logout() -> Result<()> {
    let url = crabdrive_common::routes::auth::logout();

    let _response = request(
        url,
        RequestMethod::POST,
        RequestBody::Empty,
        vec![],
        None,
        true,
    )
    .await;

    // TODO: Do anything with response?
    Ok(())
}

pub async fn get_self_user_info() -> Result<SelfUserInfo> {
    let url = routes::auth::info();
    let token = get_token()?;

    let response = request(
        url,
        RequestMethod::GET,
        RequestBody::Empty,
        vec![],
        Some(&token),
        true,
    )
    .await?;

    let response_string = string_from_response(response).await?;
    let response_object = serde_json::from_str(&response_string)?;

    Ok(response_object)
}
