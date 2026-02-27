use crate::storage::node::NodeRepository;
use crate::storage::revision::RevisionRepository;
use crate::storage::share::persistence::share_repository::ShareRepository;
use crate::storage::vfs::FileRepository;
use crate::user::auth::secrets::Keys;
use crate::user::persistence::user_repository::UserRepository;
use crate::{db::connection::DbPool, http::AppConfig};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db_pool: Arc<DbPool>,
    pub vfs: Arc<RwLock<dyn FileRepository + Send + Sync>>,
    pub node_repository: Arc<dyn NodeRepository + Send + Sync>,
    pub revision_repository: Arc<dyn RevisionRepository + Send + Sync>,
    pub user_repository: Arc<dyn UserRepository + Send + Sync>,
    pub share_repository: Arc<dyn ShareRepository + Send + Sync>,
    pub keys: Arc<Keys>,
}

impl AppState {
    pub fn new(
        config: AppConfig,
        db_pool: DbPool,
        vfs: Arc<RwLock<dyn FileRepository + Send + Sync>>,
        node_repository: Arc<dyn NodeRepository + Send + Sync>,
        revision_repository: Arc<dyn RevisionRepository + Send + Sync>,
        user_repository: Arc<dyn UserRepository + Send + Sync>,
        share_repository: Arc<dyn ShareRepository + Send + Sync>,
        keys: Keys,
    ) -> Self {
        Self {
            config: Arc::new(config),
            db_pool: Arc::new(db_pool),
            vfs,
            node_repository,
            revision_repository,
            user_repository,
            share_repository,
            keys: Arc::new(keys),
        }
    }
}
