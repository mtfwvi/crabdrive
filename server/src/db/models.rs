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
    pub storage_limit: i32,
    pub masterkey: Vec<u8>,
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub root_key: Vec<u8>,
    pub root_node: i32,
    pub trash_key: Vec<u8>,
    pub trash_node: Vec<u8>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::db::schema::files)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct File {
    pub id: i32,
    pub name: String,
    pub file_path: String,
    pub file_size: i32,
    pub mime_type: String,
    pub folder_id: Option<i32>,
    pub owner_id: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::db::schema::folders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Folder {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
    pub owner_id: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::folders)]
pub struct NewFolder {
    pub name: String,
    pub parent_id: Option<i32>,
    pub owner_id: i32,
    pub created_at: String,
    pub updated_at: String,
}
