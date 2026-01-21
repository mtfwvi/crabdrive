use serde::{Deserialize, Serialize};
use crabdrive_common::iv::IV;
use crabdrive_common::storage::NodeId;

pub type EncryptionKey = [u8; 32];
pub type ChildKey = (NodeId, EncryptionKey);

pub type DecryptedMetadata = Vec<u8>;

//TODO use struct from server
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptedMetadata {
    pub data: Vec<u8>,
    pub iv: IV,
}