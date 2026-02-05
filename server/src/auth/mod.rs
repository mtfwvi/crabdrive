use axum::extract::{FromRequestParts};
use axum::http::request::Parts;
use axum::{Json, RequestPartsExt};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use axum_extra::TypedHeader;
use chrono::Utc;
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, error};
use tracing::log::warn;
use crate::auth::claims::Claims;
use crate::http::AppState;
use crate::user::persistence::model::user_entity::UserEntity;

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
                "Access denied, this incident will be reported. Watch out for the white van below you window"
            }
            AuthError::NoToken => "missing token",
            AuthError::Expired => "expired token",
            AuthError::ServerError => "server error during authentication",
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (StatusCode::UNAUTHORIZED, body).into_response()
    }
}

// "slightly inspired" from https://github.com/tokio-rs/axum/blob/7961711fc73f2f5378f803715c8e5d2f546c8f27/examples/jwt/src/main.rs#L124-L142
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

        let token_data = decode::<Claims>(
            bearer.token(),
            &state.keys.decoding_key,
            &Validation::default(),
        )
            .map_err(|_| AuthError::Unauthorized)?;

        if Utc::now().naive_utc() >= token_data.claims.expires {
            debug!("auth: expired token");
            return Err(AuthError::Expired);
        }

        let user_entity_result = state.user_repository.get_user(token_data.claims.user_id);

        if let Err(err) = user_entity_result {
            error!("auth: Could not authenticate user because of db error: {:?}", err);
            return Err(AuthError::ServerError)
        }

        let user_entity = user_entity_result.unwrap();

        if user_entity.is_none() {
            warn!("auth: a valid token was used but the referenced user does not exist");
            return Err(AuthError::Unauthorized);
        }

        let user_entity = user_entity.unwrap();

        debug!("auth: logged in user: {:?}", user_entity.username);
        Ok(user_entity)
    }
}
