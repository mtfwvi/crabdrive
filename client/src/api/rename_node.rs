use crate::api::requests::node::patch_node;
use crate::model::node::{DecryptedNode, MetadataV1, NodeMetadata};
use crate::utils::encryption::node::encrypt_metadata;
use anyhow::{Result, anyhow};
use crabdrive_common::payloads::node::request::node::PatchNodeRequest;
use crabdrive_common::payloads::node::response::node::PatchNodeResponse;
use tracing::debug_span;

pub async fn rename_node(node: DecryptedNode, new_name: String) -> Result<()> {
    let _guard = debug_span!("api::renameNode").entered();

    let NodeMetadata::V1(old_metadata) = node.metadata;
    let new_metadata = NodeMetadata::V1(MetadataV1 {
        name: new_name,
        ..old_metadata
    });

    let encrypted_metadata = encrypt_metadata(&new_metadata, &node.encryption_key)
        .await
        .inspect_err(|e| tracing::error!("Failed to encrypt metadata: {}", e))?;

    let request_body = PatchNodeRequest {
        node_metadata: encrypted_metadata.clone(),
        node_change_count: node.change_count,
    };

    let response = patch_node(node.id, request_body)
        .await
        .inspect_err(|e| tracing::error!("Failed to patch to rename node: {}", e))?;

    match response {
        PatchNodeResponse::Ok(_) => Ok(()),
        PatchNodeResponse::NotFound => Err(anyhow!(
            "no such node: {}. Check if you have permission to access it",
            node.id
        )),
        PatchNodeResponse::Conflict => Err(anyhow!("Please try again")),
    }
}
