use chrono::NaiveDateTime;
use crabdrive_common::iv::IV;
use crabdrive_common::storage::NodeId;
use crabdrive_common::storage::RevisionId;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::db::schema::Revision)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RevisionEntity {
    pub id: RevisionId,

    /// The `NodeId` this revision belongs to. It should reference a Node with the type File
    pub file_id: NodeId,

    /// The time the revision was created
    pub upload_started_on: NaiveDateTime,

    /// The time the revision was complete on the server (all chunks were present)
    pub upload_ended_on: Option<NaiveDateTime>,

    /// The random bytes used as IV prefix to last encrypt the file. The iv for each chunk must be
    /// derived from this value + 4 Bytes describing the index to avoid reordering. This value
    /// MUST NOT be reused for encrypting a new file
    pub iv: IV,
}
