use crate::http::{AppConfig, AppState};
use crate::storage::node::NodeRepository;
use crate::storage::revision::RevisionRepository;
use crate::user::UserRepository;

use super::TestUserEntity;

use std::ops::Range;
use std::sync::Arc;

use axum_test::TestServer;
use bytes::Bytes;
use rand::{Rng, distr::Alphanumeric};
use sha2::{Digest, Sha256};

pub struct TestContext {
    pub server: Arc<TestServer>,
    pub state: AppState,
    pub users: Vec<TestUserEntity>,
    // repos
    pub user: Arc<dyn UserRepository + Send + Sync>,
    pub node: Arc<dyn NodeRepository + Send + Sync>,
    pub revision: Arc<dyn RevisionRepository + Send + Sync>,
}

impl TestContext {
    pub async fn new(amount_users: u32) -> Self {
        let config = AppConfig::test();
        let (router, state) = crate::http::server::create_app(config);

        let server = TestServer::new(router).expect("Failed to create test server!");
        let arc = Arc::new(server);

        let mut users = Vec::with_capacity(amount_users as usize);

        for _ in 0..amount_users {
            let user = TestUserEntity::new(arc.clone(), state.clone()).await;
            users.push(user);
        }

        Self {
            server: arc,
            users,
            node: state.node_repository.clone(),
            user: state.user_repository.clone(),
            revision: state.revision_repository.clone(),
            state,
        }
    }

    pub fn get_user(&self, index: usize) -> &TestUserEntity {
        &self.users[index]
    }

    pub fn validate_jwt(&self, token: &str) -> bool {
        self.state
            .user_repository
            .verify_jwt(&token)
            .unwrap()
            .is_some()
    }

    pub fn validate_checksum(expected: &str, bytes: &Bytes) {
        assert_eq!(format!("{:x}", Sha256::digest(bytes)), expected)
    }

    pub fn random_range(len_range: Range<usize>) -> usize {
        let mut rng = rand::rng();
        rng.random_range(len_range)
    }

    pub fn random_text() -> String {
        TestContext::random_text_with_len(10..20)
    }

    pub fn random_text_with_len(len_range: Range<usize>) -> String {
        let mut rng = rand::rng();
        let len = rng.random_range(len_range);
        rng.sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }

    pub fn random_bytes(size: usize) -> bytes::Bytes {
        let mut rng = rand::rng();
        let mut data = vec![0u8; size];
        rng.fill(&mut data[..]);
        data.into()
    }
}
