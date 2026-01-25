use crabdrive_common::iv::IV;
use crabdrive_common::storage::ChunkIndex;
use wasm_bindgen_futures::js_sys::{ArrayBuffer, Uint8Array};

#[derive(Debug, Clone)]
pub struct DecryptedChunk {
    pub chunk: ArrayBuffer,
    pub index: ChunkIndex,
    pub first_block: bool,
    pub last_block: bool,
}

#[derive(Debug)]
pub struct EncryptedChunk {
    pub chunk: Uint8Array,
    pub index: ChunkIndex,
    pub first_block: bool,
    pub last_block: bool,
    pub iv_prefix: IV,
}
