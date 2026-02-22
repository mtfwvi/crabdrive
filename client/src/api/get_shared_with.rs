use crate::api::requests::share::get_node_shared_with;
use anyhow::Result;
use crabdrive_common::payloads::node::response::share::GetNodeSharedWithResponse;
use crabdrive_common::storage::NodeId;

pub async fn get_shared_with(node_id: NodeId) -> Result<Vec<String>> {
    let response = get_node_shared_with(node_id).await?;

    match response {
        GetNodeSharedWithResponse::Ok(usernames) => {
            Ok(usernames)
        }
        GetNodeSharedWithResponse::NotFound => {
            Err(anyhow::anyhow!("Server returned NotFound on get_shared_with"))
        }
    }
}