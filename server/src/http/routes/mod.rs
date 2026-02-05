use crate::http::AppState;
use crate::request_handler::admin::{delete_user, get_user, post_user};
use crate::request_handler::auth::{get_user_info, post_login, post_logout, post_register};
use crate::request_handler::chunk::{get_chunk, post_chunk};
use crate::request_handler::file::{
    get_file_versions, post_commit_file, post_create_file, post_update_file,
};
use crate::request_handler::folder::post_create_folder;
use crate::request_handler::node::{
    delete_node, get_node, get_node_children, get_path_between_nodes, patch_node, post_move_node,
    post_move_node_out_of_trash, post_move_node_to_trash,
};
use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::{delete, get, post};
use crabdrive_common::da;
use crabdrive_common::routes::*;
use formatx::formatx;

pub fn routes() -> Router<AppState> {
    Router::new()
        // Add request handlers here
        .route("/", get(|| async { "Hello Crabdrive!" }))
        .merge(nodes_routes())
        .merge(admin_routes())
        .merge(auth_routes())
}

pub fn nodes_routes() -> Router<AppState> {
    Router::new()
        .route(
            &formatx!(NODE_ROUTE_NODEID, "{nodeId}").unwrap(),
            get(get_node).patch(patch_node).delete(delete_node),
        )
        .route(
            &formatx!(MOVE_NODE_ROUTE, "{nodeId}").unwrap(),
            post(post_move_node),
        )
        .route(
            &formatx!(MOVE_NODE_TO_TRASH_ROUTE, "{nodeId}").unwrap(),
            delete(post_move_node_to_trash),
        )
        .route(
            &formatx!(MOVE_NODE_OUT_OF_TASH_ROUTE, "{nodeId}").unwrap(),
            post(post_move_node_out_of_trash),
        )
        .route(
            &formatx!(CREATE_FILE_ROUTE, "{parentId}").unwrap(),
            post(post_create_file),
        )
        .route(
            &formatx!(UPDATE_FILE_ROUTE, "{fileId}").unwrap(),
            post(post_update_file),
        )
        .route(
            &formatx!(COMMIT_FILE_ROUTE, "{nodeId}", "{versionId}").unwrap(),
            post(post_commit_file),
        )
        .route(
            &formatx!(CREATE_FOLDER_ROUTE, "{parentId}").unwrap(),
            post(post_create_folder),
        )
        .route(
            &formatx!(CHILDREN_ROUTE, "{parentId}").unwrap(),
            get(get_node_children),
        )
        .route(
            &formatx!(PATH_BETWEEN_NODES_ROUTE).unwrap(),
            get(get_path_between_nodes),
        )
        .route(
            &formatx!(NODE_VERSIONS_ROUTE, "{nodeId}").unwrap(),
            get(get_file_versions),
        )
        .route(
            &formatx!(CHUNK_ROUTE, "{nodeId}", "{versionId}", "{chunkIndex}").unwrap(),
            post(post_chunk)
                .layer(DefaultBodyLimit::max(da!(18 MB).as_bytes() as usize))
                .get(get_chunk),
        )
}

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route(LOGIN_ROUTE, post(post_login))
        .route(REGISTER_ROUTE, post(post_register))
        .route(LOGOUT_ROUTE, post(post_logout))
        .route(USER_INFO_ROUTE, get(get_user_info))
}

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route(
            &formatx!(ADMIN_USER_ROUTE_ID, "{userId}").unwrap(),
            get(get_user).delete(delete_user),
        )
        .route(ADMIN_USER_ROUTE, post(post_user))
}
