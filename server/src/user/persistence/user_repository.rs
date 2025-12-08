use crate::user::persistence::model::user_entity::UserEntity;
use crabdrive_common::user::UserId;

pub(crate) trait UserRepository {
    fn create_user(username: String, password_hash: String) -> UserId; // TODO: Add remaining initialization fields as parameter

    fn get_user(user_id: UserId) -> UserEntity;

    // TODO: More operations?

    fn delete_user(user_id: UserId);
}
