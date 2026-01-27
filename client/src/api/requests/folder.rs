use crate::api::requests::{RequestBody, RequestMethod, request, string_from_response};
use crabdrive_common::payloads::node::request::folder::PostCreateFolderRequest;
use crabdrive_common::payloads::node::response::folder::PostCreateFolderResponse;
use crabdrive_common::storage::NodeId;
use formatx::formatx;
use wasm_bindgen::JsValue;
use web_sys::Response;

pub async fn post_create_folder(
    parent_id: NodeId,
    body: PostCreateFolderRequest,
    token: &String,
) -> Result<PostCreateFolderResponse, JsValue> {
    let url = formatx!(crabdrive_common::routes::CREATE_FOLDER_ROUTE, parent_id).unwrap();

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
