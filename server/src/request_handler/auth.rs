use crate::http::AppState;
use crate::user::auth::new_bearer_token;
use crate::user::persistence::model::user_entity::UserEntity;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use crabdrive_common::da;
use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::payloads::auth::request::login::PostLoginRequest;
use crabdrive_common::payloads::auth::request::register::PostRegisterRequest;
use crabdrive_common::payloads::auth::response::info::{GetSelfInfoResponse, SelfUserInfo};
use crabdrive_common::payloads::auth::response::login::LoginDeniedReason::Username;
use crabdrive_common::payloads::auth::response::login::{LoginSuccess, PostLoginResponse};

use crabdrive_common::payloads::auth::response::register::{
    PostRegisterResponse, RegisterConflictReason,
};
use crabdrive_common::storage::{NodeId, NodeType};
use crabdrive_common::user::UserKeys;

pub async fn post_login(
    State(state): State<AppState>,
    Json(payload): Json<PostLoginRequest>,
) -> (StatusCode, Json<PostLoginResponse>) {
    let username = payload.username;
    let password = payload.password;

    let user_entity = state
        .user_repository
        .authenticate_user(&username, &password)
        .expect("db error");

    if user_entity.is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(PostLoginResponse::Unauthorized(Username)),
        );
    }

    let user_entity = user_entity.unwrap();

    let jwt = new_bearer_token(
        user_entity.id,
        state.config.auth.jwt_expiration_period,
        &state.keys.encoding_key,
    )
    .unwrap();

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
        storage_limit: user.storage_limit,
        username: user.username,
        storage_used: user.storage_used,
    }))
}

pub async fn post_logout() -> StatusCode {
    //TODO implement (token blacklisting?)

    StatusCode::OK
}
