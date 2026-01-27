use crate::{db::connection::DbPool, http::AppConfig};
use std::sync::Arc;

use crate::storage::node::NodeRepository;
use crate::storage::revision::RevisionRepository;
use crate::storage::vfs::FileRepository;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db_pool: Arc<DbPool>,
    pub vfs: Arc<dyn FileRepository + Send + Sync>,
    pub node_repository: Arc<dyn NodeRepository + Send + Sync>,
    pub revision_repository: Arc<dyn RevisionRepository + Send + Sync>,
}

impl AppState {
    pub fn new<FileRepo, NodeRepo, RevisionRepo>(
        config: AppConfig,
        db_pool: DbPool,
        vfs: FileRepo,
        node_repository: NodeRepo,
        revision_repository: RevisionRepo,
    ) -> Self
    where
        FileRepo: FileRepository + Send + Sync + 'static,
        NodeRepo: NodeRepository + Send + Sync + 'static,
        RevisionRepo: RevisionRepository + Send + Sync + 'static,
    {
        Self {
            config: Arc::new(config),
            db_pool: Arc::new(db_pool),
            vfs: Arc::new(vfs),
            node_repository: Arc::new(node_repository),
            revision_repository: Arc::new(revision_repository),
        }
    }
}
