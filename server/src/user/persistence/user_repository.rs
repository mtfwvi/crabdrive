use crate::db::connection::DbPool;
use crate::db::operations::token::*;
use crate::db::operations::user::*;

use crate::user::auth::claims::Claims;
use crate::user::auth::secrets::Keys;

use crate::user::{BlacklistedTokenEntity, RefreshTokenEntity, SessionId, UserEntity};

use crabdrive_common::data::DataAmount;
use crabdrive_common::user::{UserId, UserKeys, UserType};
use crabdrive_common::uuid::UUID;

use std::sync::Arc;

use anyhow::{Context, Result};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher};
use argon2::{PasswordHash, PasswordVerifier};
use chrono::{DateTime, Local, TimeDelta, Utc};
use diesel::Connection;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use nanoid::nanoid;
use sha2::{Digest, Sha256};

const JWT_EXPIRY: i64 = 60 * 9;

type Jwt = String;
type RefreshToken = String;

pub(crate) trait UserRepository {
    /// Create a new user
    fn create_user(
        &self,
        username: &str,
        password: &str,
        storage_limit: DataAmount,
        keys: UserKeys,
    ) -> Result<UserEntity>;
    /// Get a user by ID
    fn get_user(&self, id: UserId) -> Result<Option<UserEntity>>;
    /// Validate the password hash of a user
    fn authenticate_user(&self, username: &str, password: &str) -> Result<Option<UserEntity>>;
    /// Get a user by their username
    fn get_user_by_username(&self, username: &str) -> Result<Option<UserEntity>>;
    /// Update a username
    fn update_user(&self, updated_entity: UserEntity) -> Result<UserEntity>;
    /// Hard-delete a user from the database
    fn delete_user(&self, id: UserId) -> Result<UserEntity>;
    /// Verify if a JWT is valid
    fn verify_jwt(&self, jwt: &str) -> Result<Option<UserEntity>>;
    /// Create a new session. This will create a new refresh token and JWT
    fn create_session(&self, user_id: UserId) -> Result<(RefreshToken, Jwt)>;
    /// Refresh a session with a refresh token. Returns `None` if the refresh token is invalid.
    fn refresh_session(&self, rtoken: &str) -> Result<Option<(RefreshToken, Jwt)>>;
    /// Invalidate session by ID. This will also blacklist the provided JWT.
    fn close_session(&self, jwt: &str) -> Result<()>;
}

pub struct UserRepositoryImpl {
    db_pool: Arc<DbPool>,
    secrets: Keys,
}

impl UserRepositoryImpl {
    pub fn new(db_pool: Arc<DbPool>, keys: Keys) -> Self {
        Self {
            db_pool,
            secrets: keys,
        }
    }
}

fn create_jwt(
    user_id: UserId,
    session_id: SessionId,
    expiry_time: i64,
    encoding_key: &EncodingKey,
) -> jsonwebtoken::errors::Result<String> {
    // The actual JWT has a short lifetime of 10 minutes. After expiry, the client needs to refresh the token
    // to get a new valid JWT. The argument is only different for tests.
    let lifetime = TimeDelta::new(expiry_time, 0).unwrap();
    let expiry_time = (Utc::now() + lifetime).timestamp();

    let claims = Claims {
        user_id,
        session_id,
        jti: nanoid!(),
        iat: Utc::now().timestamp(),
        exp: expiry_time,
    };

    let jwt = jsonwebtoken::encode(&Header::default(), &claims, encoding_key)?;
    Ok(jwt)
}

fn decode_jwt(claims: &str, decoding_key: &DecodingKey) -> jsonwebtoken::errors::Result<Claims> {
    let token_data = jsonwebtoken::decode::<Claims>(claims, decoding_key, &Validation::default())?;

    Ok(token_data.claims)
}

