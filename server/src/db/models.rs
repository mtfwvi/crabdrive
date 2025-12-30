use diesel::{prelude::*, sql_types::Bool};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::db::schema::User)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(encryptionKey))]
pub struct User {
    pub user_type: String,
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub created_at: String,
    pub updated_at: String,
    pub encryption_uninitialized: i32,
    pub storage_limit: i32,
    pub masterkey: Vec<u8>,
    pub private_key: Vec<u8>,
    pub privatekey_iv: Vec<u8>,
    pub public_key: Vec<u8>,
    pub root_key: Vec<u8>,
    pub rootkey_iv: Vec<u8>,
    pub root_node: i32,
    pub trash_key: Vec<u8>,
    pub trashkey_iv: Vec<u8>,
    pub trash_node: Vec<u8>,
}

pub struct Node {
    id: i32,
    parent_id: i32,
    owner_id: String,
    mdata: Vec<u8>,
    iv: Vec<u8>,
    deleted_on: String,
    created_at: String,
    metadata_change_counter: i32,
    current_revision: i32,
    node_type: String,
}

pub struct Revision {
    id: String,
    fid: String,
    upload_started_on: String,
    upload_ended_on: String,
    iv: Vec<u8>,
}
