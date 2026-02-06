use crate::api::requests::chunk::{PostChunkResponse, post_chunk};
use crate::api::requests::file::{post_commit_file, post_create_file};
use crate::constants::{CHUNK_SIZE, EMPTY_KEY};
use crate::model::chunk::DecryptedChunk;
use crate::model::encryption::EncryptionKey;
use crate::model::node::{DecryptedNode, MetadataV1, NodeMetadata};
use crate::utils::encryption::chunk;
use crate::utils::encryption::node::{decrypt_node, encrypt_metadata};
use crate::utils::encryption::random::get_random_iv;
use crate::utils::file::load_file_by_chunk;
use anyhow::{Context, Result, anyhow};
use chrono::Local;
use crabdrive_common::data::DataAmount;
use crabdrive_common::data::DataUnit::Byte;
use crabdrive_common::iv::IV;
use crabdrive_common::payloads::node::request::file::PostCreateFileRequest;
use crabdrive_common::payloads::node::response::file::{
    PostCommitFileResponse, PostCreateFileResponse,
};
use crabdrive_common::storage::{FileRevision, NodeId, RevisionId};
use wasm_bindgen_futures::js_sys::Uint8Array;
use web_sys::File;

pub async fn create_file(
    parent: &mut DecryptedNode,
    file_name: String,
    file: File,
) -> Result<DecryptedNode> {
    //TODO actually generate encryption keys
    //let new_encryption_key = get_random_encryption_key();

    let new_node_encryption_key = EMPTY_KEY;

    let file_encryption_key = EMPTY_KEY;

    let file_metadata = NodeMetadata::V1(MetadataV1 {
        name: file_name,
        last_modified: Local::now().naive_local(),
        created: Local::now().naive_local(),
        size: Some(DataAmount::new(file.size(), Byte)),
        mime_type: Some(file.type_()),
        file_key: Some(file_encryption_key),
        children_key: vec![],
    });

    let new_node_id = NodeId::random();

    let encrypted_metadata_result =
        encrypt_metadata(&file_metadata, &new_node_encryption_key).await;

    let encrypted_metadata = encrypted_metadata_result.context("could not encrypt metadata")?;

    let mut new_parent_metadata = parent.metadata.clone();

    match new_parent_metadata {
        NodeMetadata::V1(ref mut metadata) => metadata
            .children_key
            .push((new_node_id, new_node_encryption_key)),
    }

    let encrypted_parent_metadata_result =
        encrypt_metadata(&new_parent_metadata, &parent.encryption_key).await;

    let encrypted_parent_metadata =
        encrypted_parent_metadata_result.context("could not encrypt metadata")?;

    let chunk_count = (file.size() / CHUNK_SIZE).ceil() as i64;

    let file_iv = get_random_iv()?;
    let request_body = PostCreateFileRequest {
        parent_metadata_version: parent.change_count,
        parent_metadata: encrypted_parent_metadata.clone(),
        node_metadata: encrypted_metadata.clone(),
        file_iv,
        chunk_count,
        node_id: new_node_id,
    };

    let response = post_create_file(parent.id, request_body, &"".to_string()).await;

    if let Err(js_error) = response {
        return Err(anyhow!("could not create file: {:?}", js_error));
    }

    let response = response?;

    match response {
        PostCreateFileResponse::Created(new_file) => {
            parent.metadata = new_parent_metadata;
            parent.change_count += 2; // First update the parent metadata, then insert the new node

            let file_revision = new_file.current_revision;

            if file_revision.is_none() {
                return Err(anyhow!(
                    "The server did not create a file revision when creating the file"
                ));
            }

            let file_revision = file_revision.unwrap();

            // if this fails the server is lying to us
            if file_revision.iv != file_iv {
                return Err(anyhow!("The server is lying to us"));
            }

            //TODO test this
            upload_file(
                file,
                file_encryption_key,
                &file_revision,
                new_node_id,
                &"".to_string(),
            )
            .await
        }
        PostCreateFileResponse::NotFound => Err(anyhow!(
            "no such node: {}. Check if you have permission to access it",
            parent.id
        )),
        PostCreateFileResponse::BadRequest => Err(anyhow!("bad request")),
        PostCreateFileResponse::Conflict => Err(anyhow!("bad request")),
    }
}

async fn upload_file(
    file: File,
    key: EncryptionKey,
    revision: &FileRevision,
    node_id: NodeId,
    token: &String,
) -> Result<DecryptedNode> {
    //TODO test this
    load_file_by_chunk(file, |chunk| {
        // this does not clone the actual arraybuffer, just the ref to it
        let chunk = chunk.clone();
        async move {
            // TODO errors are not caught
            encrypt_and_upload_chunk(&chunk, revision.iv, &key, node_id, revision.id, token).await
        }
    })
    .await?;

    let response = post_commit_file(node_id, revision.id, token).await?;

    match response {
        PostCommitFileResponse::Ok(encrypted_node) => {
            let decrypted_node = decrypt_node(encrypted_node, key).await?;
            Ok(decrypted_node)
        }
        PostCommitFileResponse::BadRequest(err) => {
            Err(anyhow!("Server returned bad request: {:?}", err))
        }
        PostCommitFileResponse::NotFound => Err(anyhow!("no such node: {}", node_id)),
    }
}

async fn encrypt_and_upload_chunk(
    chunk: &DecryptedChunk,
    iv_prefix: IV,
    key: &EncryptionKey,
    node_id: NodeId,
    revision_id: RevisionId,
    token: &String,
) -> Result<()> {
    let encrypted_chunk = chunk::encrypt_chunk(chunk, key, iv_prefix).await?;

    let request_body = Uint8Array::new(&encrypted_chunk.chunk);

    let response = post_chunk(node_id, revision_id, chunk.index, request_body, token).await?;

    //TODO error handling
    match response {
        PostChunkResponse::Created => Ok(()),
        _ => Err(anyhow!("unexpected response on post chunk: {:?}", response)),
    }
}
