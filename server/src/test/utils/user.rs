use crate::http::AppState;
use crate::storage::node::NodeEntity;
use crate::user::UserEntity;

use super::{NodeBuilder, TestNodeEntity};

use crabdrive_common::da;
use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::storage::{NodeId, NodeType};
use crabdrive_common::user::UserKeys;
use crabdrive_common::uuid::UUID;

use std::sync::Arc;

use axum_test::{TestRequest, TestServer};

pub struct TestUserEntity {
    pub server: Arc<TestServer>,
    pub state: AppState,
    pub id: UUID,
    pub entity: UserEntity,
    pub username: String,
    pub password: String,
    pub keys: UserKeys,
    pub token: String,
    pub refresh_token: String,
}

impl TestUserEntity {
    pub async fn new(server: Arc<TestServer>, state: AppState) -> Self {
        let username = format!("test-{}", UUID::random());
        let password = UUID::random().to_string();
        let keys = UserKeys::random();

        let mut user_entity = state
            .user_repository
            .create_user(&username, &password, da!(128 MiB), keys.clone())
            .expect("Failed to create user!");

        let root_node = state
            .node_repository
            .create_node(
                None,
                EncryptedMetadata::random(),
                user_entity.id,
                NodeType::Folder,
                NodeId::random(),
            )
            .expect("Failed to create root node");
        user_entity.root_node = Some(root_node.id);

        let trash_node = state
            .node_repository
            .create_node(
                None,
                EncryptedMetadata::random(),
                user_entity.id,
                NodeType::Folder,
                NodeId::random(),
            )
            .expect("Failed to create trash node");
        user_entity.trash_node = Some(trash_node.id);

        state
            .user_repository
            .update_user(user_entity.clone())
            .expect("Failed to insert nodes for user!");

        let (refresh_token, token) = state
            .user_repository
            .create_session(user_entity.id)
            .expect("Failed to create token");

        Self {
            server,
            state,
            id: user_entity.id,
            entity: user_entity,
            username,
            password,
            keys,
            refresh_token,
            token,
        }
    }

    pub fn get(&self, url: impl AsRef<str>) -> TestRequest {
        self.server
            .get(url.as_ref())
            .authorization_bearer(&self.token)
    }

    pub fn post(&self, url: impl AsRef<str>) -> TestRequest {
        self.server
            .post(url.as_ref())
            .authorization_bearer(&self.token)
    }

    pub fn patch(&self, url: impl AsRef<str>) -> TestRequest {
        self.server
            .patch(url.as_ref())
            .authorization_bearer(&self.token)
    }

    pub fn delete(&self, url: impl AsRef<str>) -> TestRequest {
        self.server
            .delete(url.as_ref())
            .authorization_bearer(&self.token)
    }

    pub fn get_root(&self) -> NodeId {
        self.entity.root_node.unwrap()
    }

    pub fn get_trash(&self) -> NodeId {
        self.entity.trash_node.unwrap()
    }

    /// Genrate a random folder in the root node
    pub async fn generate_random_folder(&self) -> TestNodeEntity {
        self.generate_folder_in(self.entity.root_node.expect("Root node missing"))
            .await
    }

    /// Genrate a random folder in a parent node
    pub async fn generate_folder_in(&self, parent_id: NodeId) -> TestNodeEntity {
        NodeBuilder::new(&self.state, self.id)
            .folder()
            .with_parent(parent_id)
            .build()
            .await
    }

    /// Genrate a random file in the root node
    pub async fn generate_random_file(&self) -> TestNodeEntity {
        self.generate_file_in(self.entity.root_node.expect("Root node missing"))
            .await
    }

    /// Genrate a file in a parent node in a parent node
    pub async fn generate_file_in(&self, parent_id: NodeId) -> TestNodeEntity {
        NodeBuilder::new(&self.state, self.id)
            .file()
            .with_parent(parent_id)
            .build()
            .await
    }

    pub async fn generate_file_with_chunks(&self, chunks: u32) -> TestNodeEntity {
        NodeBuilder::new(&self.state, self.id)
            .file()
            .with_chunks(chunks)
            .with_parent(self.get_root())
            .build()
            .await
    }

    /// Fetches the fresh node directly from the repository
    pub fn fetch_node_from_db(&self, node_id: NodeId) -> Option<NodeEntity> {
        self.state
            .node_repository
            .get_node(node_id)
            .expect("Database error during node fetch")
    }
}
