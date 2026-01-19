use crabdrive_common::storage::NodeId;

pub type EncryptionKey = [u8; 32];
pub type ChildKey = (NodeId, EncryptionKey);
