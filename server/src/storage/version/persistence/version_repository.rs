use crate::storage::version::persistence::model::version_entity::VersionEntity;
use crate::user::persistence::model::encryption_key::IV;
use anyhow::Result;
use crabdrive_common::node::NodeId;
use crabdrive_common::version::VersionId;

pub(crate) trait VersionRepository {
    fn create_version(file_id: NodeId, upload_started_on: u64, iv: IV) -> VersionId;

    fn update_version(file_version: VersionEntity) -> Result<()>;

    fn get_version(version_id: VersionId) -> Result<VersionEntity>;

    fn delete_version(version_id: VersionId) -> Result<VersionEntity>;
}