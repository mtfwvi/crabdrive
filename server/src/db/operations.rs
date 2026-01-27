use anyhow::Result;
use crabdrive_common::{
    storage::{NodeId, RevisionId},
    user::UserId,
};
use diesel::{
    Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
};

use crate::{
    db::{
        NodeDsl, RevisionDsl,
        UserDsl::{self},
        connection::DbPool,
    },
    http::AppState,
    storage::{
        node::persistence::model::node_entity::NodeEntity,
        revision::persistence::model::revision_entity::RevisionEntity,
    },
    user::persistence::model::user_entity::UserEntity,
};
use crabdrive_common::encrypted_metadata::EncryptedMetadata;

// User Ops
// TODO: Change from AppState -> DbPool

pub fn select_user(state: &AppState, user_id: UserId) -> Result<Option<UserEntity>> {
    let mut conn = state.db_pool.get()?;
    conn.transaction(|conn| {
        let user = UserDsl::User
            .filter(UserDsl::id.eq(user_id))
            .first::<UserEntity>(conn)
            .optional()?;
        Ok(user)
    })
}

pub fn insert_user(state: &AppState, user: &UserEntity) -> Result<()> {
    let mut conn = state.db_pool.get()?;
    conn.transaction(|conn| {
        diesel::insert_into(UserDsl::User)
            .values(user)
            .execute(conn)?;
        Ok(())
    })
}

pub fn update_user(state: &AppState, user: &UserEntity) -> Result<()> {
    let mut conn = state.db_pool.get()?;
    conn.transaction(|conn| {
        diesel::update(UserDsl::User)
            .filter(UserDsl::id.eq(user.id))
            .set(user)
            .execute(conn)?;
        Ok(())
    })
}

pub fn delete_user(state: &AppState, user_id: UserId) -> Result<UserEntity> {
    let mut conn = state.db_pool.get()?;
    conn.transaction(|conn| {
        let user: UserEntity = diesel::delete(UserDsl::User)
            .filter(UserDsl::id.eq(user_id))
            .returning(UserEntity::as_select())
            .get_result(conn)?;
        Ok(user)
    })
}

// Node Ops

pub fn get_all_children(db_pool: &DbPool, node_id: UserId) -> Result<Vec<NodeEntity>> {
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        let nodes = NodeDsl::Node
            .filter(NodeDsl::parent_id.eq(node_id))
            .load::<NodeEntity>(conn)?;
        Ok(nodes)
    })
}

pub fn select_node(db_pool: &DbPool, node_id: NodeId) -> Result<Option<NodeEntity>> {
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        let node = NodeDsl::Node
            .filter(NodeDsl::id.eq(node_id))
            .first::<NodeEntity>(conn)
            .optional()?;
        Ok(node)
    })
}

pub fn insert_node(
    db_pool: &DbPool,
    node: &NodeEntity,
    parent_mdata: &EncryptedMetadata,
) -> Result<()> {
    // Insert Node
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        let node = diesel::insert_into(NodeDsl::Node)
            .values(node)
            .returning(NodeEntity::as_select())
            .get_result(conn)?;
        // In Parent-Node: Update metadata and increase Metadata-Counter by 1
        diesel::update(NodeDsl::Node)
            .filter(NodeDsl::id.eq(node.parent_id.unwrap()))
            .set((
                NodeDsl::metadata.eq(parent_mdata),
                NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
            ))
            .execute(conn)?;
        Ok(())
    })
}

pub fn update_node(
    db_pool: &DbPool,
    node: &NodeEntity,
    parent_mdata: Option<&EncryptedMetadata>,
) -> Result<()> {
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        let node = diesel::update(NodeDsl::Node)
            .filter(NodeDsl::id.eq(node.id))
            .set(node)
            .returning(NodeEntity::as_select())
            .get_result(conn)?;
        // In certain cases, the parent metadata does not need to be updated (f.e. when changing a revision)
        if let Some(parent_mdata) = parent_mdata {
            diesel::update(NodeDsl::Node)
                .filter(NodeDsl::id.eq(node.parent_id.unwrap()))
                .set((
                    NodeDsl::metadata.eq(parent_mdata),
                    NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
                ))
                .execute(conn)?;
        }
        Ok(())
    })
}

// TODO: Cascade on Delete? If not, delete associated revisions and children nodes manually.
// TODO: Prevent deletion if parent_id is NULL (This node is either a root node or a trash node)
pub fn delete_node(
    db_pool: &DbPool,
    node_id: NodeId,
    parent_mdata: &EncryptedMetadata,
) -> Result<NodeEntity> {
    // Delete node
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        let node: NodeEntity = diesel::delete(NodeDsl::Node)
            .filter(NodeDsl::id.eq(node_id))
            .returning(NodeEntity::as_select())
            .get_result(conn)?;
        // In Parent-Node: Update metadata and increase Metadata-Counter by 1
        diesel::update(NodeDsl::Node)
            .filter(NodeDsl::id.eq(node.parent_id.unwrap()))
            .set((
                NodeDsl::metadata.eq(parent_mdata),
                NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
            ))
            .execute(conn)?;
        Ok(node)
    })
}

// Revision Ops

pub fn select_revision(
    db_pool: &DbPool,
    revision_id: RevisionId,
) -> Result<Option<RevisionEntity>> {
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        let revision = RevisionDsl::Revision
            .filter(RevisionDsl::id.eq(revision_id))
            .first::<RevisionEntity>(conn)
            .optional()?;
        Ok(revision)
    })
}

pub fn insert_revision(db_pool: &DbPool, revision: &RevisionEntity) -> Result<RevisionEntity> {
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        let revision: RevisionEntity = diesel::insert_into(RevisionDsl::Revision)
            .values(revision)
            .returning(RevisionEntity::as_select())
            .get_result(conn)?;
        Ok(revision)
    })
}

pub fn update_revision(db_pool: &DbPool, revision: &RevisionEntity) -> Result<RevisionEntity> {
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        let revision = diesel::update(RevisionDsl::Revision)
            .filter(RevisionDsl::id.eq(revision.id))
            .set(revision)
            .returning(RevisionEntity::as_select())
            .get_result(conn)?;
        Ok(revision)
    })
}

pub fn delete_revision(db_pool: &DbPool, revision_id: RevisionId) -> Result<RevisionEntity> {
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        let revision: RevisionEntity = diesel::delete(RevisionDsl::Revision)
            .filter(RevisionDsl::id.eq(revision_id))
            .returning(RevisionEntity::as_select())
            .get_result(conn)?;

        Ok(revision)
    })
}

pub fn get_all_revisions_by_node(db_pool: &DbPool, node_id: NodeId) -> Result<Vec<RevisionEntity>> {
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        let revisions = RevisionDsl::Revision
            .filter(RevisionDsl::file_id.eq(node_id))
            .load::<RevisionEntity>(conn)?;
        Ok(revisions)
    })
}
