use crate::db::connection::create_pool;
use crate::storage::node::NodeRepository;
use crate::storage::node::persistence::node_repository::NodeRepositoryImpl;
use crate::storage::revision::RevisionRepository;
use crate::storage::revision::persistence::revision_repository::RevisionRepositoryImpl;
use crate::storage::share::persistence::share_repository::ShareRepository;
use crate::storage::share::persistence::share_repository::ShareRepositoryImpl;
use crate::storage::vfs::FileRepository;
use crate::storage::vfs::backend::Sfs;
use crate::user::auth::secrets::Keys;
use crate::user::persistence::user_repository::{UserRepository, UserRepositoryImpl};
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
    pub fn new(config: AppConfig) -> AppState {
        let pool = create_pool(&config.db.path, config.db.pool_size);

        let vfs = Sfs::new(&config.storage.dir);
        let keys = Keys::new(&config.auth.jwt_secret);

        let node_repository = NodeRepositoryImpl::new(Arc::new(pool.clone()));
        let revision_repository = RevisionRepositoryImpl::new(Arc::new(pool.clone()));
        let user_repository = UserRepositoryImpl::new(Arc::new(pool.clone()), keys.clone());
        let share_repository = ShareRepositoryImpl::new(Arc::new(pool.clone()));

        Self {
            config: Arc::new(config),
            db_pool: Arc::new(pool),
            vfs: Arc::new(RwLock::new(vfs)),
            node_repository: Arc::new(node_repository),
            revision_repository: Arc::new(revision_repository),
            user_repository: Arc::new(user_repository),
            share_repository: Arc::new(share_repository),
            keys: Arc::new(keys),
        }
    }
}
