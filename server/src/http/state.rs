use crate::{db::connection::DbPool, http::AppConfig};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db_pool: Arc<DbPool>,
    pub vfs: Arc<dyn crate::storage::vfs::FileRepository + Send + Sync>,
    pub node_repository:
        Arc<dyn crate::storage::node::persistence::node_repository::NodeRepository + Send + Sync>,
}

impl AppState {
    pub fn new<VFS, NodeRepository>(
        config: AppConfig,
        db_pool: DbPool,
        vfs: VFS,
        node_repository: NodeRepository,
    ) -> Self
    where
        VFS: crate::storage::vfs::FileRepository + Send + Sync + 'static,
        NodeRepository: crate::storage::node::persistence::node_repository::NodeRepository
            + Send
            + Sync
            + 'static,
    {
        Self {
            config: Arc::new(config),
            db_pool: Arc::new(db_pool),
            vfs: Arc::new(vfs),
            node_repository: Arc::new(node_repository),
        }
    }
}
