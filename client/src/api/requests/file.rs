use crate::api::requests::{json_api_request, RequestMethod};
use anyhow::Result;
use crabdrive_common::payloads::node::request::file::{
    PostCreateFileRequest, PostUpdateFileRequest,
};
use crabdrive_common::payloads::node::response::file::{
    PostCommitFileResponse, PostCreateFileResponse, PostUpdateFileResponse,
};
use crabdrive_common::storage::{NodeId, RevisionId};

pub async fn post_create_file(
    parent_id: NodeId,
    body: PostCreateFileRequest,
) -> Result<PostCreateFileResponse> {
    let url = crabdrive_common::routes::node::file::create(parent_id);
    json_api_request(&url, RequestMethod::POST, body).await
}

pub async fn post_update_file(
    node_id: NodeId,
    body: PostUpdateFileRequest,
) -> Result<PostUpdateFileResponse> {
    let url = crabdrive_common::routes::node::file::update(node_id);
    json_api_request(&url, RequestMethod::POST, body).await
}

pub async fn post_commit_file(
    node_id: NodeId,
    version_id: RevisionId,
) -> Result<PostCommitFileResponse> {
    let url = crabdrive_common::routes::node::file::commit(node_id, version_id);
    json_api_request(&url, RequestMethod::POST, ()).await
}