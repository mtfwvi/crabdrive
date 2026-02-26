use crabdrive_common::user::UserKeys;

use crabdrive_common::routes;
use crabdrive_common::payloads::auth::{
    request::{login::*, register::*},
    response::{info::*, login::*, register::*},
};

use crate::test::utils::TestContext;

use axum::http::StatusCode;

#[tokio::test]
pub async fn test_register() {
    let ctx = TestContext::new(1).await;

    let register_body = PostRegisterRequest {
        username: TestContext::random_text(),
        password: TestContext::random_text(),
        keys: UserKeys::nil(),
    };

    let register_response = ctx
        .server
        .post(&routes::auth::register())
        .json(&register_body)
        .await;

    assert_eq!(register_response.status_code(), StatusCode::CREATED);
}

#[tokio::test]
pub async fn test_register_with_conflicting_username() {
    let ctx = TestContext::new(1).await;

    let register_body = PostRegisterRequest {
        username: TestContext::random_text(),
        password: TestContext::random_text(),
        keys: UserKeys::nil(),
    };

    let register_response = ctx
        .server
        .post(&routes::auth::register())
        .json(&register_body)
        .await;

    assert_eq!(register_response.status_code(), StatusCode::CREATED);

    let register_response_2 = ctx
        .server
        .post(&routes::auth::register())
        .json(&register_body)
        .await;

    register_response_2.assert_status_conflict();
    register_response_2.assert_json(&PostRegisterResponse::Conflict(
        RegisterConflictReason::UsernameTaken,
    ));
}

#[tokio::test]
pub async fn test_login() {
    let ctx = TestContext::new(1).await;

    let user = ctx.get_user(0);
    let login_body = PostLoginRequest {
        username: user.username.clone(),
        password: user.password.clone(),
    };

    let login_request = ctx
        .server
        .post(&routes::auth::login())
        .json(&login_body)
        .await;

    login_request.assert_status_ok();

    let login_responese: PostLoginResponse = login_request.json();
    let login_responese = match login_responese {
        PostLoginResponse::Ok(login_success) => login_success,
        PostLoginResponse::Unauthorized(_) => panic!("Server returned wrong status code!"),
    };

    assert_eq!(ctx.validate_jwt(&login_responese.bearer_token), true);
    assert_eq!(login_responese.user_keys, Some(user.keys.clone()));
    assert_eq!(login_responese.root_node_id, user.entity.root_node.unwrap());
    assert_eq!(
        login_responese.trash_node_id,
        user.entity.trash_node.unwrap()
    );
}

#[tokio::test]
pub async fn test_register_account_and_login() {
    let ctx = TestContext::new(1).await;

    let username = TestContext::random_text();
    let password = TestContext::random_text();
    let keys = UserKeys::random();

    let register_body = PostRegisterRequest {
        username: username.clone(),
        password: password.clone(),
        keys: keys.clone(),
    };

    let register_response = ctx
        .server
        .post(&routes::auth::register())
        .json(&register_body)
        .await;

    assert_eq!(register_response.status_code(), StatusCode::CREATED);

    let login_body = PostLoginRequest {
        username: register_body.username.clone(),
        password: register_body.password.clone(),
    };

    let login_request = ctx
        .server
        .post(&routes::auth::login())
        .json(&login_body)
        .await;

    login_request.assert_status_ok();

    let login_responese: PostLoginResponse = login_request.json();
    let login_responese = match login_responese {
        PostLoginResponse::Ok(login_success) => login_success,
        PostLoginResponse::Unauthorized(_) => panic!("Server returned wrong status code!"),
    };

    assert_eq!(ctx.validate_jwt(&login_responese.bearer_token), true);
    assert_eq!(login_responese.user_keys, Some(keys.clone()));
}

#[tokio::test]
pub async fn test_user_info() {
    let ctx = TestContext::new(1).await;

    let user = ctx.get_user(0);
    let info_request = user.get(&routes::auth::info())
        .await;

    let info_response = match info_request.json::<GetSelfInfoResponse>() {
        GetSelfInfoResponse::Ok(self_user_info) => self_user_info,
    };

    info_request.assert_status_ok();
    assert_eq!(info_response, SelfUserInfo {
        user_id: user.id,
        username: user.username.clone(),
    });
}

#[tokio::test]
pub async fn test_missing_token() {
    let ctx = TestContext::new(1).await;

    let info_request = ctx.server
        .get(&routes::auth::info())
        .await;

    info_request.assert_status_unauthorized();
    // TODO: Check response body
}

#[tokio::test]
pub async fn test_wrong_token() {
    let ctx = TestContext::new(1).await;

    let info_request = ctx.server
        .get(&routes::auth::info())
        .authorization_bearer("CrabdriveIsBest")
        .await;

    info_request.assert_status_unauthorized();
}
