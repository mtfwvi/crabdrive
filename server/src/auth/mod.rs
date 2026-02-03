use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub mod claims;
pub mod secrets;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AuthError {
    Unauthorized,
    Expired,
    NoToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let error_message = match self {
            AuthError::Unauthorized => {
                "Access denied, this incident will be reported. Watch out for the white van below you window"
            }
            AuthError::NoToken => "missing token",
            AuthError::Expired => "expired token",
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (StatusCode::UNAUTHORIZED, body).into_response()
    }
}
