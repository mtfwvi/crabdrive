use crate::api::requests::share::post_share_node;
use crate::model::node::DecryptedNode;
use crate::utils::encryption::{generate_aes256_key, wrap_key};
use crate::utils::share::create_share_url;
use anyhow::Result;
use crabdrive_common::payloads::node::request::share::PostShareNodeRequest;
use crabdrive_common::payloads::node::response::share::PostShareNodeResponse;

/// returns the url that a user can use to accept the share
pub async fn share_node(node: &DecryptedNode) -> Result<String> {
    let encryption_key = generate_aes256_key().await?;

    // TODO maybe change the type of wrap key to be more consistent
    let wrapped_metadata_key = wrap_key(&node.encryption_key, &encryption_key).await?;

    let body = PostShareNodeRequest {
        wrapped_metadata_key,
    };

    let response = post_share_node(node.id, body).await?;

    match response {
        PostShareNodeResponse::NotFound => {
            Err(anyhow::anyhow!("server returned NotFound on share node"))
        }
        PostShareNodeResponse::Ok(share_id) => {
            let url = create_share_url(&share_id, &encryption_key);
            Ok(url)
        }
        PostShareNodeResponse::BadRequest(error) => {
            Err(anyhow::anyhow!("Server returned BadRequest: {:?}", error))
        }
    }
}
