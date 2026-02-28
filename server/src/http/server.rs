use crate::db::operations;
use crate::http::middleware::logging_middleware;
use crate::http::{AppConfig, AppState, routes};
use http_body_util::Full;

use axum::http::StatusCode;
use axum::http::header::{self, AUTHORIZATION, CONTENT_TYPE};
use axum::response::Response;
use axum::{Router, middleware};
use bytes::Bytes;
use std::any::Any;
use std::io::ErrorKind;
use tokio::time::Duration;
use tokio::{task, time};
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::cors::CorsLayer;
use tracing::{error, info};

async fn graceful_shutdown(state: AppState) {
    let _ = tokio::signal::ctrl_c().await;
    shutdown(state).await;
}

pub fn create_app(config: AppConfig) -> (Router, AppState) {
    let state = AppState::new(config);

    let cors = CorsLayer::new() // TODO: Make more specific before submission
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let router = routes::routes()
        .with_state(state.clone())
        .layer(middleware::from_fn(logging_middleware))
        .layer(CatchPanicLayer::custom(handle_panic))
        .layer(cors);

    (router, state)
}

pub async fn start(config: AppConfig) -> Result<(), ()> {
    let (app, state) = create_app(config.clone());
    let db_pool = state.db_pool.clone();

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

    task::spawn(async move {
        let mut duration = time::interval(Duration::from_secs(60 * 15));
        loop {
            duration.tick().await;
            tracing::info!("Removing expired tokens from blacklist");
            let mut conn = match db_pool.get() {
                Ok(conn) => conn,
                Err(e) => {
                    tracing::error!("Unable to remove expired tokens: {e}");
                    break;
                }
            };
            let now = chrono::Local::now().naive_utc();
            // Ignore on error to prevent server crash
            let count = operations::token::delete_expired_blacklisted_tokens(&mut conn, now)
                .inspect_err(|e| {
                    tracing::error!("Unable to remove expired tokens: {e}");
                })
                .ok()
                .unwrap_or(0);

            tracing::info!("Removed {count} tokens from blacklist!");
        }
    });

    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_shutdown(state.clone()))
        .await
        .unwrap();

    Ok(())
}

async fn shutdown(_state: AppState) {
    info!("Stopping server");
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
