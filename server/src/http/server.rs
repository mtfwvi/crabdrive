use crate::http::middleware::logging_middleware;
use crate::http::{AppConfig, AppState, routes};


use axum::http::StatusCode;
use axum::http::header::{self, AUTHORIZATION, CONTENT_TYPE};
use axum::{Router, middleware};
use axum::response::Response;
use bytes::Bytes;
use http_body_util::Full;
use std::any::Any;
use std::io::ErrorKind;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::cors::CorsLayer;

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

    let addr = config.addr();

    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => Ok(listener),
        Err(err) => {
            tracing::error!(
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

    tracing::info!("Server running on http://{}", &addr);

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
        "Unknown panic message".to_string()
    };

    tracing::error!("Request handler panicked!: {:?}", details);

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
