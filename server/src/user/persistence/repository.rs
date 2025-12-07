use crate::user::persistence::model::UserId;

pub(crate) trait UserRepository {
    fn create_user(username: String, email: String, password_hash: String);
    fn delete_user(user_id: UserId);
}
