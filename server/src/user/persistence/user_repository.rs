use crate::user::persistence::model::user_entity::UserEntity;
use anyhow::Result;
use crabdrive_common::data::DataAmount;
use crabdrive_common::user::UserId;

pub(crate) trait UserRepository {
    fn create_user(
        &self,
        username: String,
        password_hash: String,
        storage_limit: DataAmount,
    ) -> Result<UserId>;

    fn get_user(&self, id: UserId) -> Result<UserEntity>;

    fn update_user(&self, updated_entity: UserEntity) -> Result<()>;

    fn delete_user(&self, id: UserId) -> Result<()>;
}
