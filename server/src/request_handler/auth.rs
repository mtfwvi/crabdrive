use crate::http::AppState;
use crate::user::persistence::model::user_entity::UserEntity;

use axum::Json;
use axum::extract::State;
use axum::http::header::SET_COOKIE;
use axum::http::{HeaderName, StatusCode};
use axum_extra::TypedHeader;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;

use crabdrive_common::da;
use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::payloads::auth::request::login::PostLoginRequest;
use crabdrive_common::payloads::auth::request::register::PostRegisterRequest;
use crabdrive_common::payloads::auth::response::info::{GetSelfInfoResponse, SelfUserInfo};
use crabdrive_common::payloads::auth::response::login::LoginDeniedReason::Username;
use crabdrive_common::payloads::auth::response::login::{LoginSuccess, PostLoginResponse};

use crabdrive_common::payloads::auth::response::refresh::{PostRefreshResponse, RefreshBody};
use crabdrive_common::payloads::auth::response::register::{
    PostRegisterResponse, RegisterConflictReason,
};
use crabdrive_common::storage::{NodeId, NodeType};
use crabdrive_common::user::UserKeys;

pub async fn post_login(
    State(state): State<AppState>,
    Json(payload): Json<PostLoginRequest>,
) -> (
    StatusCode,
    [(HeaderName, String); 1],
    Json<PostLoginResponse>,
) {
    let username = payload.username;
    let password = payload.password;

    let user_entity = state
        .user_repository
        .authenticate_user(&username, &password)
        .expect("db error");

    if user_entity.is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            [(SET_COOKIE, "".to_string())],
            Json(PostLoginResponse::Unauthorized(Username)),
        );
    }

    let user_entity = user_entity.unwrap();

    let (rtoken, jwt) = state
        .user_repository
        .create_session(user_entity.id)
        .expect("Failed to create session!");

    let cookie = Cookie::build(("refresh_token", rtoken))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/api/")
        .build()
        .to_string();

    let keys = if user_entity.encryption_uninitialized {
        None
    } else {
        Some(UserKeys::new(
            user_entity.public_key,
            user_entity.private_key,
            user_entity.master_key,
            user_entity.root_key,
            user_entity.trash_key,
        ))
    };

    (
        StatusCode::OK,
        [(SET_COOKIE, cookie)],
        Json(PostLoginResponse::Ok(LoginSuccess::new(
            jwt,
            format!("/{}", &user_entity.root_node.unwrap()),
            user_entity.root_node.unwrap(),
            user_entity.trash_node.unwrap(),
            user_entity.encryption_uninitialized,
            keys,
        ))),
    )
}

pub async fn post_register(
    State(state): State<AppState>,
    Json(payload): Json<PostRegisterRequest>,
) -> (StatusCode, Json<PostRegisterResponse>) {
    let username = payload.username;
    let password = payload.password;
    let keys = payload.keys;

    //TODO maybe check for weird characters in usernames

    if state
        .user_repository
        .get_user_by_username(&username)
        .expect("db error")
        .is_some()
    {
        return (
            StatusCode::CONFLICT,
            Json(PostRegisterResponse::Conflict(
                RegisterConflictReason::UsernameTaken,
            )),
        );
    }

    let mut user = state
        .user_repository
        .create_user(&username, &password, da!(15 GB), keys)
        .expect("db error when creating user");

    // Create Root & Trash node with uninitialized encryption
    // The client will initiliaze the metadata on first sign-in.

    let root_node = state
        .node_repository
        .create_node(
            None,
            EncryptedMetadata::nil(),
            user.id,
            NodeType::Folder,
            NodeId::random(),
        )
        .expect("db error when creating root node");

    user.root_node = Some(root_node.id);

    let trash_node = state
        .node_repository
        .create_node(
            None,
            EncryptedMetadata::nil(),
            user.id,
            NodeType::Folder,
            NodeId::random(),
        )
        .expect("db error when creating root node");

    user.trash_node = Some(trash_node.id);

    state.user_repository.update_user(user).expect("DB Error!");

    (StatusCode::CREATED, Json(PostRegisterResponse::Created))
}

pub async fn get_user_info(
    State(_state): State<AppState>,
    user: UserEntity,
) -> Json<GetSelfInfoResponse> {
    Json(GetSelfInfoResponse::Ok(SelfUserInfo {
        user_id: user.id,
        username: user.username,
    }))
}

pub async fn post_logout(
    State(state): State<AppState>,
    _user: UserEntity,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> (StatusCode, [(HeaderName, String); 1]) {
    let jwt = auth.token().to_string();
    state
        .user_repository
        .close_session(&jwt)
        .expect("Failed to close session");
    (StatusCode::OK, [(SET_COOKIE, "".to_string())])
}

pub async fn post_refresh(
    State(state): State<AppState>,
    jar: CookieJar,
) -> (
    StatusCode,
    [(HeaderName, String); 1],
    Json<PostRefreshResponse>,
) {
    let refresh_token = match jar.get("refresh_token") {
        Some(cookie) => cookie.value().to_string(),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                [(SET_COOKIE, "".to_string())],
                Json(PostRefreshResponse::Err),
            );
        }
    };

    let res = state.user_repository.refresh_session(&refresh_token);

    if res.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(SET_COOKIE, "".to_string())],
            Json(PostRefreshResponse::Err),
        );
    }

    let res = res.unwrap();

    if res.is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            [(SET_COOKIE, "".to_string())],
            Json(PostRefreshResponse::Err),
        );
    }

    let (r_tok, jwt) = res.unwrap();

    let cookie = Cookie::build(("refresh_token", r_tok))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/api/")
        .build()
        .to_string();

    (
        StatusCode::OK,
        [(SET_COOKIE, cookie)],
        Json(PostRefreshResponse::Ok(RefreshBody { bearer_token: jwt })),
    )
}
