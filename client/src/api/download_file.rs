use crate::api::requests::chunk::GetChunkResponse;
use crate::model::chunk::EncryptedChunk;
use crate::model::encryption::FileKey;
use crate::model::node::{DecryptedNode, NodeMetadata};
use crate::utils::error::wrap_js_err;
use crate::{api, utils};
use anyhow::{Result, anyhow};
use crabdrive_common::storage::{ChunkIndex, FileRevision, NodeId};
use tracing::debug_span;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::js_sys::Uint8Array;
use web_sys::{Blob, Url};

pub async fn download_file(node: DecryptedNode) -> Result<()> {
    let _guard = debug_span!("api::downloadFile").entered();
    let token = utils::auth::get_token()
        .inspect_err(|_| tracing::error!("No token found. Is the user authenticated?"))?;

    // TODO: Support chunked downloads in chrom(e/ium)

    let file_key = match &node.metadata {
        NodeMetadata::V1(n) => n.file_key,
    }
    .ok_or(anyhow!("Cannot download folders or symlinks"))
    .inspect_err(|_| tracing::error!("Cannot download folders or symlinks."))?;

    let current_revision = node
        .current_revision
        .ok_or(anyhow!("This node does not have any file contents"))
        .inspect_err(|_| tracing::error!("Node has no revision associated with it."))?;

    if current_revision.upload_ended_on.is_none() {
        // Revision has not finished uploading
        tracing::error!("Cannot download file, which is still uploading.");
        return Err(anyhow!(
            "The file is still uploading. Please try again later"
        ));
    }

    let mut chunks = Vec::with_capacity(current_revision.chunk_count as usize);

    web_sys::console::time_with_label("Download time");
    for i in 1..=current_revision.chunk_count {
        let decrypted_chunk_result =
            download_chunk_and_decrypt(node.id, &current_revision, &file_key, i, &token)
                .await
                .inspect_err(|e| tracing::error!("Failed to download chunks: {}", e))?;
        chunks.push(decrypted_chunk_result);
    }
    web_sys::console::time_end_with_label("Download time");

    let name = match &node.metadata {
        NodeMetadata::V1(n) => &n.name,
    };

    save_file(
        utils::file::combine_chunks(chunks)
            .inspect_err(|e| tracing::error!("Failed to combine decrypted file chunks: {}", e))?,
        name,
    )
    .await
}

async fn download_chunk_and_decrypt(
    node_id: NodeId,
    revision: &FileRevision,
    file_key: &FileKey,
    index: ChunkIndex,
    token: &String,
) -> Result<Uint8Array> {
    let _guard = debug_span!("downloadChunkAndDecrypt").entered();

    let chunk_response = api::requests::chunk::get_chunk(node_id, revision.id, index, token)
        .await
        .inspect_err(|e| tracing::error!("Failed to get chunk from server: {}", e))?;

    match chunk_response {
        GetChunkResponse::Ok(encrypted_chunk_buffer) => {
            let encrypted_chunk = EncryptedChunk {
                chunk: encrypted_chunk_buffer,
                index,
                first_block: index == 1,
                last_block: index == revision.chunk_count,
                iv_prefix: revision.iv,
            };

            let decrypted_chunk =
                utils::encryption::chunk::decrypt_chunk(&encrypted_chunk, file_key)
                    .await
                    .inspect_err(|e| tracing::error!("Failed to decrypt chunk contents: {}", e))?;

            Ok(Uint8Array::new(&decrypted_chunk.chunk))
        }
        GetChunkResponse::NotFound => {
            //TODO correct error handling
            Err(anyhow!("Chunk {index} does not exist"))
        }
    }
}

async fn save_file(data: Blob, file_name: &str) -> Result<()> {
    let url = wrap_js_err(Url::create_object_url_with_blob(&data))?;
    let document = utils::browser::get_document()?;
    let a = wrap_js_err(document.create_element("a"))?;

    wrap_js_err(a.set_attribute("href", &url))?;
    wrap_js_err(a.set_attribute("download", file_name))?;
    a.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
    wrap_js_err(Url::revoke_object_url(&url))?;

    Ok(())
}
