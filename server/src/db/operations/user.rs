use crate::db::UserDsl;
use crate::user::UserEntity;

use crabdrive_common::user::UserId;

use anyhow::Result;
use diesel::{
    Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    SqliteConnection,
};
use tracing::instrument;

#[instrument(skip(conn), err)]
pub fn select_user(conn: &mut SqliteConnection, user_id: UserId) -> Result<Option<UserEntity>> {
    conn.transaction(|conn| {
        let user = UserDsl::User
            .filter(UserDsl::id.eq(user_id))
            .first::<UserEntity>(conn)
            .optional()?;
        Ok(user)
    })
}

#[instrument(skip(conn), err)]
pub fn select_user_by_username(
    conn: &mut SqliteConnection,
    username: &str,
) -> Result<Option<UserEntity>> {
    conn.transaction(|conn| {
        let user = UserDsl::User
            .filter(UserDsl::username.eq(username))
            .first::<UserEntity>(conn)
            .optional()?;
        Ok(user)
    })
}

#[instrument(skip(conn), err)]
pub fn insert_user(conn: &mut SqliteConnection, user: &UserEntity) -> Result<()> {
    conn.transaction(|conn| {
        diesel::insert_into(UserDsl::User)
            .values(user)
            .execute(conn)?;
        Ok(())
    })
}

#[instrument(skip(conn), err)]
pub fn update_user(conn: &mut SqliteConnection, user: &UserEntity) -> Result<UserEntity> {
    conn.transaction(|conn| {
        let updated = diesel::update(UserDsl::User)
            .filter(UserDsl::id.eq(user.id))
            .set(user)
            .returning(UserEntity::as_select())
            .get_result(conn)?;
        Ok(updated)
    })
}

#[instrument(skip(conn), err)]
pub fn delete_user(conn: &mut SqliteConnection, user_id: UserId) -> Result<UserEntity> {
    conn.transaction(|conn| {
        let user: UserEntity = diesel::delete(UserDsl::User)
            .filter(UserDsl::id.eq(user_id))
            .returning(UserEntity::as_select())
            .get_result(conn)?;
        Ok(user)
    })
}
