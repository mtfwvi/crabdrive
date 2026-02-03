use crate::auth::AuthError;
use crate::http::AppState;
use axum::RequestPartsExt;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use chrono::{NaiveDateTime, Utc};
use crabdrive_common::user::UserId;
use jsonwebtoken::{Validation, decode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: UserId,
    expires: NaiveDateTime,
}

// "slightly inspired" from https://github.com/tokio-rs/axum/blob/7961711fc73f2f5378f803715c8e5d2f546c8f27/examples/jwt/src/main.rs#L124-L142
impl FromRequestParts<AppState> for Claims {
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
            return Err(AuthError::Expired);
        }

        Ok(token_data.claims)
    }
}
