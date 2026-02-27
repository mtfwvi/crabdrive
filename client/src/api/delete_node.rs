use crate::api::get_accessible_path;
use crate::api::requests::node::delete_node;
use crate::model::node::NodeMetadata;
use crate::utils::encryption::node::encrypt_metadata;
use anyhow::{anyhow, Result};
use crabdrive_common::payloads::node::request::node::DeleteNodeRequest;
use crabdrive_common::payloads::node::response::node::DeleteNodeResponse;
use crabdrive_common::storage::NodeId;

pub async fn delete_node_tree(node_id: NodeId) -> Result<()> {
    let mut path_to_node = get_accessible_path(node_id).await?;
    if path_to_node.len() < 2 {
        return Err(anyhow!(
            "you do not have access to the parent of the node you are trying to delete"
        ));
    }
    path_to_node.reverse();

    let parent = &mut path_to_node[1];

    let NodeMetadata::V1(ref mut parent_metadata) = parent.metadata;

    parent_metadata.children_key.retain(|(x, _)| !node_id.eq(x));

    let encrypted_parent_metadata =
        encrypt_metadata(&parent.metadata, &parent.encryption_key).await?;

    let delete_node_request = DeleteNodeRequest {
        parent_change_count: parent.change_count,
        parent_node_metadata: encrypted_parent_metadata,
    };

    let response = delete_node(node_id, delete_node_request).await?;
    match response {
        DeleteNodeResponse::Ok => Ok(()),
        DeleteNodeResponse::NotFound => Err(anyhow!("server returned not found")),
        DeleteNodeResponse::Conflict => Err(anyhow!("reload the page and try again")),
    }
}
