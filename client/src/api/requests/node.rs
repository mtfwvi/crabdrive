use crate::api::requests::{RequestBody, RequestMethod, request, string_from_response};
use anyhow::Result;
use crabdrive_common::payloads::node::request::node::{
    DeleteNodeRequest, PatchNodeRequest, PostMoveNodeOutOfTrashRequest, PostMoveNodeRequest,
    PostMoveNodeToTrashRequest,
};
use crabdrive_common::payloads::node::response::node::{
    DeleteNodeResponse, GetNodeChildrenResponse, GetNodeResponse, GetPathBetweenNodesResponse,
    PatchNodeResponse, PostMoveNodeOutOfTrashResponse, PostMoveNodeResponse,
    PostMoveNodeToTrashResponse,
};
use crabdrive_common::storage::NodeId;
use web_sys::Response;

pub async fn delete_node(
    node_id: NodeId,
    body: DeleteNodeRequest,
    token: &String,
) -> Result<DeleteNodeResponse> {
    let url = crabdrive_common::routes::node::by_id(node_id);

    let request_method = RequestMethod::DELETE;
    let body = RequestBody::Json(serde_json::to_string(&body)?);
    let query_parameters = vec![];
    let auth_token = Some(token);

    let response: Response = request(
        url,
        request_method,
        body,
        query_parameters,
        auth_token,
        true,
    )
    .await?;
    let response_string = string_from_response(response).await?;

    let response_object = serde_json::from_str(&response_string)?;
    Ok(response_object)
}

pub async fn get_node(node_id: NodeId, token: &String) -> Result<GetNodeResponse> {
    let url = crabdrive_common::routes::node::by_id(node_id);

    let request_method = RequestMethod::GET;
    let body = RequestBody::Empty;
    let query_parameters = vec![];
    let auth_token = Some(token);

    let response: Response = request(
        url,
        request_method,
        body,
        query_parameters,
        auth_token,
        true,
    )
    .await?;
    let response_string = string_from_response(response).await?;

    let response_object = serde_json::from_str(&response_string)?;
    Ok(response_object)
}

pub async fn patch_node(
    node_id: NodeId,
    body: PatchNodeRequest,
    token: &String,
) -> Result<PatchNodeResponse> {
    let url = crabdrive_common::routes::node::by_id(node_id);

    let request_method = RequestMethod::PATCH;
    let body = RequestBody::Json(serde_json::to_string(&body)?);
    let query_parameters = vec![];
    let auth_token = Some(token);

    let response: Response = request(
        url,
        request_method,
        body,
        query_parameters,
        auth_token,
        true,
    )
    .await?;
    let response_string = string_from_response(response).await?;

    let response_object = serde_json::from_str(&response_string)?;
    Ok(response_object)
}

pub async fn get_node_children(
    parent_id: NodeId,
    token: &String,
) -> Result<GetNodeChildrenResponse> {
    let url = crabdrive_common::routes::node::children(parent_id);

    let request_method = RequestMethod::GET;
    let body = RequestBody::Empty;
    let query_parameters = vec![];
    let auth_token = Some(token);

    let response: Response = request(
        url,
        request_method,
        body,
        query_parameters,
        auth_token,
        true,
    )
    .await?;
    let response_string = string_from_response(response).await?;

    let response_object = serde_json::from_str(&response_string)?;
    Ok(response_object)
}

pub async fn post_move_node(
    node_id: NodeId,
    body: PostMoveNodeRequest,
    token: &String,
) -> Result<PostMoveNodeResponse> {
    let url = crabdrive_common::routes::node::move_to(node_id);

    let request_method = RequestMethod::POST;
    let body = RequestBody::Json(serde_json::to_string(&body)?);
    let query_parameters = vec![];
    let auth_token = Some(token);

    let response: Response = request(
        url,
        request_method,
        body,
        query_parameters,
        auth_token,
        true,
    )
    .await?;
    let response_string = string_from_response(response).await?;

    let response_object = serde_json::from_str(&response_string)?;
    Ok(response_object)
}

pub async fn post_move_node_to_trash(
    node_id: NodeId,
    body: PostMoveNodeToTrashRequest,
    token: &String,
) -> Result<PostMoveNodeToTrashResponse> {
    let url = crabdrive_common::routes::node::move_to_trash(node_id);

    let request_method = RequestMethod::POST;
    let body = RequestBody::Json(serde_json::to_string(&body)?);
    let query_parameters = vec![];
    let auth_token = Some(token);

    let response: Response = request(
        url,
        request_method,
        body,
        query_parameters,
        auth_token,
        true,
    )
    .await?;
    let response_string = string_from_response(response).await?;

    let response_object = serde_json::from_str(&response_string)?;
    Ok(response_object)
}

pub async fn post_move_node_out_of_trash(
    node_id: NodeId,
    body: PostMoveNodeOutOfTrashRequest,
    token: &String,
) -> Result<PostMoveNodeOutOfTrashResponse> {
    let url = crabdrive_common::routes::node::move_out_of_trash(node_id);

    let request_method = RequestMethod::POST;
    let body = RequestBody::Json(serde_json::to_string(&body)?);
    let query_parameters = vec![];
    let auth_token = Some(token);

    let response: Response = request(
        url,
        request_method,
        body,
        query_parameters,
        auth_token,
        true,
    )
    .await?;
    let response_string = string_from_response(response).await?;

    let response_object = serde_json::from_str(&response_string)?;
    Ok(response_object)
}

pub async fn get_path_between_nodes(
    from_id: NodeId,
    to_id: NodeId,
    token: &String,
) -> Result<GetPathBetweenNodesResponse> {
    // Arguments are reserved for future use
    let url = crabdrive_common::routes::node::path_between_nodes(from_id, to_id);

    let request_method = RequestMethod::GET;
    let body = RequestBody::Empty;
    let query_parameters = vec![];
    let auth_token = Some(token);

    let response: Response = request(
        url,
        request_method,
        body,
        query_parameters,
        auth_token,
        true,
    )
    .await?;
    let response_string = string_from_response(response).await?;

    let response_object = serde_json::from_str(&response_string)?;
    Ok(response_object)
}
