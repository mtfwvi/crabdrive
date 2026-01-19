use crate::{db::connection::DbPool, http::AppConfig};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db_pool: Arc<DbPool>,
}

impl AppState {
    pub fn new(config: AppConfig, db_pool: DbPool) -> Self {
        Self {
            config: Arc::new(config),
            db_pool: Arc::new(db_pool),
        }
    }
}
