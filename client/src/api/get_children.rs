use crate::api::requests::node::get_node_children;
use crate::constants::EMPTY_KEY;
use crate::model::node::DecryptedNode;
use crate::utils::encryption::node::decrypt_node;
use crabdrive_common::payloads::node::response::node::GetNodeChildrenResponse;
use crabdrive_common::storage::NodeId;

pub async fn get_children(parent_id: NodeId) -> Result<Vec<DecryptedNode>, String> {
    let response_result = get_node_children(parent_id, &"".to_string()).await;

    if let Err(err) = response_result {
        return Err(format!("Could not query children: {:?}", err));
    }

    let response = response_result.unwrap();
    match response {
        GetNodeChildrenResponse::Ok(children) => {
            let mut decrypted_children = Vec::with_capacity(children.len());

            for child in children {
                let decrypted_child = decrypt_node(child, EMPTY_KEY).await;
                if let Ok(decrypted_child) = decrypted_child {
                    decrypted_children.push(decrypted_child);
                } else {
                    return Err(format!(
                        "Failed to decrypt node: {:?}",
                        decrypted_child.err().unwrap()
                    ));
                }
            }

            Ok(decrypted_children)
        }
        GetNodeChildrenResponse::NotFound => Err("Could not query children: 404".to_string()),
    }
}
