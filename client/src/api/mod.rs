use crate::api::requests::chunk::{get_chunk, GetChunkResponse};
use crate::api::requests::file::{post_commit_file, post_create_file};
use crate::api::requests::folder::post_create_folder;
use crate::api::requests::node::{get_node, get_node_children};
use crate::constants::{CHUNK_SIZE, EMPTY_KEY};
use crate::model::chunk::EncryptedChunk;
use crate::model::node::{DecryptedNode, MetadataV1, NodeMetadata};
use crate::utils::encryption::chunk;
use crate::utils::encryption::chunk::decrypt_chunk;
use crate::utils::encryption::node::{decrypt_node, encrypt_metadata};
use crate::utils::encryption::random::get_random_iv;
use crate::utils::file::{combine_chunks, load_file_by_chunk};
use crabdrive_common::payloads::node::request::file::PostCreateFileRequest;
use crabdrive_common::payloads::node::request::folder::PostCreateFolderRequest;
use crabdrive_common::payloads::node::response::file::{
    PostCommitFileResponse, PostCreateFileResponse,
};
use crabdrive_common::payloads::node::response::folder::PostCreateFolderResponse;
use crabdrive_common::payloads::node::response::node::{GetNodeChildrenResponse, GetNodeResponse};
use crabdrive_common::storage::NodeId;
use web_sys::js_sys::Uint8Array;
use web_sys::{Blob, File};

pub mod requests;

pub async fn create_folder(
    parent: &mut DecryptedNode,
    folder_name: String,
) -> Result<DecryptedNode, String> {
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
        parent_metadata_iv: *encrypted_parent_metadata.iv(),
        parent_metadata_version: parent.change_count,
        parent_metadata: encrypted_parent_metadata.metadata().clone(),
        node_metadata_iv: *encrypted_metadata.iv(),
        node_metadata: encrypted_metadata.metadata().clone(),
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

            Ok(decrypted_node)
        }
        PostCreateFolderResponse::NotFound => Err(format!(
            "no such node: {}. Check if you have permission to access it",
            parent.id
        )),
        PostCreateFolderResponse::BadRequest => Err("bad request".to_string()),
        PostCreateFolderResponse::Conflict => Err("Please try again".to_string()),
    }
}

pub async fn create_file(
    parent: &mut DecryptedNode,
    file_name: String,
    file: File,
) -> Result<DecryptedNode, String> {
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

    let chunk_count = (file.size() / CHUNK_SIZE).ceil() as u64;

    let file_iv = get_random_iv();
    let request_body = PostCreateFileRequest {
        parent_metadata_iv: *encrypted_parent_metadata.iv(),
        parent_metadata_version: parent.change_count,
        parent_metadata: encrypted_parent_metadata.metadata().clone(),
        node_metadata_iv: *encrypted_metadata.iv(),
        node_metadata: encrypted_metadata.metadata().clone(),
        file_iv,
        chunk_count,
        node_id: new_node_id,
    };

    let response = post_create_file(parent.id, request_body, &"".to_string()).await;

    if let Err(js_error) = response {
        return Err(format!("could not upload chunks: {:?}", js_error));
    }

    let response = response.unwrap();

    match response {
        PostCreateFileResponse::Created(new_file) => {
            parent.metadata = new_parent_metadata;
            parent.change_count += 1;

            let file_revision = new_file
                .current_revision
                .expect("The server did not create a file revision when creating the file");

            // if this fails the server is lying to us
            assert_eq!(file_revision.iv, file_iv);

            //TODO test this
            let result = load_file_by_chunk(file, |chunk| {
                // this does not clone the actual arraybuffer, just the ref to it
                let chunk = chunk.clone();
                async move {
                    chunk::encrypt_and_upload_chunk(
                        &chunk,
                        file_iv,
                        &file_encryption_key,
                        new_node_id,
                        file_revision.id,
                    )
                    .await
                }
            })
            .await;

            if let Err(js_error) = result {
                return Err(format!("could not upload chunks: {:?}", js_error));
            }

            let response = post_commit_file(new_node_id, file_revision.id, &"".to_string()).await;

            if let Err(ref js_error) = response {
                return Err(format!("could not commit file: {:?}", js_error));
            };

            let response = response.unwrap();
            match response {
                PostCommitFileResponse::Ok(encrypted_node) => {
                    let decrypted_node = decrypt_node(encrypted_node, new_node_encryption_key)
                        .await
                        .unwrap();
                    Ok(decrypted_node)
                }
                PostCommitFileResponse::BadRequest(missing_chunks) => {
                    Err(format!("missing chunks: {:?}", missing_chunks))
                }
                PostCommitFileResponse::NotFound => Err(format!("no such node: {}", new_node_id)),
            }
        }
        PostCreateFileResponse::NotFound => Err(format!(
            "no such node: {}. Check if you have permission to access it",
            parent.id
        )),
        PostCreateFileResponse::BadRequest => Err("bad request".to_string()),
        PostCreateFileResponse::Conflict => Err("Please try again".to_string()),
    }
}

