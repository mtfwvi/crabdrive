use crabdrive_common::da;
use std::time::Duration;

pub const AES_GCM: &str = "AES-GCM";

/// chunk size in bytes when uploading files
pub const CHUNK_SIZE: f64 = da!(16 MiB).as_bytes() as f64;

pub const API_BASE_PATH: &str = "http://localhost:2722";

pub const DEFAULT_TOAST_TIMEOUT: Duration = Duration::from_secs(10);
pub const INFINITE_TOAST_TIMEOUT: Duration = Duration::from_secs(9999);
