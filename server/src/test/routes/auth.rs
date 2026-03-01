use std::time::Duration;

use axum_extra::extract::cookie::Cookie;
use crabdrive_common::payloads::auth::response::refresh::PostRefreshResponse;
use crabdrive_common::user::UserKeys;

use crabdrive_common::payloads::auth::{
    request::{login::*, register::*},
    response::{info::*, login::*, register::*},
};
use crabdrive_common::routes;

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

    assert!(ctx.validate_jwt(&login_responese.bearer_token));
    assert_eq!(login_responese.user_keys, Some(user.keys.clone()));
    assert_eq!(login_responese.root_node_id, user.entity.root_node.unwrap());
    assert_eq!(
        login_responese.trash_node_id,
        user.entity.trash_node.unwrap()
    );
}

#[tokio::test]
pub async fn test_login_with_wrong_password() {
    let ctx = TestContext::new(1).await;

    let user = ctx.get_user(0);
    let login_body = PostLoginRequest {
        username: user.username.clone(),
        password: TestContext::random_text(),
    };

    let login_request = ctx
        .server
        .post(&routes::auth::login())
        .json(&login_body)
        .await;

    login_request.assert_status_unauthorized();
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

    assert!(ctx.validate_jwt(&login_responese.bearer_token));
    assert_eq!(login_responese.user_keys, Some(keys.clone()));
}

#[tokio::test]
pub async fn test_user_info() {
    let ctx = TestContext::new(1).await;

    let user = ctx.get_user(0);
    let info_request = user.get(routes::auth::info()).await;

    let GetSelfInfoResponse::Ok(info_response) = info_request.json::<GetSelfInfoResponse>();

    info_request.assert_status_ok();
    assert_eq!(
        info_response,
        SelfUserInfo {
            user_id: user.id,
            username: user.username.clone(),
            storage_limit: crabdrive_common::da!(128 MiB), // In test environments: 128 MiB limit, otherwise 15GB
            storage_used: crabdrive_common::da!(0 B),
        }
    );
}

#[tokio::test]
pub async fn test_missing_token() {
    let ctx = TestContext::new(1).await;

    let info_request = ctx.server.get(&routes::auth::info()).await;

    info_request.assert_status_unauthorized();
}

#[tokio::test]
pub async fn test_wrong_token() {
    let ctx = TestContext::new(1).await;

    let info_request = ctx
        .server
        .get(&routes::auth::info())
        .authorization_bearer("CrabdriveIsBest")
        .await;

    info_request.assert_status_unauthorized();
}

#[tokio::test]
pub async fn test_logout() {
    let ctx = TestContext::new(1).await;
    let user1 = ctx.get_user(0);

    let logout_request = user1.post(routes::auth::logout()).await;
    logout_request.assert_status_ok();

    let info_request = user1.get(&routes::auth::info()).await;
    info_request.assert_status_unauthorized();
}

#[tokio::test]
pub async fn test_logout_without_authorization() {
    let ctx = TestContext::new(1).await;

    let request = ctx.server.post(&routes::auth::logout()).await;

    request.assert_status_unauthorized();
}

#[tokio::test]
pub async fn test_jwt_reusal_after_logout() {
    let ctx = TestContext::new(1).await;
    let user1 = ctx.get_user(0);

    let logout_request = user1.post(routes::auth::logout()).await;
    logout_request.assert_status_ok();
}

#[tokio::test]
pub async fn test_refresh_token() {
    let ctx = TestContext::new(1).await;
    let user1 = ctx.get_user(0);

    let request = user1
        .post(routes::auth::refresh())
        .add_cookie(Cookie::new("refresh_token", user1.refresh_token.clone()))
        .await;

    request.assert_status_ok();

    let response = match request.json::<PostRefreshResponse>() {
        PostRefreshResponse::Ok(refresh_body) => refresh_body,
        PostRefreshResponse::Err => panic!("Wrong status code"),
    };

    assert_ne!(response.bearer_token, user1.token);

    let refresh_token = request.cookie("refresh_token");

    assert!(refresh_token.secure().expect("Failed to check cookie"));
    assert_eq!(
        refresh_token.path().expect("Failed to check cookie"),
        routes::auth::ROUTE_REFRESH
    );
    assert!(refresh_token.http_only().expect("Failed to check cookie"));
}

#[tokio::test]
pub async fn test_refresh_token_reusal_in_grace_period() {
    let ctx = TestContext::new(1).await;
    let user1 = ctx.get_user(0);

    let request = user1
        .post(routes::auth::refresh())
        .add_cookie(Cookie::new("refresh_token", user1.refresh_token.clone()))
        .await;

    request.assert_status_ok();

    let response = match request.json::<PostRefreshResponse>() {
        PostRefreshResponse::Ok(refresh_body) => refresh_body,
        PostRefreshResponse::Err => panic!("Wrong status code"),
    };

    assert_ne!(response.bearer_token, user1.token);

    let refresh_token = request.cookie("refresh_token");

    assert!(refresh_token.secure().expect("Failed to check cookie"));
    assert_eq!(
        refresh_token.path().expect("Failed to check cookie"),
        routes::auth::ROUTE_REFRESH
    );
    assert!(refresh_token.http_only().expect("Failed to check cookie"));

    tokio::time::sleep(Duration::from_secs(2)).await;

    let request = user1
        .post(routes::auth::refresh())
        // Reuse old refresh token
        .add_cookie(Cookie::new("refresh_token", user1.refresh_token.clone()))
        .await;

    request.assert_status_ok();

    let response = match request.json::<PostRefreshResponse>() {
        PostRefreshResponse::Ok(refresh_body) => refresh_body,
        PostRefreshResponse::Err => panic!("Wrong status code"),
    };

    assert_ne!(response.bearer_token, user1.token);

    let refresh_token = request.cookie("refresh_token");

    assert!(refresh_token.secure().expect("Failed to check cookie"));
    assert_eq!(
        refresh_token.path().expect("Failed to check cookie"),
        routes::auth::ROUTE_REFRESH
    );
    assert!(refresh_token.http_only().expect("Failed to check cookie"));
}
