use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(encryptionKey))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: String,
    pub user_type: String,
    pub storage_limit: i32,
    pub masterkey: Vec<u8>,
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub root_key: Vec<u8>,
    pub root_node: i32,
    pub trash_key: Vec<u8>,
    pub trash_node: Vec<u8>,
}
