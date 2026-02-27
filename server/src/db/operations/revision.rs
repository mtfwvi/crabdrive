use crate::db::RevisionDsl;
use crate::storage::revision::RevisionEntity;

use crabdrive_common::storage::{NodeId, RevisionId};

use anyhow::Result;
use diesel::{
    Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    SqliteConnection,
};
use tracing::instrument;

#[instrument(skip(conn), err)]
pub fn select_revision(
    conn: &mut SqliteConnection,
    revision_id: RevisionId,
) -> Result<Option<RevisionEntity>> {
    conn.transaction(|conn| {
        let revision = RevisionDsl::Revision
            .filter(RevisionDsl::id.eq(revision_id))
            .first::<RevisionEntity>(conn)
            .optional()?;
        Ok(revision)
    })
}

#[instrument(skip(conn), err)]
pub fn insert_revision(
    conn: &mut SqliteConnection,
    revision: &RevisionEntity,
) -> Result<RevisionEntity> {
    conn.transaction(|conn| {
        let revision: RevisionEntity = diesel::insert_into(RevisionDsl::Revision)
            .values(revision)
            .returning(RevisionEntity::as_select())
            .get_result(conn)?;
        Ok(revision)
    })
}

#[instrument(skip(conn), err)]
pub fn update_revision(
    conn: &mut SqliteConnection,
    revision: &RevisionEntity,
) -> Result<RevisionEntity> {
    conn.transaction(|conn| {
        let revision = diesel::update(RevisionDsl::Revision)
            .filter(RevisionDsl::id.eq(revision.id))
            .set(revision)
            .returning(RevisionEntity::as_select())
            .get_result(conn)?;
        Ok(revision)
    })
}

#[instrument(skip(conn), err)]
pub fn delete_revision(
    conn: &mut SqliteConnection,
    revision_id: RevisionId,
) -> Result<RevisionEntity> {
    conn.transaction(|conn| {
        let revision: RevisionEntity = diesel::delete(RevisionDsl::Revision)
            .filter(RevisionDsl::id.eq(revision_id))
            .returning(RevisionEntity::as_select())
            .get_result(conn)?;

        Ok(revision)
    })
}

#[instrument(skip(conn), err)]
pub fn get_all_revisions_by_node(
    conn: &mut SqliteConnection,
    node_id: NodeId,
) -> Result<Vec<RevisionEntity>> {
    conn.transaction(|conn| {
        let revisions = RevisionDsl::Revision
            .filter(RevisionDsl::file_id.eq(node_id))
            .load::<RevisionEntity>(conn)?;
        Ok(revisions)
    })
}
