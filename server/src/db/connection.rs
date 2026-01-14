use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn create_pool(database_url: &str, pool_size: usize) -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .max_size(pool_size.try_into().unwrap())
        .build(manager)
        .expect("Failed to create a pool")
}
