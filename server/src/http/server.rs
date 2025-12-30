use crate::http::{Config, routes};
use std::io::ErrorKind;

use crate::http::middleware::logging_middleware;
use crate::http::state::AppState;
use axum::{Router, middleware};
use tracing::{error, info};

async fn graceful_shutdown(state: AppState) {
    let _ = tokio::signal::ctrl_c().await;
    shutdown(state).await;
}

#[allow(unused_variables)] // TODO: Remove when state is actually used
async fn shutdown(state: AppState) {
    info!("Stopping server");
}

pub async fn start(config: Config) -> Result<(), ()> {
    let state = AppState::new(config.clone());

    let app = Router::<AppState>::new()
        .with_state(state.clone())
        .merge(routes::routes())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            logging_middleware,
        ));

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
        .with_graceful_shutdown(graceful_shutdown(state.clone()))
        .await
        .unwrap();

    Ok(())
}
