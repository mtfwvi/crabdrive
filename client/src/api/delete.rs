use crate::api::requests::node::delete_node;
use crate::api::{get_children, get_trash_node};
use crate::model::node::{DecryptedNode, NodeMetadata};
use crate::utils::browser::SessionStorage;
use crate::utils::encryption::node::encrypt_metadata;
use anyhow::{Result, anyhow};
use crabdrive_common::payloads::node::request::node::DeleteNodeRequest;
use crabdrive_common::payloads::node::response::node::DeleteNodeResponse;

/// permanently delete a node that is in the trash
pub async fn delete_node_tree(node: DecryptedNode) -> Result<()> {
    if node.parent_id.is_none() {
        return Err(anyhow!("cannot delete root node"));
    }

    if !node.parent_id.eq(&SessionStorage::get("trash_id")?) {
        return Err(anyhow!(
            "cannot permanently delete node that is not in trash"
        ));
    }

    let mut parent = get_trash_node().await?;

    let NodeMetadata::V1(ref mut parent_metadata) = parent.metadata;

    parent_metadata.children_key.retain(|(x, _)| !node.id.eq(x));

    let encrypted_parent_metadata =
        encrypt_metadata(&parent.metadata, &parent.encryption_key).await?;

    let delete_node_request = DeleteNodeRequest {
        parent_change_count: parent.change_count,
        parent_node_metadata: encrypted_parent_metadata,
    };

    let response = delete_node(node.id, delete_node_request).await?;
    match response {
        DeleteNodeResponse::Ok => Ok(()),
        DeleteNodeResponse::NotFound => Err(anyhow!("server returned not found")),
        DeleteNodeResponse::Conflict => Err(anyhow!("reload the page and try again")),
    }
}

/// permanently delete all nodes in the trash
pub async fn empty_trash() -> Result<()> {
    // TODO this is completely inefficient
    // the solution would be to add a server request that allows you to upload a new
    // (empty) trash node/metadata and possible a list of nodes to be deleted

    let trash = get_trash_node().await?;
    let trashed_items = get_children(trash).await?;

    for item in trashed_items {
        delete_node_tree(item).await?;
    }

    Ok(())
}
