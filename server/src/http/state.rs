use crate::auth::secrets::Keys;
use crate::storage::node::NodeRepository;
use crate::storage::revision::RevisionRepository;
use crate::storage::vfs::FileRepository;
use crate::user::persistence::user_repository::UserRepository;
use crate::{db::connection::DbPool, http::AppConfig};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db_pool: Arc<DbPool>,
    pub vfs: Arc<RwLock<dyn FileRepository + Send + Sync>>,
    pub node_repository: Arc<dyn NodeRepository + Send + Sync>,
    pub revision_repository: Arc<dyn RevisionRepository + Send + Sync>,
    pub user_repository: Arc<dyn UserRepository + Send + Sync>,
    pub keys: Arc<Keys>,
}

impl AppState {
    pub fn new<FileRepo, NodeRepo, RevisionRepo, UserRepo>(
        config: AppConfig,
        db_pool: DbPool,
        vfs: FileRepo,
        node_repository: NodeRepo,
        revision_repository: RevisionRepo,
        user_repository: UserRepo,
        keys: Keys,
    ) -> Self
    where
        FileRepo: FileRepository + Send + Sync + 'static,
        NodeRepo: NodeRepository + Send + Sync + 'static,
        RevisionRepo: RevisionRepository + Send + Sync + 'static,
        UserRepo: UserRepository + Send + Sync + 'static,
    {
        Self {
            config: Arc::new(config),
            db_pool: Arc::new(db_pool),
            vfs: Arc::new(RwLock::new(vfs)),
            node_repository: Arc::new(node_repository),
            revision_repository: Arc::new(revision_repository),
            user_repository: Arc::new(user_repository),
            keys: Arc::new(keys),
        }
    }
}
