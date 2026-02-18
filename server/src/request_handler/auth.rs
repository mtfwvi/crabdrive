use crate::auth::new_bearer_token;
use crate::http::AppState;
use crate::user::persistence::model::user_entity::UserEntity;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use crabdrive_common::da;
use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::payloads::auth::request::login::PostLoginRequest;
use crabdrive_common::payloads::auth::request::register::PostRegisterRequest;
use crabdrive_common::payloads::auth::response::info::{GetSelfInfoResponse, SelfUserInfo};
use crabdrive_common::payloads::auth::response::login::LoginDeniedReason::{Password, Username};
use crabdrive_common::payloads::auth::response::login::{
    LoginSuccess, PostLoginResponse, UserKeys,
};
use crabdrive_common::payloads::auth::response::register::{
    PostRegisterResponse, RegisterConflictReason,
};
use crabdrive_common::storage::{NodeId, NodeType};
use tracing::log::debug;

pub async fn post_login(
    State(state): State<AppState>,
    Json(payload): Json<PostLoginRequest>,
) -> (StatusCode, Json<PostLoginResponse>) {
    let username = payload.username;

    let user_entity = state
        .user_repository
        .get_user_by_username(&username)
        .expect("db error");

    if user_entity.is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(PostLoginResponse::Unauthorized(Username)),
        );
    }

    let mut user_entity = user_entity.unwrap();

    let parsed_hash = PasswordHash::new(&user_entity.password_hash);
    if parsed_hash.is_ok()
        && Argon2::default()
            .verify_password(payload.password.as_bytes(), &parsed_hash.unwrap())
            .is_ok()
    {
        let jwt = new_bearer_token(
            user_entity.id,
            state.config.auth.jwt_expiration_period,
            &state.keys.encoding_key,
        )
        .unwrap();

        if user_entity.trash_node.is_none() {
            debug!(
                "login: user {}:{} does not have a trash node",
                username, user_entity.id
            );

            let inserted = state
                .node_repository
                .create_node(
                    None,
                    EncryptedMetadata::nil(),
                    user_entity.id,
                    NodeType::Folder,
                    NodeId::random(),
                )
                .expect("db error when creating trash node");

            user_entity.trash_node = Some(inserted.id);
            user_entity = state
                .user_repository
                .update_user(user_entity)
                .expect("db error when setting new trash node on user");
        }

        if user_entity.root_node.is_none() {
            debug!(
                "login: user {}:{} does not have a root node",
                username, user_entity.id
            );

            let inserted = state
                .node_repository
                .create_node(
                    None,
                    EncryptedMetadata::nil(),
                    user_entity.id,
                    NodeType::Folder,
                    NodeId::random(),
                )
                .expect("db error when creating trash node");

            user_entity.root_node = Some(inserted.id);
            user_entity = state
                .user_repository
                .update_user(user_entity)
                .expect("db error when setting new root node on user");
        }

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

        return (
            StatusCode::OK,
            Json(PostLoginResponse::Ok(LoginSuccess::new(
                jwt,
                format!("/{}", &user_entity.root_node.unwrap()),
                user_entity.root_node.unwrap(),
                user_entity.trash_node.unwrap(),
                user_entity.encryption_uninitialized,
                keys,
            ))),
        );
    }

    (
        StatusCode::UNAUTHORIZED,
        Json(PostLoginResponse::Unauthorized(Password)),
    )
}

pub async fn post_register(
    State(state): State<AppState>,
    Json(payload): Json<PostRegisterRequest>,
) -> (StatusCode, Json<PostRegisterResponse>) {
    let username = payload.username;

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

    let password_salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &password_salt)
        .unwrap()
        .to_string();

    // TODO pretty inefficient to do create + update
    let mut created_user = state
        .user_repository
        .create_user(username, password_hash, da!(15 GB))
        .expect("db error when creating user");

    debug!("Master Key: {:?}", &payload.keys.root_key);
    debug!("Trash Key: {:?}", &payload.keys.trash_key);
    debug!("Root Key: {:?}", &payload.keys.root_key);
    debug!("Public Key: {:?}", &payload.keys.public_key);
    debug!("Private Key: {:?}", &payload.keys.private_key);

    created_user.encryption_uninitialized = false;
    created_user.root_key = payload.keys.root_key;
    created_user.trash_key = payload.keys.trash_key;
    created_user.public_key = payload.keys.public_key;
    created_user.master_key = payload.keys.master_key;
    created_user.private_key = payload.keys.private_key;

    let _ = state
        .user_repository
        .update_user(created_user)
        .expect("db error when updating user");

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

pub async fn post_logout() -> StatusCode {
    //TODO implement (token blacklisting?)

    StatusCode::OK
}
