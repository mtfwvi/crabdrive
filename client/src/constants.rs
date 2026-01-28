use crate::model::encryption::EncryptionKey;
use crabdrive_common::da;

pub const EMPTY_KEY: EncryptionKey = [0; 32];
pub const AES_GCM: &str = "AES-GCM";

/// chunk size in bytes when uploading files
pub const CHUNK_SIZE: f64 = da!(16 MiB).as_bytes() as f64;

pub const API_BASE_PATH: &str = "http://localhost:2722";
