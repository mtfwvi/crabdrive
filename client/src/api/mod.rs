use crate::api::requests::chunk::{post_chunk, PostChunkResponse};
use crate::api::requests::file::{post_commit_file, post_create_file};
use crate::api::requests::folder::post_create_folder;
use crate::constants::EMPTY_KEY;
use crate::model::encryption::EncryptionKey;
use crate::model::node::{DecryptedNode, MetadataV1, NodeMetadata};
use crate::utils::encryption::chunk::encrypt_chunk;
use crate::utils::encryption::node::{decrypt_node, encrypt_metadata};
use crate::utils::encryption::random::get_random_iv;
use crate::utils::file::{load_file_by_chunk, DecryptedChunk};
use crabdrive_common::iv::IV;
use crabdrive_common::payloads::node::request::file::PostCreateFileRequest;
use crabdrive_common::payloads::node::request::folder::PostCreateFolderRequest;
use crabdrive_common::payloads::node::response::file::{
    PostCommitFileResponse, PostCreateFileResponse,
};
use crabdrive_common::payloads::node::response::folder::PostCreateFolderResponse;
use crabdrive_common::storage::{NodeId, RevisionId};
use wasm_bindgen::JsValue;
use web_sys::js_sys::Uint8Array;
use web_sys::File;

pub mod requests;

pub enum CreateNodeResponse {
    Created(DecryptedNode),
    Failed(String),
}

pub async fn create_folder(parent: &mut DecryptedNode, folder_name: String) -> CreateNodeResponse {
    let folder_metadata = NodeMetadata::V1(MetadataV1 {
        name: folder_name,
        last_modified: Default::default(),
        created: Default::default(),
        size: None,
        mime_type: None,
        file_key: None,
        children_key: vec![],
    });

    //TODO actually generate encryption key
    //let new_encryption_key = get_random_encryption_key();

    let new_node_encryption_key = EMPTY_KEY;
    let new_node_id = NodeId::random();

    let encrypted_metadata = encrypt_metadata(&folder_metadata, &new_node_encryption_key)
        .await
        .unwrap();

    let mut new_parent_metadata = parent.metadata.clone();

    match new_parent_metadata {
        NodeMetadata::V1(ref mut metadata) => metadata
            .children_key
            .push((new_node_id, new_node_encryption_key)),
    }

    let encrypted_parent_metadata = encrypt_metadata(&new_parent_metadata, &parent.encryption_key)
        .await
        .unwrap();

    let request_body = PostCreateFolderRequest {
        parent_metadata_iv: encrypted_parent_metadata.iv,
        parent_metadata_version: parent.change_count,
        parent_metadata: encrypted_parent_metadata.data,
        node_metadata_iv: encrypted_metadata.iv,
        node_metadata: encrypted_metadata.data,
        node_id: new_node_id,
    };

    let response = post_create_folder(parent.id, request_body, &"".to_string())
        .await
        .expect("failed to post create folder");

    match response {
        PostCreateFolderResponse::Created(new_folder) => {
            parent.metadata = new_parent_metadata;
            parent.change_count += 1;

            let decrypted_node = decrypt_node(new_folder, new_node_encryption_key)
                .await
                .unwrap();

            CreateNodeResponse::Created(decrypted_node)
        }
        PostCreateFolderResponse::NotFound => CreateNodeResponse::Failed(format!(
            "no such node: {}. Check if you have permission to access it",
            parent.id
        )),
        PostCreateFolderResponse::BadRequest => {
            CreateNodeResponse::Failed("bad request".to_string())
        }
        PostCreateFolderResponse::Conflict => {
            CreateNodeResponse::Failed("Please try again".to_string())
        }
    }
}

