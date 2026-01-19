use crate::model::encryption::EncryptionKey;
use crate::model::node::DecryptedNode;
use crate::utils::file::ChunkInfo;
use crabdrive_common::iv::IV;
use crabdrive_common::storage::EncryptedNode;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::js_sys::ArrayBuffer;
use web_sys::js_sys::Uint8Array;

pub async fn decrypt_node(
    node: EncryptedNode,
    key: EncryptionKey,
) -> Result<DecryptedNode, JsValue> {
    //TODO do actual decryption
    let decrypted_metadata = serde_json::from_slice(node.encrypted_metadata.as_slice()).unwrap();

    let decrypted_node = DecryptedNode {
        id: node.id,
        change_count: node.change_count,
        parent_id: node.parent_id,
        owner_id: node.owner_id,
        deleted_on: node.deleted_on,
        node_type: node.node_type,
        current_revision: node.current_revision,
        metadata: decrypted_metadata,
    };

    Ok(decrypted_node)
}

pub async fn encrypt_node(
    node: DecryptedNode,
    key: EncryptionKey,
) -> Result<EncryptedNode, JsValue> {
    //TODO do actual encryption

    let encrypted_metadata = serde_json::to_vec(&node.metadata).unwrap();

    let encrypted_node = EncryptedNode {
        id: node.id,
        change_count: node.change_count,
        parent_id: node.parent_id,
        owner_id: node.owner_id,
        deleted_on: node.deleted_on,
        node_type: node.node_type,
        current_revision: node.current_revision,
        encrypted_metadata,
        metadata_iv: IV::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    };

    Ok(encrypted_node)
}

pub async fn decrypt_chunk(chunk: ChunkInfo, key: EncryptionKey) -> Result<ArrayBuffer, JsValue> {
    Ok(chunk.chunk)
}

pub async fn encrypt_chunk(chunk: ChunkInfo, key: EncryptionKey) -> Result<Uint8Array, JsValue> {
    Ok(Uint8Array::new(&chunk.chunk))
}
