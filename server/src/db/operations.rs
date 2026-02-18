use crate::{
    db::{
        NodeDsl, RevisionDsl,
        UserDsl::{self},
        connection::DbPool,
    },
    storage::{
        node::persistence::model::node_entity::NodeEntity,
        revision::persistence::model::revision_entity::RevisionEntity,
    },
    user::persistence::model::user_entity::UserEntity,
};
use anyhow::{Context, Result};
use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::routes::node::path_between_nodes;
use crabdrive_common::routes::node::shared::share;
use crabdrive_common::storage::ShareId;
use crate::db::ShareDsl;
use crate::request_handler::node::get_node;
use crate::storage::share::persistence::model::share_entity::ShareEntity;
use crabdrive_common::{
    storage::{NodeId, RevisionId},
    user::UserId,
    uuid::UUID,
};
use diesel::sql_types::Text;
use diesel::{
    Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    sql_query,
};
// User Ops

pub fn select_user(db_pool: &DbPool, user_id: UserId) -> Result<Option<UserEntity>> {
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        let user = UserDsl::User
            .filter(UserDsl::id.eq(user_id))
            .first::<UserEntity>(conn)
            .optional()?;
        Ok(user)
    })
}

pub fn select_user_by_username(db_pool: &DbPool, username: &str) -> Result<Option<UserEntity>> {
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        let user = UserDsl::User
            .filter(UserDsl::username.eq(username))
            .first::<UserEntity>(conn)
            .optional()?;
        Ok(user)
    })
}

pub fn insert_user(db_pool: &DbPool, user: &UserEntity) -> Result<()> {
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        diesel::insert_into(UserDsl::User)
            .values(user)
            .execute(conn)?;
        Ok(())
    })
}

pub fn update_user(db_pool: &DbPool, user: &UserEntity) -> Result<UserEntity> {
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        let updated = diesel::update(UserDsl::User)
            .filter(UserDsl::id.eq(user.id))
            .set(user)
            .returning(UserEntity::as_select())
            .get_result(conn)?;
        Ok(updated)
    })
}

pub fn delete_user(db_pool: &DbPool, user_id: UserId) -> Result<UserEntity> {
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        let user: UserEntity = diesel::delete(UserDsl::User)
            .filter(UserDsl::id.eq(user_id))
            .returning(UserEntity::as_select())
            .get_result(conn)?;
        Ok(user)
    })
}

// Node Ops

