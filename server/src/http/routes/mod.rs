use axum::Router;
use axum::routing::get;

pub fn routes() -> Router {
    Router::new()
        // Add request handlers here
        .route("/", get(|| async { "Hello Crabdrive!" }))
}
