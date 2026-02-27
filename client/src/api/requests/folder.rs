use crate::api::requests::{RequestMethod, json_api_request};
use anyhow::Result;
use crabdrive_common::payloads::node::request::folder::PostCreateFolderRequest;
use crabdrive_common::payloads::node::response::folder::PostCreateFolderResponse;
use crabdrive_common::storage::NodeId;

pub async fn post_create_folder(
    parent_id: NodeId,
    body: PostCreateFolderRequest,
) -> Result<PostCreateFolderResponse> {
    let url = crabdrive_common::routes::node::folder::create(parent_id);
    json_api_request(&url, RequestMethod::POST, body).await
}
