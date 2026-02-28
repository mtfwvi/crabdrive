use crate::api::requests::{RequestMethod, json_api_request};
use anyhow::Result;
use crabdrive_common::payloads::node::request::node::{
    DeleteNodeRequest, PatchNodeRequest, PostMoveNodeOutOfTrashRequest, PostMoveNodeRequest,
    PostMoveNodeToTrashRequest,
};
use crabdrive_common::payloads::node::response::file::GetVersionsResponse;
use crabdrive_common::payloads::node::response::node::{
    DeleteNodeResponse, GetAccessiblePathResponse, GetNodeChildrenResponse, GetNodeResponse,
    GetPathBetweenNodesResponse, PatchNodeResponse, PostMoveNodeOutOfTrashResponse,
    PostMoveNodeResponse, PostMoveNodeToTrashResponse,
};
use crabdrive_common::routes;
use crabdrive_common::storage::NodeId;

pub async fn delete_node(node_id: NodeId, body: DeleteNodeRequest) -> Result<DeleteNodeResponse> {
    let url = routes::node::by_id(node_id);
    json_api_request(&url, RequestMethod::DELETE, body).await
}

pub async fn get_node(node_id: NodeId) -> Result<GetNodeResponse> {
    let url = routes::node::by_id(node_id);
    json_api_request(&url, RequestMethod::GET, ()).await
}

pub async fn patch_node(node_id: NodeId, body: PatchNodeRequest) -> Result<PatchNodeResponse> {
    let url = routes::node::by_id(node_id);
    json_api_request(&url, RequestMethod::PATCH, body).await
}

pub async fn get_node_children(parent_id: NodeId) -> Result<GetNodeChildrenResponse> {
    let url = routes::node::children(parent_id);
    json_api_request(&url, RequestMethod::GET, ()).await
}

pub async fn post_move_node(
    node_id: NodeId,
    body: PostMoveNodeRequest,
) -> Result<PostMoveNodeResponse> {
    let url = routes::node::move_to(node_id);
    json_api_request(&url, RequestMethod::POST, body).await
}

pub async fn post_move_node_to_trash(
    node_id: NodeId,
    body: PostMoveNodeToTrashRequest,
) -> Result<PostMoveNodeToTrashResponse> {
    let url = routes::node::move_to_trash(node_id);
    json_api_request(&url, RequestMethod::POST, body).await
}

pub async fn post_move_node_out_of_trash(
    node_id: NodeId,
    body: PostMoveNodeOutOfTrashRequest,
) -> Result<PostMoveNodeOutOfTrashResponse> {
    let url = routes::node::move_out_of_trash(node_id);
    json_api_request(&url, RequestMethod::POST, body).await
}

//TODO this could probably be removed
pub async fn get_path_between_nodes(
    from_id: NodeId,
    to_id: NodeId,
) -> Result<GetPathBetweenNodesResponse> {
    // Arguments are reserved for future use
    let url = routes::node::path_between_nodes(from_id, to_id);
    json_api_request(&url, RequestMethod::GET, ()).await
}

pub async fn get_accessible_path(node_id: NodeId) -> Result<GetAccessiblePathResponse> {
    let url = routes::node::accessible_path(node_id);
    json_api_request(&url, RequestMethod::GET, ()).await
}

pub async fn get_file_versions(node_id: NodeId) -> Result<GetVersionsResponse> {
    let url = routes::node::versions(node_id);
    json_api_request(&url, RequestMethod::GET, ()).await
}
