use axum::{http::Request, middleware::Next, response::Response};
use std::time::Instant;

use axum::body::Body;
use axum::http::{HeaderName, HeaderValue};
use tracing::{debug, trace_span};

pub async fn logging_middleware(mut request: Request<Body>, next: Next) -> Response {
    let req_id = nanoid::nanoid!();
    // Check for an existing X-Request-ID header?
    // If a reverse proxy sets the same header, this would (currently) be overwritten.
    request.headers_mut().insert(
        HeaderName::from_static("x-request-id"),
        HeaderValue::from_str(&req_id).unwrap(),
    );

    let span = trace_span!(
        "request",
        method = %request.method(),
        uri = %request.uri(),
        id = req_id,
    );

    let _enter = span.enter();

    let response_time_start = Instant::now();
    let mut response = next.run(request).await;
    let response_time = Instant::now() - response_time_start;

    debug!(
        "[{}] Request took {}ms",
        response.status(),
        response_time.as_millis()
    );

    response.headers_mut().insert(
        HeaderName::from_static("x-request-id"),
        HeaderValue::from_str(&req_id).unwrap(),
    );

    response
}
