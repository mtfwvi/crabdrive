use crate::model::node::{DecryptedNode, MetadataV1, NodeMetadata};
use crate::{api, utils};

use anyhow::{Context, Result, anyhow};
use chrono::Local;
use crabdrive_common::payloads::node::request::folder::PostCreateFolderRequest;
use crabdrive_common::payloads::node::response::folder::PostCreateFolderResponse;
use crabdrive_common::storage::NodeId;
use tracing::debug_span;

/// Create a new folder node. Returns `Err` if called unauthenticated.
///
/// Returns the freshly created node.
pub async fn create_folder(parent: DecryptedNode, folder_name: String) -> Result<DecryptedNode> {
    let _guard = debug_span!("api::createFile").entered();

    let folder_metadata = NodeMetadata::V1(MetadataV1 {
        name: folder_name,
        last_modified: Local::now().naive_local(),
        created: Local::now().naive_local(),
        size: None,
        mime_type: None,
        file_key: None,
        children_key: vec![],
    });

    let metadata_encryption_key = utils::encryption::generate_aes256_key()
        .await
        .inspect_err(|e| tracing::error!("Failed to generate AES256 key: {}", e))?;

    let new_node_id = NodeId::random();

    let encrypted_metadata =
        utils::encryption::node::encrypt_metadata(&folder_metadata, &metadata_encryption_key)
            .await
            .inspect_err(|e| tracing::error!("Failed to encrypt metadata: {}", e))?;

    let mut new_parent_metadata = parent.metadata.clone();

    match new_parent_metadata {
        NodeMetadata::V1(ref mut metadata) => metadata
            .children_key
            .push((new_node_id, metadata_encryption_key)),
    }

    let encrypted_parent_metadata =
        utils::encryption::node::encrypt_metadata(&new_parent_metadata, &parent.encryption_key)
            .await
            .inspect_err(|e| tracing::error!("Failed to encrypt parent metadata: {}", e))?;

    let request_body = PostCreateFolderRequest {
        parent_metadata_version: parent.change_count,
        parent_metadata: encrypted_parent_metadata.clone(),
        node_metadata: encrypted_metadata.clone(),
        node_id: new_node_id,
    };

    let response = api::requests::folder::post_create_folder(parent.id, request_body)
        .await
        .inspect_err(|e| tracing::error!("Failed to post to create_folder: {}", e))?;

    match response {
        PostCreateFolderResponse::Created(new_folder) => {
            let decrypted_node =
                utils::encryption::node::decrypt_node(new_folder, metadata_encryption_key)
                    .await
                    .context("Failed to decrypt node")
                    .inspect_err(|e| tracing::error!("Failed to decrypt node metadata: {}", e))?;

            Ok(decrypted_node)
        }
        PostCreateFolderResponse::NotFound => Err(anyhow!(
            "No such node: {}. Check if you have permission to access it",
            parent.id
        )),
        PostCreateFolderResponse::BadRequest => Err(anyhow!("Bad Request")),
        PostCreateFolderResponse::Conflict => Err(anyhow!("Please try again")),
    }
}