/// return a list of nodes from the to_node to the from_node or a root node if not path exists.
/// **Check that the first node in the list is really the node you want and not a root node**
// Query that finds the path between the nodes in the tree
// It should stop as soon as it cannot discover a new node (reached a root node) or if the desired node was reached
pub fn get_path_between_nodes(
    db_pool: &DbPool,
    from: NodeId,
    to: NodeId,
) -> Result<Vec<NodeEntity>> {
    let query = sql_query("\
        WITH RECURSIVE path_between_nodes AS ( \
            SELECT *,1 as _count FROM Node s1 \
            WHERE s1.id = $1 \
        UNION ALL \
            SELECT s2.*,_count+1 as _count FROM Node s2 \
            JOIN path_between_nodes s1 ON s1.parent_id = s2.id WHERE NOT s1.id = $2
        ) \
        SELECT id, parent_id, owner_id, metadata, deleted_on, metadata_change_counter, current_revision, node_type \
        FROM path_between_nodes \
        ORDER BY _count DESC \
    ").bind::<Text, _>(to.to_string()).bind::<Text, _>(from.to_string());
    Ok(query.load(&mut db_pool.get()?)?)
}

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
        // Skip this, if ID is nil, since it is the global root node
        // TODO: Remove when adding auth
        if node.id != UUID::nil() && node.parent_id.is_some() {
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

pub fn update_node(
    db_pool: &DbPool,
    node: &NodeEntity,
    parent_mdata: Option<&EncryptedMetadata>,
) -> Result<NodeEntity> {
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
        Ok(node)
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

pub fn move_node(
    db_pool: &DbPool,
    id: NodeId,
    from: NodeId,
    from_metadata: EncryptedMetadata,
    to: NodeId,
    to_metadata: EncryptedMetadata,
) -> Result<()> {
    let mut conn = db_pool.get().context("Failed to get database connection")?;
    conn.transaction(|conn| {
        diesel::update(NodeDsl::Node)
            .filter(NodeDsl::id.eq(from))
            .set((
                NodeDsl::metadata.eq(&from_metadata),
                NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
            ))
            .execute(conn)
            .context("Failed to update from parent")?;
        diesel::update(NodeDsl::Node)
            .filter(NodeDsl::id.eq(to))
            .set((
                NodeDsl::metadata.eq(&to_metadata),
                NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
            ))
            .execute(conn)
            .context("Failed to update to parent")?;
        diesel::update(NodeDsl::Node)
            .filter(NodeDsl::id.eq(id))
            .set((
                NodeDsl::parent_id.eq(Some(to)),
                NodeDsl::metadata_change_counter.eq(NodeDsl::metadata_change_counter + 1),
            ))
            .execute(conn)
            .context("Failed to move node")?;
        Ok::<(), anyhow::Error>(())
    })?;
    Ok(())
}

pub fn has_access(
    db_pool: &DbPool,
    node_id: NodeId,
    user_id: UserId,
) -> Result<bool> {
    let node = if let Some(node) = select_node(db_pool, node_id)? {
        node
    } else {
        return Ok(false);
    };

    if node.owner_id == user_id {
        return Ok(true);
    }

    // this will return the path to between a root node and the current node as the nil node does not exist (probably)
    let path_to_root = get_path_between_nodes(db_pool, NodeId::nil(), node_id)?;

    let shared_with_user = get_all_shares_by_user(db_pool, user_id)?;

    for node in path_to_root.iter().rev() {
        // if the subtree containing the node was moved to the trash it should not be accessible anymore
        if node.deleted_on.is_some() {
            return Ok(false);
        }

        let matching_nodes = shared_with_user.iter().filter(|share_entity| share_entity.node_id == node.id).collect::<Vec<&ShareEntity>>();

        if !matching_nodes.is_empty() {
            return Ok(true);
        }
    }
    Ok(false)
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

//Share ops

pub fn select_share(
    db_pool: &DbPool,
    share_id: ShareId,
) -> Result<Option<ShareEntity>> {
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        let share = ShareDsl::Share
            .filter(ShareDsl::id.eq(share_id))
            .first::<ShareEntity>(conn)
            .optional()?;
        Ok(share)
    })
}

pub fn insert_share(db_pool: &DbPool, share: &ShareEntity) -> Result<ShareEntity> {
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        let share: ShareEntity = diesel::insert_into(ShareDsl::Share)
            .values(share)
            .returning(ShareEntity::as_select())
            .get_result(conn)?;
        Ok(share)
    })
}

pub fn update_share(db_pool: &DbPool, share: &ShareEntity) -> Result<ShareEntity> {
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        let share = diesel::update(ShareDsl::Share)
            .filter(ShareDsl::id.eq(share.id))
            .set(share)
            .returning(ShareEntity::as_select())
            .get_result(conn)?;
        Ok(share)
    })
}

pub fn delete_share(db_pool: &DbPool, share_id: ShareId) -> Result<ShareEntity> {
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        let share: ShareEntity = diesel::delete(ShareDsl::Share)
            .filter(ShareDsl::id.eq(share_id))
            .returning(ShareEntity::as_select())
            .get_result(conn)?;
        Ok(share)
    })
}

pub fn get_all_shares_by_node(db_pool: &DbPool, node_id: NodeId) -> Result<Vec<ShareEntity>> {
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        let shares = ShareDsl::Share
            .filter(ShareDsl::node_id.eq(node_id))
            .load::<ShareEntity>(conn)?;
        Ok(shares)
    })
}

pub fn get_all_shares_by_user(db_pool: &DbPool, user_id: UserId) -> Result<Vec<ShareEntity>> {
    let mut conn = db_pool.get()?;
    conn.transaction(|conn| {
        let shares = ShareDsl::Share
            .filter(ShareDsl::accepted_by.eq(user_id))
            .load::<ShareEntity>(conn)?;
        Ok(shares)
    })
}
