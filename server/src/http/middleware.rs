use axum::{http::Request, middleware::Next, response::Response};
use std::time::Instant;

use crate::http::state::AppState;
use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderName, HeaderValue};
use tracing::{debug, debug_span, info_span};

pub async fn logging_middleware(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    let req_id = nanoid::nanoid!();
    // Check for an existing X-Request-ID header?
    // If a reverse proxy sets the same header, this would (currently) be overwritten.
    request.headers_mut().insert(
        HeaderName::from_static("x-request-id"),
        HeaderValue::from_str(&req_id).unwrap(),
    );

    let span = match state.config.is_dev() {
        true => debug_span!(
            "request",
            method = %request.method(),
            uri = %request.uri(),
            id = req_id,
        ),
        false => info_span!(
            "request",
            req.method = %request.method(),
            req.uri = %request.uri(),
            req.id = req_id,
        ),
    };

    let _enter = span.enter();

    let start = Instant::now();
    let mut response = next.run(request).await;
    let lat = Instant::now() - start;

    debug!("[{}] Request took {}ms", response.status(), lat.as_millis());

    response.headers_mut().insert(
        HeaderName::from_static("x-request-id"),
        HeaderValue::from_str(&req_id).unwrap(),
    );

    response
}
