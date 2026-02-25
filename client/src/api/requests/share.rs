use crate::api::requests::{RequestMethod, json_api_request};
use anyhow::Result;
use crabdrive_common::payloads::node::request::share::{
    PostAcceptShareRequest, PostShareNodeRequest,
};
use crabdrive_common::payloads::node::response::share::{
    GetAcceptedSharedResponse, GetNodeShareInfo, GetAcceptShareInfoResponse,
    PostAcceptShareResponse, PostShareNodeResponse,
};
use crabdrive_common::routes;
use crabdrive_common::storage::{NodeId, ShareId};

pub async fn post_share_node(
    node_id: NodeId,
    body: PostShareNodeRequest,
) -> Result<PostShareNodeResponse> {
    let url = routes::node::share::share(node_id);

    json_api_request(url, RequestMethod::POST, body).await
}

pub async fn get_share_info(share_id: ShareId) -> Result<GetAcceptShareInfoResponse> {
    let url = routes::node::share::get_share_accept_info(share_id);

    json_api_request(url, RequestMethod::GET, ()).await
}

pub async fn get_node_share_info(node_id: NodeId) -> Result<GetNodeShareInfo> {
    let url = routes::node::share::get_node_share_info(node_id);

    json_api_request(url, RequestMethod::GET, ()).await
}

pub async fn post_accept_share(
    share_id: ShareId,
    body: PostAcceptShareRequest,
) -> Result<PostAcceptShareResponse> {
    let url = routes::node::share::accept_share(share_id);

    json_api_request(url, RequestMethod::POST, body).await
}

pub async fn get_accepted_shared_nodes() -> Result<GetAcceptedSharedResponse> {
    let url = routes::node::share::get_accepted_shared();

    json_api_request(url, RequestMethod::GET, ()).await
}
