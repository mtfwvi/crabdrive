use crate::user::persistence::model::user_entity::UserEntity;
use anyhow::Result;
use crabdrive_common::{data::DataAmount, uuid::UUID};

pub(crate) trait UserRepository {
    fn create_user(
        &self,
        username: String,
        password_hash: String,
        storage_limit: DataAmount,
    ) -> Result<UserEntity>;

    fn get_user(&self, id: UUID) -> Result<UserEntity>;

    fn update_user(&self, updated_entity: UserEntity) -> Result<()>;

    fn delete_user(&self, id: UUID) -> Result<()>;
}