pub async fn create_file(
    parent: &mut DecryptedNode,
    file_name: String,
    file: File,
) -> CreateNodeResponse {
    //TODO actually generate encryption keys
    //let new_encryption_key = get_random_encryption_key();

    let new_node_encryption_key = EMPTY_KEY;

    let file_encryption_key = EMPTY_KEY;

    let file_metadata = NodeMetadata::V1(MetadataV1 {
        name: file_name,
        last_modified: Default::default(),
        created: Default::default(),
        size: None,
        mime_type: None,
        file_key: Some(file_encryption_key),
        children_key: vec![],
    });

    let new_node_id = NodeId::random();

    let encrypted_metadata = encrypt_metadata(&file_metadata, &new_node_encryption_key)
        .await
        .unwrap();

    let mut new_parent_metadata = parent.metadata.clone();

    match new_parent_metadata {
        NodeMetadata::V1(ref mut metadata) => metadata
            .children_key
            .push((new_node_id, new_node_encryption_key)),
    }

    let encrypted_parent_metadata = encrypt_metadata(&new_parent_metadata, &parent.encryption_key)
        .await
        .unwrap();

    let file_iv = IV::new(get_random_iv());
    let request_body = PostCreateFileRequest {
        parent_metadata_iv: encrypted_parent_metadata.iv,
        parent_metadata_version: parent.change_count,
        parent_metadata: encrypted_parent_metadata.data,
        node_metadata_iv: encrypted_metadata.iv,
        node_metadata: encrypted_metadata.data,
        file_iv,
        chunk_count: 0,
        node_id: new_node_id,
    };

    let response = post_create_file(parent.id, request_body, &"".to_string()).await;

    if let Err(error) = response {
        return CreateNodeResponse::Failed(format!("could not upload chunks: {:?}", error));
    }

    let response = response.unwrap();

    match response {
        PostCreateFileResponse::Created(new_file) => {
            parent.metadata = new_parent_metadata;
            parent.change_count += 1;

            //let decrypted_node = decrypt_node(new_file, new_node_encryption_key).await.unwrap();
            //CreateNodeResponse::Created(decrypted_node)

            let file_revision = new_file
                .current_revision
                .expect("The server did not create a file revision when creating the file");

            // if this fails the server is lying to us
            assert_eq!(file_revision.iv, file_iv);


            //TODO test this
            let result = load_file_by_chunk(file, |chunk| {
                //TODO check if clone copies the ref or the object
                let chunk = chunk.clone();
                    async move {
                        encrypt_and_upload_chunk(
                            &chunk,
                            file_iv,
                            &file_encryption_key,
                            new_node_id,
                            file_revision.id,
                        ).await
                    }
            }).await;

            if let Err(error) = result {
                return CreateNodeResponse::Failed(format!("could not upload chunks: {:?}", error));
            }

            let response = post_commit_file(new_node_id, file_revision.id, &"".to_string()).await;

            if let Err(ref error) = response {
                return CreateNodeResponse::Failed(format!("could not commit file: {:?}", error));
            };

            let response = response.unwrap();
            match response {
                PostCommitFileResponse::Ok(encrypted_node) => {
                    let decrypted_node = decrypt_node(encrypted_node, new_node_encryption_key)
                        .await
                        .unwrap();
                    CreateNodeResponse::Created(decrypted_node)
                }
                PostCommitFileResponse::BadRequest(missing_chunks) => {
                    CreateNodeResponse::Failed(format!("missing chunks: {:?}", missing_chunks))
                }
                PostCommitFileResponse::NotFound => {
                    CreateNodeResponse::Failed(format!("no such node: {}", new_node_id))
                }
            }
        }
        PostCreateFileResponse::NotFound => {
            CreateNodeResponse::Failed(format!(
                "no such node: {}. Check if you have permission to access it",
                parent.id
            ))
        }
        PostCreateFileResponse::BadRequest => {
            CreateNodeResponse::Failed("bad request".to_string())
        }
        PostCreateFileResponse::Conflict => {
            CreateNodeResponse::Failed("Please try again".to_string())
        }
    }
}

async fn encrypt_and_upload_chunk(
    chunk: &DecryptedChunk,
    iv_prefix: IV,
    key: &EncryptionKey,
    node_id: NodeId,
    revision_id: RevisionId,
) -> Result<(), JsValue> {
    let encrypted_chunk = encrypt_chunk(chunk, key, iv_prefix)
        .await
        .expect("failed to encrypt chunk");

    let request_body = Uint8Array::new(&encrypted_chunk.chunk);

    let response = post_chunk(
        node_id,
        revision_id,
        chunk.index,
        request_body,
        &"".to_string(),
    )
    .await?;

    //TODO error handling
    match response {
        PostChunkResponse::Created => Ok(()),
        PostChunkResponse::NotFound => {
            panic!("404 when uploading chunk")
        }
        PostChunkResponse::BadRequest => {
            panic!("400 when uploading chunk")
        }
        PostChunkResponse::Conflict => {
            panic!("409 when uploading chunk")
        }
        PostChunkResponse::OutOfStorage => {
            panic!("413 when uploading chunk")
        }
    }
}
