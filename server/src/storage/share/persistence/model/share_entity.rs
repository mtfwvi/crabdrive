use crate::storage::node::NodeEntity;
use crate::user::UserEntity;

use crabdrive_common::encryption_key::EncryptionKey;
use crabdrive_common::storage::{NodeId, ShareId};
use crabdrive_common::user::UserId;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    Debug,
    Insertable,
    AsChangeset,
    Clone,
    Associations,
)]
#[diesel(table_name = crate::db::schema::Share)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(UserEntity, foreign_key = shared_by))]
#[diesel(belongs_to(NodeEntity, foreign_key = node_id))]
pub struct ShareEntity {
    pub id: ShareId,
    pub node_id: NodeId,
    pub shared_by: UserId,
    pub accepted_by: Option<UserId>,
    pub time_shared: NaiveDateTime,
    pub time_accepted: Option<NaiveDateTime>,
    pub shared_encryption_key: Option<EncryptionKey>,
    pub accepted_encryption_key: Option<EncryptionKey>,
}
