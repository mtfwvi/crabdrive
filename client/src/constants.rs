use crate::model::encryption::EncryptionKey;

pub const EMPTY_KEY: EncryptionKey = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
pub const AES_GCM: &str = "AES-GCM";
