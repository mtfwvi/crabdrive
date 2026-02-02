use crate::api::requests::node::get_node_children;
use crate::constants::EMPTY_KEY;
use crate::model::node::DecryptedNode;
use crate::utils::encryption::node::decrypt_node;
use crabdrive_common::payloads::node::response::node::GetNodeChildrenResponse;
use anyhow::{anyhow, Context, Result};

pub async fn get_children(parent: DecryptedNode) -> Result<Vec<DecryptedNode>> {
    let response_result = get_node_children(parent.id, &"".to_string()).await;

    if let Err(err) = response_result {
        return Err(anyhow!("Could not query children: {:?}", err));
    }

    let response = response_result?;
    match response {
        GetNodeChildrenResponse::Ok(children) => {
            let mut decrypted_children = Vec::with_capacity(children.len());

            for child in children {
                let decrypted_child = decrypt_node(child, EMPTY_KEY).await.context("Could not decrypt node")?;
                decrypted_children.push(decrypted_child);
            }

            Ok(decrypted_children)
        }
        GetNodeChildrenResponse::NotFound => Err(anyhow!("Could not query children: 404")),
    }
}
