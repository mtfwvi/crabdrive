use crate::storage::revision::persistence::model::revision_entity::RevisionEntity;
use anyhow::Result;
use chrono::NaiveDateTime;
use crabdrive_common::iv::IV;
use crabdrive_common::storage::NodeId;
use crabdrive_common::storage::RevisionId;

pub(crate) trait RevisionRepository {
    fn create_revision(
        &self,
        file_id: NodeId,
        upload_started_on: NaiveDateTime,
        iv: IV,
    ) -> Result<RevisionEntity>;

    fn update_revision(&self, file_version: RevisionEntity) -> Result<()>;

    fn get_revision(&self, id: RevisionId) -> Result<RevisionEntity>;

    fn delete_revision(&self, id: RevisionId) -> Result<RevisionEntity>;
}
