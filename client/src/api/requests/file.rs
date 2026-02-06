use crate::api::requests::{RequestBody, RequestMethod, request, string_from_response};
use crabdrive_common::payloads::node::request::file::{
    PostCreateFileRequest, PostUpdateFileRequest,
};
use crabdrive_common::payloads::node::response::file::{
    GetVersionsResponse, PostCommitFileResponse, PostCreateFileResponse, PostUpdateFileResponse,
};
use crabdrive_common::storage::{NodeId, RevisionId};
use wasm_bindgen::JsValue;
use web_sys::Response;

pub async fn post_create_file(
    parent_id: NodeId,
    body: PostCreateFileRequest,
    token: &String,
) -> Result<PostCreateFileResponse, JsValue> {
    let url = crabdrive_common::routes::node::file::create(parent_id);

    let request_method = RequestMethod::POST;
    let body = RequestBody::Json(serde_json::to_string(&body).unwrap());
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

    let response_object = serde_json::from_str(&response_string).unwrap();
    Ok(response_object)
}

pub async fn post_update_file(
    node_id: NodeId,
    body: PostUpdateFileRequest,
    token: &String,
) -> Result<PostUpdateFileResponse, JsValue> {
    let url = crabdrive_common::routes::node::file::update(node_id);

    let request_method = RequestMethod::POST;
    let body = RequestBody::Json(serde_json::to_string(&body).unwrap());
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

    let response_object = serde_json::from_str(&response_string).unwrap();
    Ok(response_object)
}

pub async fn post_commit_file(
    node_id: NodeId,
    version_id: RevisionId,
    token: &String,
) -> Result<PostCommitFileResponse, JsValue> {
    let url = crabdrive_common::routes::node::file::commit(node_id, version_id);

    let request_method = RequestMethod::POST;

    //TODO if this works with current server routes
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

    let response_object = serde_json::from_str(&response_string).unwrap();
    Ok(response_object)
}

pub async fn get_file_versions(
    node_id: NodeId,
    token: &String,
) -> Result<GetVersionsResponse, JsValue> {
    let url = crabdrive_common::routes::node::versions(node_id);

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

    let response_object = serde_json::from_str(&response_string).unwrap();
    Ok(response_object)
}
