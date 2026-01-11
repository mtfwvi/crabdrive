use crate::storage::chunk_request_handler::{get_chunk, post_chunk};
use crate::storage::file_request_handler::{
    get_file_versions, post_commit_file, post_create_file, post_update_file,
};
use crate::storage::folder_request_handler::post_create_folder;
use crate::storage::node_request_handler::{
    delete_node, get_node, get_node_children, get_path_between_nodes, patch_node, post_move_node,
    post_move_node_out_of_trash, post_move_node_to_trash,
};
use crate::user::admin_request_handler::{delete_user, get_user, post_user};
use crate::user::auth_request_handler::{post_login, post_logout, post_register};
use axum::Router;
use axum::routing::{delete, get, post};

pub fn routes() -> Router {
    Router::new()
        // Add request handlers here
        .route("/", get(|| async { "Hello Crabdrive!" }))
        .merge(nodes_routes())
        .merge(admin_routes())
        .merge(auth_routes())
}

pub fn nodes_routes() -> Router {
    Router::new()
        .route(
            "/node/{nodeId}",
            get(get_node).patch(patch_node).delete(delete_node),
        )
        .route("/node/{nodeId}/move", post(post_move_node))
        .route(
            "/node/{nodeId}/move_to_trash",
            delete(post_move_node_to_trash),
        )
        .route(
            "/node/{nodeId}/move_out_of_trash",
            post(post_move_node_out_of_trash),
        )
        .route("/node/{parentId}/create_file", post(post_create_file))
        .route("/node/{nodeId}/update_file", post(post_update_file))
        .route(
            "/node/{nodeId}/versions/{versionId}/commit",
            post(post_commit_file),
        )
        .route("/node/{parentId}/create_folder", post(post_create_folder))
        .route("/node/{parentId}/children", get(get_node_children))
        .route("/path_between_nodes", get(get_path_between_nodes))
        .route("/node/{nodeId}/versions", get(get_file_versions))
        .route(
            "/node/{nodeId}/versions/{versionId}/chunks/{chunkIndex}",
            post(post_chunk).get(get_chunk),
        )
}

pub fn auth_routes() -> Router {
    Router::new()
        .route("/auth/login", post(post_login))
        .route("/auth/register", post(post_register))
        .route("/auth/logout", post(post_logout))
}

pub fn admin_routes() -> Router {
    Router::new()
        .route("/admin/user/{userId}", get(get_user).delete(delete_user))
        .route("/admin/user", post(post_user))
}
