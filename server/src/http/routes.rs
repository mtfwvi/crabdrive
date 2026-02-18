use crate::http::AppState;

use crate::request_handler::admin::*;
use crate::request_handler::auth::*;
use crate::request_handler::chunk::*;
use crate::request_handler::file::*;
use crate::request_handler::folder::*;
use crate::request_handler::node::*;

use crabdrive_common::da;
use crabdrive_common::routes;

use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::{delete, get, post};
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};
use crate::request_handler::share::{get_accepted_shared_nodes, get_node_shared_with, get_share_info, post_accept_share, post_share_node};

pub fn routes() -> Router<AppState> {
    let frontend_build =
        ServeDir::new("./client/dist").fallback(ServeFile::new("./client/dist/index.html"));

    Router::new()
        .fallback_service(frontend_build)
        .layer(CompressionLayer::new())
        .merge(nodes_routes())
        .merge(admin_routes())
        .merge(auth_routes())
}

pub fn nodes_routes() -> Router<AppState> {
    Router::new()
        .route(
            routes::node::ROUTE_BY_ID,
            get(get_node).patch(patch_node).delete(delete_node),
        )
        .route(routes::node::ROUTE_MOVE, post(post_move_node))
        .route(
            routes::node::ROUTE_MOVE_TO_TRASH,
            delete(post_move_node_to_trash),
        )
        .route(
            routes::node::ROUTE_MOVE_OUT_OF_TRASH,
            post(post_move_node_out_of_trash),
        )
        .route(routes::node::file::ROUTE_CREATE, post(post_create_file))
        .route(routes::node::file::ROUTE_UPDATE, post(post_update_file))
        .route(routes::node::file::ROUTE_COMMIT, post(post_commit_file))
        .route(routes::node::folder::ROUTE_CREATE, post(post_create_folder))
        .route(routes::node::ROUTE_CHILDREN, get(get_node_children))
        .route(
            routes::node::ROUTE_PATH_BETWEEN,
            get(get_path_between_nodes),
        )
        .route(routes::node::ROUTE_VERSIONS, get(get_file_versions))
        .route(
            routes::node::ROUTE_CHUNKS,
            post(post_chunk)
                .layer(DefaultBodyLimit::max(da!(18 MB).as_bytes() as usize))
                .get(get_chunk),
        )
}

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route(routes::auth::ROUTE_LOGIN, post(post_login))
        .route(routes::auth::ROUTE_REGISTER, post(post_register))
        .route(routes::auth::ROUTE_LOGOUT, post(post_logout))
        .route(routes::auth::ROUTE_INFO, get(get_user_info))
}

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route(
            routes::admin::ROUTE_USER_BY_ID,
            get(get_user).delete(delete_user),
        )
        .route(routes::admin::ROUTE_USER, post(post_user))
}

pub fn share_routes() -> Router<AppState> {
    Router::new()
        .route(routes::node::share::SHARE_NODE, post(post_share_node))
        .route(routes::node::share::NODE_SHARE_INFO, get(get_node_shared_with))
        .route(routes::node::share::GET_SHARE_INFO, get(get_share_info))
        .route(routes::node::share::GET_ACCEPTED_SHARED, get(get_accepted_shared_nodes))
        .route(routes::node::share::ACCEPT_SHARE, post(post_accept_share))
}
