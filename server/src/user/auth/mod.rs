use crate::http::AppState;
use crate::user::persistence::model::user_entity::UserEntity;
use anyhow::Result;
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum::{Json, RequestPartsExt};
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::debug;

pub mod claims;
pub mod secrets;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AuthError {
    Unauthorized,
    Expired,
    NoToken,
    ServerError,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let error_message = match self {
            AuthError::Unauthorized => {
                "Access denied, this incident will be reported. Watch out for the white van below your window!"
            }
            AuthError::NoToken => "Missing token",
            AuthError::Expired => "Expired token",
            AuthError::ServerError => "Internal server error",
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (StatusCode::UNAUTHORIZED, body).into_response()
    }
}

impl FromRequestParts<AppState> for UserEntity {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::NoToken)?;

        let user = state
            .user_repository
            .verify_jwt(bearer.token())
            .map_err(|_| AuthError::ServerError)?
            .ok_or(AuthError::Unauthorized)?;

        debug!("Authenticated user: {}:{}", user.username, user.id);

        Ok(user)
    }
}
