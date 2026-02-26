use crate::db::connection::create_pool;
use crate::http::middleware::logging_middleware;
use crate::http::{AppConfig, AppState, routes};
use crate::storage::node::persistence::node_repository::NodeState;
use crate::storage::revision::persistence::revision_repository::RevisionService;
use crate::storage::vfs::backend::Sfs;

use http_body_util::Full;

use crate::user::auth::secrets::Keys;
use crate::storage::share::persistence::share_repository::ShareRepositoryImpl;
use crate::user::persistence::user_repository::UserState;
use axum::http::StatusCode;
use axum::http::header::{self, AUTHORIZATION, CONTENT_TYPE};
use axum::middleware;
use axum::response::Response;
use bytes::Bytes;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use std::any::Any;
use std::io::ErrorKind;
use std::sync::Arc;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::cors::CorsLayer;
use tracing::{error, info};

async fn graceful_shutdown(state: AppState) {
    let _ = tokio::signal::ctrl_c().await;
    shutdown(state).await;
}

async fn shutdown(_state: AppState) {
    info!("Stopping server");
}

pub async fn start(config: AppConfig) -> Result<(), ()> {
    let pool = create_pool(&config.db.path, config.db.pool_size);

    let vfs = Sfs::new(&config.storage.dir);

    let node_repository = NodeState::new(Arc::new(pool.clone()));
    let revision_repository = RevisionService::new(Arc::new(pool.clone()));
    let user_repository = UserState::new(Arc::new(pool.clone()));
    let share_repository = ShareRepositoryImpl::new(Arc::new(pool.clone()));

    let keys = Keys::new(&config.auth.jwt_secret);

    let state = AppState::new(
        config.clone(),
        pool,
        vfs,
        node_repository,
        revision_repository,
        user_repository,
        share_repository,
        keys,
    );

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./res/migrations/");
    {
        let mut conn = state.db_pool.get().unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();
    }

    let cors = CorsLayer::new() // TODO: Make more specific before submission
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let app = routes::routes()
        .with_state(state.clone())
        .layer(middleware::from_fn(logging_middleware))
        .layer(CatchPanicLayer::custom(handle_panic))
        .layer(cors);

    let addr = config.addr();

    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => Ok(listener),
        Err(err) => {
            error!(
                "Failed to bind to {}. {}",
                addr,
                match err.kind() {
                    ErrorKind::AddrInUse => "The port is already in use!".to_string(),
                    ErrorKind::PermissionDenied =>
                        "You do not have sufficient permissions!".to_string(),
                    ErrorKind::AddrNotAvailable => "The requested IP is not available!".to_string(),
                    _ => format!("{}", err),
                }
            );
            Err(())
        }
    }?;

    info!("Server running on http://{}", &addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_shutdown(state.clone()))
        .await
        .unwrap();

    Ok(())
}

// copied from here: https://docs.rs/tower-http/latest/tower_http/catch_panic/index.html
pub(crate) fn handle_panic(err: Box<dyn Any + Send + 'static>) -> Response<Full<Bytes>> {
    let details = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown panic message".to_string()
    };

    error!("panic: {:?}", details);

    let body = serde_json::json!({
        "error": {
            "kind": "panic",
            "details": details,
        }
    });
    let body = serde_json::to_string(&body).unwrap();

    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Full::from(body))
        .unwrap()
}
