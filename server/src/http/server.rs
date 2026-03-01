use crate::db::operations;
use crate::http::middleware::logging_middleware;
use crate::http::{AppConfig, AppState, routes};


use axum::http::StatusCode;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::{Router, middleware};
use axum::response::Response;
use bytes::Bytes;
use http_body_util::Full;
use tokio::{task, time};
use std::any::Any;
use std::io::ErrorKind;
use std::time::Duration;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::cors::CorsLayer;
use tracing::{error, info};

async fn graceful_shutdown(state: AppState) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    shutdown(state).await;
}

pub async fn create_app(config: AppConfig) -> (Router, AppState) {
    let state = AppState::new(config).await;

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
    let (app, state) = create_app(config.clone()).await;
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
            info!("Removing expired tokens from blacklist");
            let mut conn = match db_pool.get() {
                Ok(conn) => conn,
                Err(e) => {
                    error!("Unable to remove expired tokens: {e}");
                    break;
                }
            };
            let now = chrono::Local::now().naive_utc();
            // Ignore on error to prevent server crash
            let count = operations::token::delete_expired_blacklisted_tokens(&mut conn, now)
                .inspect_err(|e| {
                    error!("Unable to remove expired tokens: {e}");
                })
                .ok()
                .unwrap_or(0);

            info!("Removed {count} tokens from blacklist!");
        }
    });

    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_shutdown(state.clone()))
        .await
        .unwrap();

    Ok(())
}

async fn shutdown(_state: AppState) {
    tracing::info!("Stopping server");
}

// copied from here: https://docs.rs/tower-http/latest/tower_http/catch_panic/index.html
pub(crate) fn handle_panic(err: Box<dyn Any + Send + 'static>) -> Response<Full<Bytes>> {
    let details = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "[Unknown panic message]".to_string()
    };

    error!("Request handler panicked: {:?}", details);

    let client_details = if cfg!(debug_assertions) {
        details
    } else {
        "Internal Server Error".to_string()
    };

    let body = serde_json::json!({
        "error": {
            "kind": "panic",
            "details": client_details,
        }
    });
    let body = serde_json::to_string(&body).unwrap();

    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(CONTENT_TYPE, "application/json")
        .body(Full::from(body))
        .unwrap()
}