fn create_new_refresh_token(
    user_id: UserId,
    session_id: Option<SessionId>,
) -> (String, RefreshTokenEntity) {
    // Refresh tokens are valid for up to 7 days
    let lifetime = TimeDelta::new(60 * 60 * 24 * 7, 0).unwrap();
    let expiry_time = Local::now().naive_local() + lifetime;

    // Create a random ID for the refresh token
    let refresh_token = nanoid!(30);
    let refresh_token_hash = Sha256::digest(refresh_token.as_bytes()).to_vec();

    let tok = RefreshTokenEntity {
        token: refresh_token_hash,
        user_id,
        session_id: session_id.unwrap_or(UUID::random()),
        expires_at: expiry_time,
        invalidated_at: None,
    };

    (refresh_token, tok)
}

impl UserRepository for UserRepositoryImpl {
    fn create_user(
        &self,
        username: &str,
        password: &str,
        storage_limit: DataAmount,
        keys: UserKeys,
    ) -> Result<UserEntity> {
        let mut conn = self.db_pool.get()?;

        let password_salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &password_salt)
            .unwrap()
            .to_string();

        let user = UserEntity {
            user_type: UserType::User,
            id: UserId::random(),
            created_at: Utc::now().naive_utc(),
            username: username.to_string(),
            password_hash,
            storage_limit,
            // Currently unused. Maybe useful for admin routes.
            encryption_uninitialized: false,
            master_key: keys.master_key,
            private_key: keys.private_key,
            public_key: keys.public_key,
            root_key: keys.root_key,

            root_node: None,
            trash_key: keys.trash_key,
            trash_node: None,
        };

