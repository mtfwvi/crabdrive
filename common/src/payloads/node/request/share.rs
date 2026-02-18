use crate::encryption_key::EncryptionKey;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostShareNodeRequest {
    /// the metadata key encrypted with the key in the url
    pub wrapped_metadata_key: EncryptionKey
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostAcceptShareRequest {
    /// the metadata key encrypted with the users master key
    pub new_wrapped_metadata_key: EncryptionKey
}