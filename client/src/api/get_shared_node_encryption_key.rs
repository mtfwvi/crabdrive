use crate::api::requests::share::get_node_share_info;
use anyhow::Result;
use crabdrive_common::encryption_key::EncryptionKey;
use crabdrive_common::payloads::node::response::share::GetNodeShareInfo;
use crabdrive_common::storage::NodeId;

pub async fn get_shared_node_encryption_key(node_id: NodeId) -> Result<EncryptionKey> {
    let response = get_node_share_info(node_id).await?;

    match response {
        GetNodeShareInfo::Ok(encryption_info) => Ok(encryption_info.wrapped_metadata_key),
        GetNodeShareInfo::NotFound => Err(anyhow::anyhow!(
            "Server returned NotFound on get_node_share_info"
        )),
    }
}
