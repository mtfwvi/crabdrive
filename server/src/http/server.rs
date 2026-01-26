use crate::db::connection::create_pool;
use crate::http::middleware::logging_middleware;
use crate::http::{AppConfig, AppState, routes};
use crate::storage::node::persistence::node_repository::NodeState;
use crate::storage::{
    node::persistence::model::{encrypted_metadata::EncryptedMetadata, node_entity::NodeEntity},
    vfs::backend::Sfs,
};

use crabdrive_common::uuid::UUID;

use std::io::ErrorKind;
use std::sync::Arc;

use axum::{Router, middleware};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use tracing::{error, info};

async fn graceful_shutdown(state: AppState) {
    let _ = tokio::signal::ctrl_c().await;
    shutdown(state).await;
}

#[allow(unused_variables)] // TODO: Remove when state is actually used
async fn shutdown(state: AppState) {
    info!("Stopping server");
}

pub async fn start(config: AppConfig) -> Result<(), ()> {
    let pool = create_pool(&config.db.path, config.db.pool_size);

    let mut storage_dir = std::path::PathBuf::new();
    storage_dir.push(&config.storage.dir);
    let vfs = Sfs::new(storage_dir);
    let node_repository = NodeState::new(Arc::new(pool.clone()));

    let state = AppState::new(config.clone(), pool, vfs, node_repository);

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./res/migrations/");
    let mut conn = state.db_pool.get().unwrap();
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    // HACK: Create a root node with a zeroed UUID, if it's not already existing.
    // TODO: Remove when adding auth!
    if crate::db::operations::select_node(&state.db_pool, UUID::nil())
        .unwrap()
        .is_none()
    {
        let node = NodeEntity {
            id: UUID::nil(),
            owner_id: UUID::nil(),
            parent_id: None,
            metadata: EncryptedMetadata::nil(),
            deleted_on: None,
            current_revision: None,
            metadata_change_counter: 0,
            node_type: crabdrive_common::storage::NodeType::Folder,
        };
        crate::db::operations::insert_node(&state.db_pool, &node, &EncryptedMetadata::nil())
            .unwrap();
    }

    let app = Router::<AppState>::new()
        .with_state(state.clone())
        .merge(routes::routes())
        .layer(middleware::from_fn(logging_middleware));

    let addr = config.addr();

    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => Ok(listener),
        Err(err) => {
            error!(
                "Failed to bind to {}. {}",
                addr,
                match err.kind() {
                    ErrorKind::AddrInUse => "The port is already in use!".to_string(),
                    ErrorKind::PermissionDenied =>
                        "You do not have sufficient permissions!".to_string(),
                    ErrorKind::AddrNotAvailable => "The requested IP is not available!".to_string(),
                    _ => format!("{}", err),
                }
            );
            Err(())
        }
    }?;

    info!("Server running on http://{}", &addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_shutdown(state.clone()))
        .await
        .unwrap();

    Ok(())
}
