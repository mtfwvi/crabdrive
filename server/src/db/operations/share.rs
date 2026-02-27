use crate::db::ShareDsl;
use crate::db::operations::node::{get_path_between_nodes, select_node};
use crate::db::operations::user::select_user;
use crate::storage::share::ShareEntity;

use crabdrive_common::storage::{NodeId, ShareId};
use crabdrive_common::user::UserId;

use anyhow::Result;
use diesel::{
    Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    SqliteConnection,
};
use tracing::instrument;

#[instrument(skip(conn), err)]
pub fn select_share(conn: &mut SqliteConnection, share_id: ShareId) -> Result<Option<ShareEntity>> {
    conn.transaction(|conn| {
        let share = ShareDsl::Share
            .filter(ShareDsl::id.eq(share_id))
            .first::<ShareEntity>(conn)
            .optional()?;
        Ok(share)
    })
}

#[instrument(skip(conn), err)]
pub fn insert_share(conn: &mut SqliteConnection, share: &ShareEntity) -> Result<ShareEntity> {
    conn.transaction(|conn| {
        let share: ShareEntity = diesel::insert_into(ShareDsl::Share)
            .values(share)
            .returning(ShareEntity::as_select())
            .get_result(conn)?;
        Ok(share)
    })
}

#[instrument(skip(conn), err)]
pub fn update_share(conn: &mut SqliteConnection, share: &ShareEntity) -> Result<ShareEntity> {
    conn.transaction(|conn| {
        let share = diesel::update(ShareDsl::Share)
            .filter(ShareDsl::id.eq(share.id))
            .set(share)
            .returning(ShareEntity::as_select())
            .get_result(conn)?;
        Ok(share)
    })
}

#[instrument(skip(conn), err)]
pub fn delete_share(conn: &mut SqliteConnection, share_id: ShareId) -> Result<ShareEntity> {
    conn.transaction(|conn| {
        let share: ShareEntity = diesel::delete(ShareDsl::Share)
            .filter(ShareDsl::id.eq(share_id))
            .returning(ShareEntity::as_select())
            .get_result(conn)?;
        Ok(share)
    })
}

#[instrument(skip(conn), err)]
pub fn get_all_shares_by_node(
    conn: &mut SqliteConnection,
    node_id: NodeId,
) -> Result<Vec<ShareEntity>> {
    conn.transaction(|conn| {
        let shares = ShareDsl::Share
            .filter(ShareDsl::node_id.eq(node_id))
            .load::<ShareEntity>(conn)?;
        Ok(shares)
    })
}

#[instrument(skip(conn), err)]
pub fn get_all_shares_by_user(
    conn: &mut SqliteConnection,
    user_id: UserId,
) -> Result<Vec<ShareEntity>> {
    conn.transaction(|conn| {
        let shares = ShareDsl::Share
            .filter(ShareDsl::accepted_by.eq(user_id))
            .load::<ShareEntity>(conn)?;
        Ok(shares)
    })
}

#[instrument(skip(conn), err)]
pub fn get_share_by_node_id_and_accepted_user_id(
    conn: &mut SqliteConnection,
    node_id: NodeId,
    user_id: UserId,
) -> Result<Option<ShareEntity>> {
    conn.transaction(|conn| {
        let share = ShareDsl::Share
            .filter(ShareDsl::node_id.eq(node_id))
            .filter(ShareDsl::accepted_by.eq(user_id))
            .first::<ShareEntity>(conn)
            .optional()?;
        Ok(share)
    })
}

#[instrument(skip(conn), err)]
pub fn get_access_list(
    conn: &mut SqliteConnection,
    node_id: NodeId,
) -> Result<Vec<(UserId, String)>> {
    let Some(node) = select_node(conn, node_id)? else {
        return Ok(vec![]);
    };

    let owner = select_user(conn, node.owner_id)?
        .ok_or(anyhow::anyhow!("db constraints are not respected"))?;

    let mut access_list = vec![(owner.id, owner.username)];

    let share_entities = get_all_shares_by_node(conn, node_id)?;

    for share_entity in share_entities {
        let Some(user_id) = share_entity.accepted_by else {
            continue;
        };

        let user = select_user(conn, user_id)?
            .ok_or(anyhow::anyhow!("db constraints are not respected"))?;
        access_list.push((user.id, user.username));
    }

    Ok(access_list)
}

#[instrument(skip(conn), err)]
pub fn has_access(conn: &mut SqliteConnection, node_id: NodeId, user_id: UserId) -> Result<bool> {
    let node = if let Some(node) = select_node(conn, node_id)? {
        node
    } else {
        return Ok(false);
    };

    if node.owner_id == user_id {
        return Ok(true);
    }

    // this will return the path to between a root node and the current node as the nil node does not exist (probably)
    let path_to_root = get_path_between_nodes(conn, NodeId::nil(), node_id)?;

    let shared_with_user = get_all_shares_by_user(conn, user_id)?;

    for node in path_to_root.iter().rev() {
        // if the subtree containing the node was moved to the trash it should not be accessible anymore
        if node.deleted_on.is_some() {
            return Ok(false);
        }

        if shared_with_user
            .iter()
            .any(|share_entity| share_entity.node_id == node.id)
        {
            return Ok(true);
        }
    }
    Ok(false)
}