pub async fn get_children(parent: DecryptedNode) -> Result<Vec<DecryptedNode>, String> {
    let response_result = get_node_children(parent.id, &"".to_string()).await;

    if let Err(err) = response_result {
        return Err(format!("Could not query children: {:?}", err));
    }

    let response = response_result.unwrap();
    match response {
        GetNodeChildrenResponse::Ok(children) => {
            let mut decrypted_children = Vec::with_capacity(children.len());

            for child in children {
                let decrypted_child = decrypt_node(child, EMPTY_KEY).await;
                if let Ok(decrypted_child) = decrypted_child {
                    decrypted_children.push(decrypted_child);
                } else {
                    return Err(format!(
                        "could not decrypt node: {:?}",
                        decrypted_child.err().unwrap()
                    ));
                }
            }

            Ok(decrypted_children)
        }
        GetNodeChildrenResponse::NotFound => Err("Could not query children: 404".to_string()),
    }
}

pub async fn download_file(node: DecryptedNode) -> Result<Blob, String> {
    // TODO support chunked downloads in chrom(e/ium)

    let current_revision = node.current_revision;
    if current_revision.is_none() {
        return Err("this node does not have a file associated with".to_string());
    }
    let current_revision = current_revision.unwrap();

    let mut chunks = Vec::with_capacity(current_revision.chunk_count as usize);

    for i in 1..(current_revision.chunk_count) {
        let chunk_result = get_chunk(node.id, current_revision.id, i, &"".to_string()).await;

        if let Err(js_error) = chunk_result {
            return Err(format!("could not download chunk: {:?}", js_error));
        }

        let chunk_response = chunk_result.unwrap();
        match chunk_response {
            GetChunkResponse::Ok(encrypted_chunk_buffer) => {
                let encrypted_chunk = EncryptedChunk {
                    chunk: encrypted_chunk_buffer,
                    index: i,
                    first_block: i == 1,
                    last_block: i == current_revision.chunk_count,
                    iv_prefix: current_revision.iv,
                };
                let decrypted_chunk = decrypt_chunk(&encrypted_chunk, &EMPTY_KEY).await;
                if decrypted_chunk.is_err() {
                    return Err(format!(
                        "chunk decryption failed for chunk {i}: {:?}",
                        decrypted_chunk.err().unwrap()
                    ));
                }
                chunks.push(Uint8Array::new(&decrypted_chunk.unwrap().chunk));
            }
            GetChunkResponse::NotFound => {
                return Err(format!("chunk {i} return 404"));
            }
        }
    }

    Ok(combine_chunks(chunks))
}

pub async fn get_root_node() -> Result<DecryptedNode, String> {
    let get_node_result = get_node(NodeId::nil(), &"".to_string()).await;
    if let Err(js_error) = get_node_result {
        return Err(format!("could not get root node: {:?}", js_error));
    }
    let get_node_response = get_node_result.unwrap();

    match get_node_response {
        GetNodeResponse::Ok(encrypted_node) => {
            if encrypted_node.encrypted_metadata.eq(&[0, 0, 0, 0]) {
                // TODO remove when actually implementing users, as each user should have a root

                // this is the empty node the server created on start => cannot be decrypted

                // metadata will be uploaded to the server during the next update (e.g. adding children)
                let root_node_metadata = NodeMetadata::V1(MetadataV1 {
                    name: "root".to_string(),
                    last_modified: Default::default(),
                    created: Default::default(),
                    size: None,
                    mime_type: None,
                    file_key: None,
                    children_key: vec![],
                });

                let decrypted_node = DecryptedNode {
                    id: encrypted_node.id,
                    change_count: 0,
                    parent_id: encrypted_node.parent_id,
                    owner_id: encrypted_node.owner_id,
                    deleted_on: encrypted_node.deleted_on,
                    node_type: encrypted_node.node_type,
                    current_revision: encrypted_node.current_revision,
                    metadata: root_node_metadata,
                    encryption_key: EMPTY_KEY,
                };

                Ok(decrypted_node)
            } else {
                let decrypted_node_result = decrypt_node(encrypted_node, EMPTY_KEY).await;

                if let Err(js_error) = decrypted_node_result {
                    return Err(format!("could not decrypt node: {:?}", js_error));
                }

                let decrypted_node = decrypted_node_result.unwrap();
                Ok(decrypted_node)
            }
        }
        GetNodeResponse::NotFound => Err("root node returned 404".to_string()),
    }
}
