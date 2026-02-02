use crate::api::requests::node::get_node;
use crate::constants::EMPTY_KEY;
use crate::model::node::{DecryptedNode, MetadataV1, NodeMetadata};
use crate::utils::encryption::node::decrypt_node;
use crabdrive_common::payloads::node::response::node::GetNodeResponse;
use crabdrive_common::storage::NodeId;
use anyhow::{anyhow, Result};

pub async fn get_root_node() -> Result<DecryptedNode> {
    let get_node_response= get_node(NodeId::nil(), &"".to_string()).await?;

    match get_node_response {
        GetNodeResponse::Ok(encrypted_node) => {
            if encrypted_node
                .encrypted_metadata
                .metadata()
                .eq(&[0, 0, 0, 0])
            {
                // TODO remove when actually implementing users, as each user should have a root

                // this is the empty node the server created on start => cannot be decrypted

                // metadata will be uploaded to the server during the next update (e.g. adding children)
                let root_node_metadata = NodeMetadata::V1(MetadataV1 {
                    name: "root".to_string(),
                    last_modified: Default::default(),
                    created: Default::default(),
                    size: None,
                    mime_type: None,
                    file_key: None,
                    children_key: vec![],
                });

                let decrypted_node = DecryptedNode {
                    id: encrypted_node.id,
                    change_count: 0,
                    parent_id: encrypted_node.parent_id,
                    owner_id: encrypted_node.owner_id,
                    deleted_on: encrypted_node.deleted_on,
                    node_type: encrypted_node.node_type,
                    current_revision: encrypted_node.current_revision,
                    metadata: root_node_metadata,
                    encryption_key: EMPTY_KEY,
                };

                Ok(decrypted_node)
            } else {
                let decrypted_node_result = decrypt_node(encrypted_node, EMPTY_KEY).await;

                if let Err(js_error) = decrypted_node_result {
                    return Err(anyhow!("could not decrypt node: {:?}", js_error));
                }

                let decrypted_node = decrypted_node_result?;
                Ok(decrypted_node)
            }
        }
        GetNodeResponse::NotFound => Err(anyhow!("root node returned 404")),
    }
}
