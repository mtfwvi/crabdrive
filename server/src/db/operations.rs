// [ ] Einfügen für Node, User, Revision
// [ ] Löschen für Node, User, Revision
// [ ] Updaten für Node, User, Revision
// [ ] Selektieren für Node, User, Revision
// [ ] Kinder von Node selektieren
// [ ]

use std::error::Error;

use crabdrive_common::uuid::UUID;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    db::UserDsl::{self},
    http::AppState,
    user::persistence::model::user_entity::UserEntity,
};

// User Ops

pub fn select_user(state: &AppState, user_id: UUID) -> Result<Option<UserEntity>, Box<dyn Error>> {
    let mut conn = state.db_pool.get()?;
    let users = UserDsl::User
        .filter(UserDsl::id.eq(user_id))
        .load::<UserEntity>(&mut conn)?;
    Ok(users.first().cloned())
}

pub fn insert_user(state: &AppState, user: &UserEntity) -> Result<(), Box<dyn Error>> {
    let mut conn = state.db_pool.get()?;
    diesel::insert_into(UserDsl::User)
        .values(user)
        .execute(&mut conn)?;
    Ok(())
}

pub fn update_user(state: &AppState, user: &UserEntity) -> Result<(), Box<dyn Error>> {
    let mut conn = state.db_pool.get()?;
    diesel::update(UserDsl::User)
        .filter(UserDsl::id.eq(user.id))
        .set(user)
        .execute(&mut conn)?;
    Ok(())
}

pub fn delete_user(state: &AppState, user_id: UUID) -> Result<UserEntity, Box<dyn Error>> {
    let mut conn = state.db_pool.get()?;
    let user: UserEntity = diesel::delete(UserDsl::User)
        .filter(UserDsl::id.eq(user_id))
        .returning(UserEntity::as_select())
        .get_result(&mut conn)?;
    Ok(user)
}

// Node Ops

pub fn select_node(state: &AppState, user_id: UUID) -> Result<Option<UserEntity>, Box<dyn Error>> {
    let mut conn = state.db_pool.get()?;
    let users = UserDsl::User
        .filter(UserDsl::id.eq(user_id))
        .load::<UserEntity>(&mut conn)?;
    Ok(users.first().cloned())
}

pub fn insert_node(state: &AppState, user: &UserEntity) -> Result<(), Box<dyn Error>> {
    let mut conn = state.db_pool.get()?;
    diesel::insert_into(UserDsl::User)
        .values(user)
        .execute(&mut conn)?;
    Ok(())
}

pub fn update_node(state: &AppState, user: &UserEntity) -> Result<(), Box<dyn Error>> {
    let mut conn = state.db_pool.get()?;
    diesel::update(UserDsl::User)
        .filter(UserDsl::id.eq(user.id))
        .set(user)
        .execute(&mut conn)?;
    Ok(())
}

pub fn delete_node(state: &AppState, user_id: UUID) -> Result<UserEntity, Box<dyn Error>> {
    let mut conn = state.db_pool.get()?;
    let user: UserEntity = diesel::delete(UserDsl::User)
        .filter(UserDsl::id.eq(user_id))
        .returning(UserEntity::as_select())
        .get_result(&mut conn)?;
    Ok(user)
}
