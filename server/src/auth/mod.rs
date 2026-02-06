use crate::auth::claims::Claims;
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
use chrono::{TimeDelta, Utc};
use crabdrive_common::user::UserId;
use jsonwebtoken::errors::ErrorKind::ExpiredSignature;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::log::warn;
use tracing::{debug, error};

pub mod claims;
pub mod secrets;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AuthError {
    Unauthorized,
    Expired,
    NoToken,
    ServerError,
    NoUser,
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
            AuthError::NoUser => "user does not exist",
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

        let claims =
            decode_bearer_token(bearer.token(), &state.keys.decoding_key).map_err(|err| {
                if err.kind().eq(&ExpiredSignature) {
                    AuthError::Expired
                } else {
                    AuthError::Unauthorized
                }
            })?;

        let user_entity_result = state.user_repository.get_user(claims.user_id);

        if let Err(err) = user_entity_result {
            error!(
                "auth: Could not authenticate user because of db error: {:?}",
                err
            );
            return Err(AuthError::ServerError);
        }

        let user_entity = user_entity_result.unwrap();

        if user_entity.is_none() {
            warn!("auth: a valid token was used but the referenced user does not exist");
            return Err(AuthError::NoUser);
        }

        let user_entity = user_entity.unwrap();

        debug!("auth: logged in user: {:?}", user_entity.username);
        Ok(user_entity)
    }
}

pub fn new_bearer_token(
    user_id: UserId,
    expires_in_seconds: i64,
    encoding_key: &EncodingKey,
) -> jsonwebtoken::errors::Result<String> {
    let time_delta = TimeDelta::new(expires_in_seconds, 0).unwrap();
    let expiry_time = (Utc::now() + time_delta).timestamp();

    let claims = Claims {
        user_id,
        exp: expiry_time,
    };

    let jwt = jsonwebtoken::encode(&Header::default(), &claims, encoding_key)?;

    Ok(jwt)
}

pub fn decode_bearer_token(
    token: &str,
    decoding_key: &DecodingKey,
) -> jsonwebtoken::errors::Result<Claims> {
    let token_data = decode::<Claims>(token, decoding_key, &Validation::default())?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod test {
    use crate::auth::{decode_bearer_token, new_bearer_token};
    use crabdrive_common::user::UserId;
    use jsonwebtoken::errors::ErrorKind::ExpiredSignature;
    use jsonwebtoken::{DecodingKey, EncodingKey};

    #[test]
    fn test_bearer_token() {
        let secret = "CLASSIFIED";

        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());

        let id = UserId::random();
        let token = new_bearer_token(id, 1, &encoding_key).unwrap();

        let claims = decode_bearer_token(&token, &decoding_key).unwrap();

        assert_eq!(claims.user_id, id)
    }

    #[test]
    fn test_bearer_token_expiry() {
        let secret = "CLASSIFIED";

        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());

        let id = UserId::random();

        // default settings allow for 60 seconds of "leeway" so our expiry must be at least 60 seconds in the past
        let token = new_bearer_token(id, -61, &encoding_key).unwrap();

        let claims_result_error: jsonwebtoken::errors::Error =
            decode_bearer_token(&token, &decoding_key).err().unwrap();

        assert_eq!(ExpiredSignature, claims_result_error.kind().clone());
    }
}
