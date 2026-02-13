use crate::api::requests::node::patch_node;
use crate::model::node::{DecryptedNode, MetadataV1, NodeMetadata};
use crate::utils;
use crate::utils::encryption::node::encrypt_metadata;
use anyhow::{Result, anyhow};
use crabdrive_common::payloads::node::request::node::PatchNodeRequest;
use crabdrive_common::payloads::node::response::node::PatchNodeResponse;
use tracing::debug_span;

pub async fn rename_node(
    node: DecryptedNode,
    parent: DecryptedNode,
    new_name: String,
) -> Result<()> {
    let _guard = debug_span!("api::renameNode").entered();
    let token = utils::auth::get_token()
        .inspect_err(|_| tracing::error!("No token found. Is the user authenticated?"))?;

    let metadata_key = match &parent.metadata {
        NodeMetadata::V1(metadata) => metadata.children_key.iter(),
    }
    .find(|(id, _)| *id == node.id)
    .ok_or(anyhow!("Failed to get metadata key"))
    .inspect_err(|_| tracing::error!("Failed to find key in parent metadata"))?;

    let NodeMetadata::V1(old_metadata) = node.metadata;
    let new_metadata = NodeMetadata::V1(MetadataV1 {
        name: new_name,
        ..old_metadata
    });

    let encrypted_metadata = encrypt_metadata(&new_metadata, &metadata_key.1)
        .await
        .inspect_err(|e| tracing::error!("Failed to encrypt metadata: {}", e))?;

    let request_body = PatchNodeRequest {
        node_metadata: encrypted_metadata.clone(),
        node_change_count: node.change_count,
    };

    let response = patch_node(node.id, request_body, &token)
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
