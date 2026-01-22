use crate::model::encryption::EncryptionKey;

pub const EMPTY_KEY: EncryptionKey = [0; 32];
pub const AES_GCM: &str = "AES-GCM";

pub const CHUNK_SIZE: f64 = 1024.0 * 1024.0 * 16.0;
