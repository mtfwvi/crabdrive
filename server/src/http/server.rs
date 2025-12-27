use crate::http::{Config, routes};
use std::io::ErrorKind;

use crate::http::middleware::logging_middleware;
use crate::http::state::AppState;
use axum::{Router, middleware};
use tracing::{error, info};

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    println!("Exiting...");
}

pub async fn serve(config: Config) -> Result<(), ()> {
    let state = AppState::new(config.clone());
    let app = Router::new()
        .with_state(state.clone())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            logging_middleware,
        ))
        .merge(routes::routes());

    let addr = config.addr();

    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => Ok(listener),
        Err(err) => {
            error!(
                "Failed to bind to {}. {}",
                addr,
                match err.kind() {
                    ErrorKind::AddrInUse => "The port is already in use!",
                    ErrorKind::PermissionDenied => "You do not have sufficient permissions!",
                    ErrorKind::AddrNotAvailable => "The requested IP is not available!",
                    _ => &*format!("{}", err),
                }
            );
            Err(())
        }
    }?;

    info!("Server running on http://{}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    Ok(())
}
