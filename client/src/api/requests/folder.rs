use crate::api::requests::{RequestBody, RequestMethod, request, string_from_response};
use anyhow::Result;
use crabdrive_common::payloads::node::request::folder::PostCreateFolderRequest;
use crabdrive_common::payloads::node::response::folder::PostCreateFolderResponse;
use crabdrive_common::storage::NodeId;
use web_sys::Response;

pub async fn post_create_folder(
    parent_id: NodeId,
    body: PostCreateFolderRequest,
    token: &String,
) -> Result<PostCreateFolderResponse> {
    let url = crabdrive_common::routes::node::folder::create(parent_id);

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
