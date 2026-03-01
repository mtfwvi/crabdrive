use crate::http::AppState;
use crate::storage::vfs::FileChunk;
use super::{TestChunk, TestNodeEntity, TestRevisionEntity};

use crabdrive_common::da;
use crabdrive_common::encrypted_metadata::EncryptedMetadata;
use crabdrive_common::iv::IV;
use crabdrive_common::storage::{ChunkIndex, NodeId, NodeType};
use crabdrive_common::uuid::UUID;

use bytes::BytesMut;
use rand::{Rng, RngCore};
use sha2::{Digest, Sha256};

pub struct NodeBuilder<'a> {
    state: &'a AppState,
    owner_id: UUID,
    parent_id: Option<NodeId>,
    node_type: NodeType,
    chunk_count: Option<u32>,
    chunk_size: usize,
}

impl<'a> NodeBuilder<'a> {
    pub fn new(state: &'a AppState, owner_id: UUID) -> Self {
        Self {
            state,
            owner_id,
            parent_id: None,
            node_type: NodeType::File,
            chunk_count: None,
            chunk_size: da!(4 KiB).as_bytes() as usize,
        }
    }

    pub fn folder(mut self) -> Self {
        self.node_type = NodeType::Folder;
        self
    }

    pub fn file(mut self) -> Self {
        self.node_type = NodeType::File;
        self
    }

    pub fn with_parent(mut self, parent_id: NodeId) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    pub fn with_chunks(mut self, count: u32) -> Self {
        self.chunk_count = Some(count);
        self
    }

    pub async fn build(self) -> TestNodeEntity {
        let mut node = self
            .state
            .node_repository
            .create_node(
                self.parent_id,
                EncryptedMetadata::random(),
                self.owner_id,
                self.node_type,
                NodeId::random(),
            )
            .expect("Failed to create node in database");

        let mut test_node_entity = TestNodeEntity {
            id: node.id,
            active_revision: None,
        };

        if self.node_type == NodeType::File {
            let mut rng = rand::rng();
            // gen 1-10 chunks if not specified
            let chunk_count = self.chunk_count.unwrap_or_else(|| rng.random_range(1..=10));

            let revision = self
                .state
                .revision_repository
                .create_revision(
                    node.id,
                    chrono::Local::now().naive_local(),
                    IV::random(),
                    chunk_count as i64,
                )
                .expect("Failed to create revision in database");

            let mut test_revision_entity = TestRevisionEntity {
                id: revision.id,
                chunks: Vec::new(),
            };

            let mut vfs = self.state.vfs.write().await;
            vfs.create_file(&revision.id).await.expect("Failed to create file in VFS");

            for index in 0..chunk_count {
                let mut bytes = BytesMut::with_capacity(self.chunk_size);
                bytes.resize(self.chunk_size, 0x00);
                rng.fill_bytes(&mut bytes);

                test_revision_entity.chunks.push(TestChunk {
                    checksum: format!("{:x}", Sha256::digest(&bytes)),
                });

                vfs.write_chunk(
                    &revision.id,
                    FileChunk {
                        index: index as ChunkIndex,
                        data: bytes.into(),
                    },
                )
                .await
                .expect("Failed to write chunk to VFS");
            }

            vfs.commit_file(&revision.id).await.expect("Failed to commit VFS file");

            node.current_revision = Some(revision.id);
            self.state
                .node_repository
                .update_node(&node)
                .expect("Failed to update node with current revision");

            test_node_entity.active_revision = Some(test_revision_entity);
        }

        test_node_entity
    }
}
