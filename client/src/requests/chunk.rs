use crate::requests::{RequestBody, RequestMethod, request, uint8array_from_response};
use crabdrive_common::storage::{ChunkIndex, NodeId, RevisionId};
use formatx::formatx;
use wasm_bindgen::JsValue;
use web_sys::Response;
use web_sys::js_sys::Uint8Array;

pub enum GetChunkResponse {
    Ok(Uint8Array),
    NotFound,
}

pub enum PostChunkResponse {
    Created,
    NotFound,
    BadRequest,
    Conflict,
    OutOfStorage,
}

pub async fn get_chunk(
    node_id: NodeId,
    version_id: RevisionId,
    chunk_index: ChunkIndex,
    token: &String,
) -> Result<GetChunkResponse, JsValue> {
    let url = formatx!(
        crabdrive_common::routes::CHUNK_ROUTE,
        node_id,
        version_id,
        chunk_index
    )
    .unwrap();

    let request_method = RequestMethod::GET;
    let body = RequestBody::Empty;
    let query_parameters = vec![];
    let auth_token = Some(token);

    let response: Response =
        request(url, request_method, body, query_parameters, auth_token).await?;
    let parsed_response = match response.status() {
        200 => GetChunkResponse::Ok(uint8array_from_response(response).await?),
        404 => GetChunkResponse::NotFound,
        _ => unreachable!("the error code cannot be handled"),
    };

    Ok(parsed_response)
}

pub async fn post_chunk(
    node_id: NodeId,
    version_id: RevisionId,
    chunk_index: ChunkIndex,
    body: Uint8Array,
    token: &String,
) -> Result<PostChunkResponse, JsValue> {
    let url = formatx!(
        crabdrive_common::routes::CHUNK_ROUTE,
        node_id,
        version_id,
        chunk_index
    )
    .unwrap();

    let request_method = RequestMethod::POST;
    let body = RequestBody::Bytes(body);
    let query_parameters = vec![];
    let auth_token = Some(token);

    let response: Response =
        request(url, request_method, body, query_parameters, auth_token).await?;
    let parsed_response = match response.status() {
        201 => PostChunkResponse::Created,
        404 => PostChunkResponse::NotFound,
        400 => PostChunkResponse::BadRequest,
        409 => PostChunkResponse::Conflict,
        413 => PostChunkResponse::OutOfStorage,
        _ => unreachable!("the error code cannot be handled"),
    };

    Ok(parsed_response)
}
