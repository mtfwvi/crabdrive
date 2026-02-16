use crate::http::AppState;
use crate::user::persistence::model::user_entity::UserEntity;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use crabdrive_common::payloads::node::request::share::PostShareNodeRequest;
use crabdrive_common::payloads::node::response::share::PostShareNodeResponse;
use crabdrive_common::storage::NodeId;

pub fn post_share_node(
    current_user: UserEntity,
    State(state): State<AppState>,
    Path(node_id): Path<NodeId>,
    Json(payload): Json<PostShareNodeRequest>
) -> (StatusCode, Json<PostShareNodeResponse>) {
    let node = state.node_repository.get_node(node_id).expect("db error");

    if node.is_none() {
        return (StatusCode::NOT_FOUND, Json(PostShareNodeResponse::NotFound))
    }

    let node = node.unwrap();

    if node.owner_id != current_user.id {
        return (StatusCode::NOT_FOUND, Json(PostShareNodeResponse::NotFound))
    }

    todo!()
}
