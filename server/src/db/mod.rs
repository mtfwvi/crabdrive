notepad src\db\mod.rs
pub mod models;
pub mod schema;
pub mod connection;
pub mod operations;

pub use connection::{create_pool, establish_connection, DbPool};
