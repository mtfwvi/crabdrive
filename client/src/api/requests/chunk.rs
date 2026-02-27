use crate::api::requests::{RequestBody, RequestMethod, request, uint8array_from_response};
use crate::utils::auth::get_token;
use anyhow::{Result, anyhow};
use crabdrive_common::storage::{ChunkIndex, NodeId, RevisionId};
use web_sys::Response;
use web_sys::js_sys::Uint8Array;

#[derive(Debug)]
pub enum GetChunkResponse {
    Ok(Uint8Array),
    NotFound,
}

#[derive(Debug)]
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
) -> Result<GetChunkResponse> {
    let url = crabdrive_common::routes::node::chunks(node_id, version_id, chunk_index);

    let request_method = RequestMethod::GET;
    let body = RequestBody::Empty;
    let auth_token = Some(token);

    let response: Response = request(&url, request_method, body, auth_token, true).await?;

    let parsed_response = match response.status() {
        200 => GetChunkResponse::Ok(uint8array_from_response(response).await?),
        404 => GetChunkResponse::NotFound,
        _ => {
            return Err(anyhow!(
                "unexpected status code on get chunk: {}",
                response.status()
            ));
        }
    };

    Ok(parsed_response)
}

pub async fn post_chunk(
    node_id: NodeId,
    version_id: RevisionId,
    chunk_index: ChunkIndex,
    body: Uint8Array,
) -> Result<PostChunkResponse> {
    let token = get_token()?;
    let url = crabdrive_common::routes::node::chunks(node_id, version_id, chunk_index);

    let request_method = RequestMethod::POST;
    let body = RequestBody::Bytes(body);
    let auth_token = Some(&token);

    let response: Response = request(&url, request_method, body, auth_token, true).await?;

    let parsed_response = match response.status() {
        201 => PostChunkResponse::Created,
        404 => PostChunkResponse::NotFound,
        400 => PostChunkResponse::BadRequest,
        409 => PostChunkResponse::Conflict,
        413 => PostChunkResponse::OutOfStorage,
        _ => {
            return Err(anyhow!(
                "unexpected status code on post chunk: {}",
                response.status()
            ));
        }
    };

    Ok(parsed_response)
}
