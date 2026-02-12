use crate::api::requests::chunk::PostChunkResponse;
use crate::constants::CHUNK_SIZE;
use crate::model::chunk::DecryptedChunk;
use crate::model::encryption::{FileKey, MetadataKey};
use crate::model::node::{DecryptedNode, MetadataV1, NodeMetadata};
use crate::{api, utils};

use crabdrive_common::da;
use crabdrive_common::iv::IV;
use crabdrive_common::payloads::node::request::file::PostCreateFileRequest;
use crabdrive_common::payloads::node::response::file::{
    PostCommitFileResponse, PostCreateFileResponse,
};
use crabdrive_common::storage::{FileRevision, NodeId, RevisionId};

use anyhow::{Context, Result, anyhow};
use chrono::Local;
use tracing::debug_span;
use wasm_bindgen_futures::js_sys::Uint8Array;
use web_sys::File;

/// Create a new file node. Returns `Err` if called unauthenticated.
///
/// Returns the freshly created node.
pub async fn create_file(
    parent: &mut DecryptedNode,
    file_name: String,
    file: File,
) -> Result<DecryptedNode> {
    let _guard = debug_span!("api::createFile").entered();
    let token = utils::auth::get_token()?;

    // The key which is used for encrypting metadata. This will later be stored inside the encrypted
    // metadata of the parent.
    let metadata_encryption_key: MetadataKey = utils::encryption::generate_aes256_key().await?;
    // The key used to encrypt the file chunks. This will be stored in the encrypted metadata of the node.
    let file_encryption_key: FileKey = utils::encryption::generate_aes256_key().await?;

    let file_metadata = NodeMetadata::V1(MetadataV1 {
        name: file_name,
        last_modified: Local::now().naive_local(),
        created: Local::now().naive_local(),
        size: Some(da!(file.size())),
        mime_type: Some(file.type_()),
        file_key: Some(file_encryption_key),
        children_key: vec![],
    });

    let new_node_id = NodeId::random();

    let encrypted_metadata =
        utils::encryption::node::encrypt_metadata(&file_metadata, &metadata_encryption_key)
            .await
            .context("Could not encrypt metadata")
            .inspect_err(|e| tracing::error!("Failed to encrypt metadata: {}", e))?;

    let mut new_parent_metadata = parent.metadata.clone();

    match new_parent_metadata {
        NodeMetadata::V1(ref mut metadata) => metadata
            .children_key
            .push((new_node_id, metadata_encryption_key)),
    }

    let encrypted_parent_metadata =
        utils::encryption::node::encrypt_metadata(&new_parent_metadata, &parent.encryption_key)
            .await
            .context("Could not encrypt metadata")
            .inspect_err(|e| tracing::error!("Failed to encrypt parent metadata: {}", e))?;

    let chunk_count = (file.size() / CHUNK_SIZE).ceil() as i64;

    let file_iv = utils::encryption::random::get_random_iv()
        .inspect_err(|e| tracing::error!("Failed to generate random IV: {}", e))?;

    let request_body = PostCreateFileRequest {
        parent_metadata_version: parent.change_count,
        parent_metadata: encrypted_parent_metadata.clone(),
        node_metadata: encrypted_metadata.clone(),
        file_iv,
        chunk_count,
        node_id: new_node_id,
    };

    let response = api::requests::file::post_create_file(parent.id, request_body, &token)
        .await
        .inspect_err(|e| tracing::error!("Failed to post to create_file: {}", e))
        .map_err(|e| anyhow!("Could not create file: {:?}", e))?;

    match response {
        PostCreateFileResponse::Created(new_file) => {
            parent.metadata = new_parent_metadata;
            parent.change_count += 1;

            let file_revision = new_file.current_revision;

            if file_revision.is_none() {
                tracing::error!("No associated revision found for file.");
                return Err(anyhow!(
                    "The server did not create a file revision when creating the file"
                ));
            }

            let file_revision = file_revision.unwrap();

            // if this fails the server is lying to us
            if file_revision.iv != file_iv {
                tracing::error!("IV Mismatch!");
                return Err(anyhow!("The server is lying to us"));
            }

            //TODO test this
            upload_file(
                file,
                metadata_encryption_key,
                file_encryption_key,
                &file_revision,
                new_node_id,
                &token,
            )
            .await
        }
        PostCreateFileResponse::NotFound => Err(anyhow!(
            "No such node: {}. Check if you have permission to access it",
            parent.id
        )),
        PostCreateFileResponse::BadRequest => Err(anyhow!("Bad request")),
        PostCreateFileResponse::Conflict => Err(anyhow!("Bad request")),
    }
}

async fn upload_file(
    file: File,
    metadata_key: MetadataKey,
    file_key: FileKey,
    revision: &FileRevision,
    node_id: NodeId,
    token: &String,
) -> Result<DecryptedNode> {
    //TODO test this
    let _guard = debug_span!("uploadFile").entered();

    utils::file::load_file_by_chunk(file, |chunk| {
        // this does not clone the actual arraybuffer, just the ref to it
        let chunk = chunk.clone();
        async move {
            encrypt_and_upload_chunk(&chunk, revision.iv, &file_key, node_id, revision.id, token)
                .await
        }
    })
    .await
    .inspect_err(|e| tracing::error!("Failed to split file into chunks: {}", e))?;

    let response = api::requests::file::post_commit_file(node_id, revision.id, token)
        .await
        .inspect_err(|e| tracing::error!("Failed to post to commit_file: {}", e))?;

    match response {
        PostCommitFileResponse::Ok(encrypted_node) => {
            let decrypted_node =
                utils::encryption::node::decrypt_node(encrypted_node, metadata_key)
                    .await
                    .inspect_err(|e| tracing::error!("Failed to decrypt node metadata: {}", e))?;
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
    file_key: &FileKey,
    node_id: NodeId,
    revision_id: RevisionId,
    token: &String,
) -> Result<()> {
    let _guard = debug_span!("encryptAndUploadChunk").entered();

    let encrypted_chunk = utils::encryption::chunk::encrypt_chunk(chunk, file_key, iv_prefix)
        .await
        .inspect_err(|e| tracing::error!("Failed to encrypt chunk: {}", e))?;
    let request_body = Uint8Array::new(&encrypted_chunk.chunk);
    let response =
        api::requests::chunk::post_chunk(node_id, revision_id, chunk.index, request_body, token)
            .await
            .inspect_err(|e| tracing::error!("Failed to post to create_file: {}", e))?;

    //TODO error handling
    match response {
        PostChunkResponse::Created => Ok(()),
        PostChunkResponse::OutOfStorage => Err(anyhow!("You have exceeded your quota")),
        _ => Err(anyhow!(
            "Unexpected error while uploading chunk: {:?}",
            response
        )),
    }
}
