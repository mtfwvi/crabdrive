use crate::db::{RefreshTokenDsl, TokenBlacklistDsl};
use crate::user::{BlacklistedTokenEntity, RefreshTokenEntity, SessionId};

use anyhow::Result;
use chrono::NaiveDateTime;
use diesel::{
    Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    SqliteConnection,
};
use tracing::instrument;

#[instrument(skip(conn), err)]
pub fn insert_refresh_token(
    conn: &mut SqliteConnection,
    token: &RefreshTokenEntity,
) -> Result<RefreshTokenEntity> {
    conn.transaction(|conn| {
        let new_token = diesel::insert_into(RefreshTokenDsl::RefreshToken)
            .values(token)
            .returning(RefreshTokenEntity::as_select())
            .get_result(conn)?;
        Ok(new_token)
    })
}

#[instrument(skip(conn), err)]
pub fn select_refresh_token(
    conn: &mut SqliteConnection,
    token: Vec<u8>,
) -> Result<Option<RefreshTokenEntity>> {
    conn.transaction(|conn| {
        let token = RefreshTokenDsl::RefreshToken
            .filter(RefreshTokenDsl::token.eq(token))
            .first::<RefreshTokenEntity>(conn)
            .optional()?;
        Ok(token)
    })
}

#[instrument(skip(conn), err)]
pub fn invalidate_refresh_token(
    conn: &mut SqliteConnection,
    session_id: SessionId,
    invalidated_at_time: NaiveDateTime,
) -> Result<RefreshTokenEntity> {
    conn.transaction(|conn| {
        let token = diesel::update(RefreshTokenDsl::RefreshToken)
            .filter(RefreshTokenDsl::session_id.eq(session_id))
            .filter(RefreshTokenDsl::invalidated_at.is_null())
            .set(RefreshTokenDsl::invalidated_at.eq(Some(invalidated_at_time)))
            .returning(RefreshTokenEntity::as_select())
            .get_result(conn)?;
        Ok(token)
    })
}

#[instrument(skip(conn), err)]
pub fn invalidate_token_family(
    conn: &mut SqliteConnection,
    family_id: SessionId,
    invalidated_at_time: NaiveDateTime,
) -> Result<usize> {
    conn.transaction(|conn| {
        // Return only the count for logs, the tokens are not required anymore
        let deleted_count = diesel::update(RefreshTokenDsl::RefreshToken)
            .filter(RefreshTokenDsl::session_id.eq(family_id))
            .filter(RefreshTokenDsl::invalidated_at.is_null())
            .set(RefreshTokenDsl::invalidated_at.eq(Some(invalidated_at_time)))
            .execute(conn)?;
        Ok(deleted_count)
    })
}

#[instrument(skip(conn), err)]
pub fn selected_blacklisted_token(
    conn: &mut SqliteConnection,
    id: &str,
) -> Result<Option<BlacklistedTokenEntity>> {
    conn.transaction(|conn| {
        let token = TokenBlacklistDsl::TokenBlacklist
            .filter(TokenBlacklistDsl::id.eq(id))
            .first::<BlacklistedTokenEntity>(conn)
            .optional()?;
        Ok(token)
    })
}

#[instrument(skip(conn), err)]
pub fn insert_blacklisted_token(
    conn: &mut SqliteConnection,
    blacklist_entry: &BlacklistedTokenEntity,
) -> Result<BlacklistedTokenEntity> {
    conn.transaction(|conn| {
        let entry = diesel::insert_into(TokenBlacklistDsl::TokenBlacklist)
            .values(blacklist_entry)
            .returning(BlacklistedTokenEntity::as_select())
            .get_result(conn)?;
        Ok(entry)
    })
}

#[instrument(skip(conn), err)]
pub fn delete_expired_blacklisted_tokens(
    conn: &mut SqliteConnection,
    limit_time: NaiveDateTime,
) -> Result<usize> {
    conn.transaction(|conn| {
        let deleted_count = diesel::delete(TokenBlacklistDsl::TokenBlacklist)
            .filter(TokenBlacklistDsl::expires_at.lt(limit_time))
            .execute(conn)?;
        Ok(deleted_count)
    })
}
