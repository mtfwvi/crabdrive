use diesel::connection::SimpleConnection;
use diesel::r2d2::{ConnectionManager, CustomizeConnection, Pool};
use diesel::{prelude::*, r2d2};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

// https://docs.rs/diesel/latest/diesel/r2d2/trait.CustomizeConnection.html
#[derive(Debug)]
struct ConnectionCustomizer;

impl CustomizeConnection<SqliteConnection, r2d2::Error> for ConnectionCustomizer {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), r2d2::Error> {
        conn.batch_execute(
            "
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA foreign_keys = ON;
        ",
        )
        .map_err(r2d2::Error::QueryError)?;

        Ok(())
    }
}

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn create_pool(database_url: &str, pool_size: usize) -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let db_pool = Pool::builder()
        .connection_customizer(Box::new(ConnectionCustomizer))
        .max_size(pool_size.try_into().unwrap())
        .build(manager)
        .expect("Failed to create a pool");

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./res/migrations/");
    let mut conn = db_pool.get().unwrap();
    tracing::info!("Running pending migrations");
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    db_pool
}