        insert_user(&mut conn, &user).context("Failed to insert user")?;
        Ok(user)
    }

    fn authenticate_user(&self, username: &str, password: &str) -> Result<Option<UserEntity>> {
        let Some(user) = self.get_user_by_username(username)? else {
            tracing::debug!("User not found");
            return Ok(None);
        };

        let parsed_hash = PasswordHash::new(&user.password_hash).expect("Failed to hash password!");
        if Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_err()
        {
            tracing::debug!("Wrong password!");
            return Ok(None);
        }

        Ok(Some(user))
    }

    fn get_user(&self, id: UserId) -> Result<Option<UserEntity>> {
        let mut conn = self.db_pool.get()?;
        select_user(&mut conn, id).context("Failed to select user")
    }

    fn get_user_by_username(&self, username: &str) -> Result<Option<UserEntity>> {
        let mut conn = self.db_pool.get()?;
        select_user_by_username(&mut conn, username).context("Failed to select user by username")
    }

    fn update_user(&self, updated_entity: UserEntity) -> Result<UserEntity> {
        let mut conn = self.db_pool.get()?;
        update_user(&mut conn, &updated_entity).context("Failed to update user")
    }

    fn delete_user(&self, id: UserId) -> Result<UserEntity> {
        let mut conn = self.db_pool.get()?;
        delete_user(&mut conn, id).context("Failed to delete user")
    }

    fn verify_jwt(&self, jwt: &str) -> Result<Option<UserEntity>> {
        let mut conn = self.db_pool.get()?;

        let claims = decode_jwt(jwt, &self.secrets.decoding_key)?;
        if selected_blacklisted_token(&mut conn, &claims.jti)?.is_some() {
            return Ok(None);
        }

        let user = self.get_user(claims.user_id)?.or_else(|| {
            tracing::warn!("Found valid JWT, but user {} is invalid!", claims.user_id);
            None
        });

        Ok(user)
    }

    fn create_session(&self, user_id: UserId) -> Result<(String, String)> {
        let mut conn = self.db_pool.get()?;

        let (raw_tok, tok) = create_new_refresh_token(user_id, None);
        let jwt = create_jwt(
            user_id,
            tok.session_id,
            JWT_EXPIRY,
            &self.secrets.encoding_key,
        )?;

        insert_refresh_token(&mut conn, &tok)?;

        Ok((raw_tok, jwt))
    }

    fn refresh_session(&self, refresh_token: &str) -> Result<Option<(String, String)>> {
        let mut conn = self.db_pool.get()?;

        let now = Local::now().naive_local();

        let refresh_token_hash = Sha256::digest(refresh_token.as_bytes()).to_vec();
        let r_tok = select_refresh_token(&mut conn, refresh_token_hash)?;
        if r_tok.is_none() {
            tracing::debug!("No matching refresh token found in DB!");
            return Ok(None);
        }
        let r_tok = r_tok.unwrap();

        if now >= r_tok.expires_at {
            tracing::warn!("Already expired refresh token");
            return Ok(None);
        }

        // Check expiry time; Allows for 10 seconds of leeway. After this, using a token is considered abuse, and
        // all active sessions will be revoked.
        if let Some(invalidated_at) = r_tok.invalidated_at {
            let grace_period = invalidated_at + TimeDelta::try_seconds(10).unwrap();

            if now >= grace_period {
                // The token has been refreshed after the grace period. Nuke all sessions.
                tracing::warn!(
                    "Attempted refresh on already expired token (Session: {})",
                    r_tok.session_id
                );
                invalidate_token_family(&mut conn, r_tok.session_id, now)?;
                anyhow::bail!("Unauthorized");
            }
        }

        let (raw_new_r_tok, new_r_tok) =
            create_new_refresh_token(r_tok.user_id, Some(r_tok.session_id));
        let new_jwt = create_jwt(
            r_tok.user_id,
            r_tok.session_id,
            JWT_EXPIRY,
            &self.secrets.encoding_key,
        )?;

        conn.transaction(|conn| {
            // Invalidate all tokens, which are currently valid
            invalidate_refresh_token(conn, r_tok.session_id, now)?;
            insert_refresh_token(conn, &new_r_tok)?;
            Ok::<(), anyhow::Error>(())
        })?;

        Ok(Some((raw_new_r_tok, new_jwt)))
    }

    fn close_session(&self, jwt: &str) -> Result<()> {
        let mut conn = self.db_pool.get()?;

        let jwt = decode_jwt(jwt, &self.secrets.decoding_key)?;
        if selected_blacklisted_token(&mut conn, &jwt.jti)?.is_some() {
            anyhow::bail!("Token already blacklisted");
        }

        conn.transaction(|conn| {
            // Blacklist the JWT
            insert_blacklisted_token(
                conn,
                &BlacklistedTokenEntity {
                    id: jwt.jti,
                    expires_at: DateTime::from_timestamp(jwt.exp, 0).unwrap().naive_local(),
                },
            )?;
            // Invalidate the session
            invalidate_token_family(conn, jwt.session_id, Local::now().naive_local())?;

            Ok::<(), anyhow::Error>(())
        })
    }
}

#[cfg(test)]
mod test {
    use crate::user::SessionId;

    use super::{create_jwt, decode_jwt};
    use crabdrive_common::user::UserId;
    use jsonwebtoken::errors::ErrorKind::ExpiredSignature;
    use jsonwebtoken::{DecodingKey, EncodingKey};

    #[test]
    fn test_bearer_token() {
        let secret = "CLASSIFIED";

        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());

        let id = UserId::random();
        let session_id = SessionId::random();
        let token = create_jwt(id, session_id, 60, &encoding_key).unwrap();

        let claims = decode_jwt(&token, &decoding_key).unwrap();

        assert_eq!(claims.user_id, id);
        assert_eq!(claims.session_id, session_id);
    }

    #[test]
    fn test_bearer_token_expiry() {
        let secret = "CLASSIFIED";

        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());

        let id = UserId::random();

        // default settings allow for 60 seconds of "leeway" so our expiry must be at least 60 seconds in the past
        let token = create_jwt(id, SessionId::nil(), -61, &encoding_key).unwrap();

        let claims_result_error: jsonwebtoken::errors::Error =
            decode_jwt(&token, &decoding_key).err().unwrap();

        assert_eq!(ExpiredSignature, claims_result_error.kind().clone());
    }
}
