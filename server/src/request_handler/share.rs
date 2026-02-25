use crate::http::AppState;
use crate::request_handler::node::entity_to_encrypted_node;
use crate::storage::share::persistence::model::share_entity::ShareEntity;
use crate::user::persistence::model::user_entity::UserEntity;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use chrono::Utc;
use crabdrive_common::encryption_key::EncryptionKey;
use crabdrive_common::payloads::node::request::share::{
    PostAcceptShareRequest, PostShareNodeRequest,
};
use crabdrive_common::payloads::node::response::share::{
    GetAcceptShareInfoResponse, GetAcceptedSharedResponse, GetNodeShareInfo,
    PostAcceptShareResponse, PostShareNodeResponse, ShareEncryptionInfo,
};
use crabdrive_common::storage::{EncryptedNode, NodeId, ShareId};
use tracing::error;

pub async fn post_share_node(
    current_user: UserEntity,
    State(state): State<AppState>,
    Path(node_id): Path<NodeId>,
    Json(payload): Json<PostShareNodeRequest>,
) -> (StatusCode, Json<PostShareNodeResponse>) {
    let node = state.node_repository.get_node(node_id).expect("db error");

    if node.is_none() {
        return (StatusCode::NOT_FOUND, Json(PostShareNodeResponse::NotFound));
    }

    let node = node.unwrap();

    if node.owner_id != current_user.id {
        return (
            StatusCode::BAD_REQUEST,
            Json(PostShareNodeResponse::BadRequest(
                "cannot share a file that you do not own".to_string(),
            )),
        );
    }

    if node.parent_id.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(PostShareNodeResponse::BadRequest(
                "Cannot share a root node".to_string(),
            )),
        );
    }

    if node.deleted_on.is_some() {
        return (
            StatusCode::BAD_REQUEST,
            Json(PostShareNodeResponse::BadRequest(
                "Cannot share a node that is in the trash".to_string(),
            )),
        );
    }

    let share_entity = ShareEntity {
        id: NodeId::random(),
        node_id: node.id,
        shared_by: current_user.id,
        accepted_by: None,
        time_shared: Utc::now().naive_utc(),
        time_accepted: None,
        shared_encryption_key: Some(payload.wrapped_metadata_key),
        accepted_encryption_key: None,
    };

    let share_entity = state
        .share_repository
        .insert_share(share_entity)
        .expect("db error");

    (
        StatusCode::OK,
        Json(PostShareNodeResponse::Ok(share_entity.id)),
    )
}

pub async fn get_accept_share_info(
    _current_user: UserEntity,
    State(state): State<AppState>,
    Path(share_id): Path<ShareId>,
) -> (StatusCode, Json<GetAcceptShareInfoResponse>) {
    let share_entity = state
        .share_repository
        .get_share(share_id)
        .expect("db error");

    if share_entity.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(GetAcceptShareInfoResponse::NotFound),
        );
    }

    let share_entity = share_entity.unwrap();

    if share_entity.accepted_by.is_some() {
        return (
            StatusCode::NOT_FOUND,
            Json(GetAcceptShareInfoResponse::NotFound),
        );
    }

    let response = GetAcceptShareInfoResponse::Ok(ShareEncryptionInfo {
        node_id: share_entity.node_id,
        wrapped_metadata_key: share_entity
            .shared_encryption_key
            .expect("share entity that has not been accepted is missing encryption key"),
    });

    (StatusCode::OK, Json(response))
}

pub async fn get_node_share_info(
    current_user: UserEntity,
    State(state): State<AppState>,
    Path(node_id): Path<NodeId>,
) -> (StatusCode, Json<GetNodeShareInfo>) {
    if !state
        .node_repository
        .has_access(node_id, current_user.id)
        .expect("db error")
    {
        return (StatusCode::NOT_FOUND, Json(GetNodeShareInfo::NotFound));
    }

    let Some(share_entity) = state
        .share_repository
        .get_share_by_node_id_and_user_id(node_id, current_user.id)
        .expect("db error")
    else {
        return (StatusCode::NOT_FOUND, Json(GetNodeShareInfo::NotFound));
    };

    let Some(wrapped_key) = share_entity.accepted_encryption_key else {
        error!("A user has accepted a share but there is not key in the db???????");
        return (StatusCode::NOT_FOUND, Json(GetNodeShareInfo::NotFound));
    };

    (
        StatusCode::OK,
        Json(GetNodeShareInfo::Ok(ShareEncryptionInfo {
            node_id,
            wrapped_metadata_key: wrapped_key,
        })),
    )
}

pub async fn post_accept_share(
    current_user: UserEntity,
    State(state): State<AppState>,
    Path(share_id): Path<ShareId>,
    Json(payload): Json<PostAcceptShareRequest>,
) -> (StatusCode, Json<PostAcceptShareResponse>) {
    let Some(mut share_entity) = state
        .share_repository
        .get_share(share_id)
        .expect("db error")
    else {
        return (
            StatusCode::NOT_FOUND,
            Json(PostAcceptShareResponse::NotFound),
        );
    };

    // cannot accept a share that is already accessible (owned/ access to parent)
    if state
        .node_repository
        .has_access(share_entity.node_id, current_user.id)
        .expect("db error")
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(PostAcceptShareResponse::BadRequest(
                "share is already accessible".to_string(),
            )),
        );
    }

    if share_entity.accepted_by.is_some() {
        return (
            StatusCode::NOT_FOUND,
            Json(PostAcceptShareResponse::NotFound),
        );
    }

    share_entity.accepted_by = Some(current_user.id);
    share_entity.time_accepted = Some(Utc::now().naive_utc());
    share_entity.accepted_encryption_key = Some(payload.new_wrapped_metadata_key);

    // this key is not required anymore and should be deleted
    share_entity.shared_encryption_key = None;

    state
        .share_repository
        .update_share(share_entity)
        .expect("db error");

    (StatusCode::OK, Json(PostAcceptShareResponse::Ok))
}

pub async fn get_accepted_shared_nodes(
    current_user: UserEntity,
    State(state): State<AppState>,
) -> (StatusCode, Json<GetAcceptedSharedResponse>) {
    let accepted_shares = state
        .share_repository
        .get_shares_by_user(current_user.id)
        .expect("db error");

    let nodes: Vec<(EncryptionKey, EncryptedNode)> = accepted_shares
        .iter()
        .map(|share_entity| {
            let node = state
                .node_repository
                .get_node(share_entity.node_id)
                .expect("db error");
            if node.is_none() {
                unreachable!("violating db constraints");
            }

            if share_entity.accepted_encryption_key.is_none() {
                unreachable!("a node is marked as accepted but there is no key")
            }

            let node = entity_to_encrypted_node(node.unwrap(), &state).expect("db error");

            (share_entity.accepted_encryption_key.clone().unwrap(), node)
        })
        .collect();

    (StatusCode::OK, Json(GetAcceptedSharedResponse::Ok(nodes)))
}
