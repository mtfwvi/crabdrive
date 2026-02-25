use crate::constants::APPLICATION_BASE_PATH;
use crate::model::encryption::RawEncryptionKey;
use crate::utils::encryption::{decode_key, encode_key};
use anyhow::Result;
use anyhow::anyhow;
use crabdrive_common::storage::ShareId;
use crabdrive_common::uuid::UUID;

pub fn parse_share_url(url: &str) -> Result<(ShareId, RawEncryptionKey)> {
    // we have to start by splitting at '#' because base64 allows '/'
    let split = url.split_once('#');
    if split.is_none() {
        return Err(anyhow!("could not find encryption key in url"));
    }
    let (url, encryption_key_string_from_url) = split.unwrap();

    let share_id = url
        .split('/')
        .next_back()
        .ok_or(anyhow!("url {url} is not valid"))?;
    let share_id = UUID::parse_string(share_id).ok_or(anyhow!("not a valid uuid"))?;

    let wrapping_encryption_key = decode_key(encryption_key_string_from_url)?;
    Ok((share_id, wrapping_encryption_key))
}

pub fn create_share_url(share_id: &ShareId, wrapped_key: &RawEncryptionKey) -> String {
    let encoded_key = encode_key(wrapped_key);
    let url = format!("{APPLICATION_BASE_PATH}/share/{share_id}#{encoded_key}");
    url
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_parse_share_url() {
        let key = [
            1, 2, 5, 87, 58, 5, 4, 7, 8, 56, 64, 85, 63, 84, 53, 74, 7, 4, 2, 6, 7, 8, 9, 7, 56, 4,
            7, 8, 6, 3, 2, 5,
        ];
        let share_id = ShareId::random();

        let url = create_share_url(&share_id, &key);
        let parsed = parse_share_url(&url).unwrap();

        assert_eq!(parsed.0, share_id);
        assert_eq!(parsed.1, key);
    }

    #[test]
    fn test_parse_url() {
        let url = "http://localhost:2722/share/99202218-bee9-4c9d-b3b7-5bfb6ccc7862#AW5TSC+Dp00T92iZsie4UjByeXg/vQPqTxztE4mo3es=";
        let _parsed = parse_share_url(url).unwrap();
    }
}
