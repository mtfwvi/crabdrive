// [ ] Einfügen für Node, User, Revision
// [ ] Löschen für Node, User, Revision
// [ ] Updaten für Node, User, Revision
// [ ] Selektieren für Node, User, Revision
// [ ] Kinder von Node selektieren

use crabdrive_common::uuid::UUID;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use std::error::Error;

use crate::{
    db::{
        NodeDsl, RevisionDsl,
        UserDsl::{self},
    },
    http::AppState,
    storage::{
        node::persistence::model::{
            encrypted_metadata::EncryptedMetadata, node_entity::NodeEntity,
        },
        revision::persistence::model::revision_entity::RevisionEntity,
    },
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

pub fn select_node(state: &AppState, node_id: UUID) -> Result<Option<NodeEntity>, Box<dyn Error>> {
    let mut conn = state.db_pool.get()?;
    let node = NodeDsl::Node
        .filter(NodeDsl::id.eq(node_id))
        .first::<NodeEntity>(&mut conn)
        .optional()?;
    Ok(node)
}

pub fn insert_node(
    state: &AppState,
    node: &NodeEntity,
    parent_mdata: &EncryptedMetadata,
) -> Result<(), Box<dyn Error>> {
    // Bei Insert:
    // - Node einfügen
    let mut conn = state.db_pool.get()?;
    let node = diesel::insert_into(NodeDsl::Node)
        .values(node)
        .returning(NodeEntity::as_select())
        .get_result(&mut conn)?;
    // - im Parent-Node: Metadaten aktualisieren, inkl. Mdata-change-counter
    diesel::update(NodeDsl::Node)
        .filter(NodeDsl::id.eq(node.parent_id.unwrap()))
        .set((
            NodeDsl::metadata.eq(parent_mdata),
            NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
        ))
        .execute(&mut conn)?;
    Ok(())
}

pub fn update_node(
    state: &AppState,
    node: &NodeEntity,
    parent_mdata: Option<&EncryptedMetadata>,
) -> Result<(), Box<dyn Error>> {
    let mut conn = state.db_pool.get()?;
    let node = diesel::update(NodeDsl::Node)
        .filter(NodeDsl::id.eq(node.id))
        .set(node)
        .returning(NodeEntity::as_select())
        .get_result(&mut conn)?;
    //No need to change parent_mdata if revision is changed
    if let Some(parent_mdata) = parent_mdata {
        diesel::update(NodeDsl::Node)
            .filter(NodeDsl::id.eq(node.parent_id.unwrap()))
            .set((
                NodeDsl::metadata.eq(parent_mdata),
                NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
            ))
            .execute(&mut conn)?;
    }
    Ok(())
}

pub fn delete_node(
    state: &AppState,
    node_id: UUID,
    parent_mdata: &EncryptedMetadata,
) -> Result<NodeEntity, Box<dyn Error>> {
    // Bei Delete:
    // - Node löschen
    // - im Parent-Node: Metadaten aktualisieren, inkl. Mdata-change-counter
    let mut conn = state.db_pool.get()?;
    let node: NodeEntity = diesel::delete(NodeDsl::Node)
        .filter(NodeDsl::id.eq(node_id))
        .returning(NodeEntity::as_select())
        .get_result(&mut conn)?;
    diesel::update(NodeDsl::Node)
        .filter(NodeDsl::id.eq(node.parent_id.unwrap()))
        .set((
            NodeDsl::metadata.eq(parent_mdata),
            NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
        ))
        .execute(&mut conn)?;
    Ok(node)
}

pub fn select_revision(
    state: &AppState,
    revision_id: UUID,
) -> Result<Option<RevisionEntity>, Box<dyn Error>> {
    let mut conn = state.db_pool.get()?;
    let revision = RevisionDsl::Revision
        .filter(RevisionDsl::id.eq(revision_id))
        .first::<RevisionEntity>(&mut conn)
        .optional()?;
    Ok(revision)
}

pub fn insert_revision(
    state: &AppState,
    revision: &RevisionEntity,
) -> Result<RevisionEntity, Box<dyn Error>> {
    // Bei Insert:
    // - Revision einfügen
    let mut conn = state.db_pool.get()?;
    let revision: RevisionEntity = diesel::insert_into(RevisionDsl::Revision)
        .values(revision)
        .returning(RevisionEntity::as_select())
        .get_result(&mut conn)?;
    Ok(revision)
}

pub fn update_revision(
    state: &AppState,
    revision: &RevisionEntity,
) -> Result<RevisionEntity, Box<dyn Error>> {
    let mut conn = state.db_pool.get()?;
    let revision = diesel::update(RevisionDsl::Revision)
        .filter(RevisionDsl::id.eq(revision.id))
        .set(revision)
        .returning(RevisionEntity::as_select())
        .get_result(&mut conn)?;
    //No need to change parent_mdata if revision is changed
    Ok(revision)
}

pub fn delete_revision(
    state: &AppState,
    revision_id: UUID,
) -> Result<RevisionEntity, Box<dyn Error>> {
    // Bei Delete:
    // - Node löschen
    // - im Parent-Node: Metadaten aktualisieren, inkl. Mdata-change-counter
    let mut conn = state.db_pool.get()?;
    let revision: RevisionEntity = diesel::delete(RevisionDsl::Revision)
        .filter(RevisionDsl::id.eq(revision_id))
        .returning(RevisionEntity::as_select())
        .get_result(&mut conn)?;
    Ok(revision)
}
