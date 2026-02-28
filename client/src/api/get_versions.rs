use crate::api::requests::node::get_file_versions;
use anyhow::{Result, anyhow};
use crabdrive_common::payloads::node::response::file::GetVersionsResponse;
use crabdrive_common::storage::{FileRevision, NodeId};

pub async fn file_versions(node_id: NodeId) -> Result<Vec<FileRevision>> {
    let response = get_file_versions(node_id).await?;

    match response {
        GetVersionsResponse::Ok(versions) => Ok(versions),
        GetVersionsResponse::NotFound => {
            Err(anyhow!("server returned notfound when querying versions"))
        }
    }
}
