use crate::constants::EMPTY_KEY;
use crate::model::node::DecryptedNode;
use crate::utils::encryption::node::decrypt_node;
use crate::{api, utils};
use anyhow::{Context, Result, anyhow};
use crabdrive_common::payloads::node::response::node::GetNodeChildrenResponse;

pub async fn get_children(parent: DecryptedNode) -> Result<Vec<DecryptedNode>> {
    let token = utils::auth::get_token()?;

    let response = api::requests::node::get_node_children(parent.id, &token)
        .await
        .context("Failed to get children")?;

    match response {
        GetNodeChildrenResponse::Ok(children) => {
            let mut decrypted_children = Vec::with_capacity(children.len());

            for child in children {
                // TODO: Decrypt nodes
                let decrypted_child = decrypt_node(child, EMPTY_KEY)
                    .await
                    .context("Could not decrypt node")?;
                decrypted_children.push(decrypted_child);
            }

            Ok(decrypted_children)
        }
        GetNodeChildrenResponse::NotFound => Err(anyhow!("Could not query children: 404")),
    }
}
