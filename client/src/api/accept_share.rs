use crate::api::requests::share::{get_share_info, post_accept_share};
use crate::model::encryption::RawEncryptionKey;
use crate::utils::encryption::auth::get_master_key;
use crate::utils::encryption::{decode_key, unwrap_key, wrap_key};
use anyhow::Result;
use anyhow::anyhow;
use crabdrive_common::payloads::node::request::share::PostAcceptShareRequest;
use crabdrive_common::payloads::node::response::share::{
    GetShareInfoResponse, PostAcceptShareResponse,
};
use crabdrive_common::storage::NodeId;
use crabdrive_common::uuid::UUID;
use regex::Regex;

//TODO test this + create share url
fn parse_share_url(url: &str) -> Result<(NodeId, RawEncryptionKey)> {
    // regex stolen from here: https://stackoverflow.com/a/8798297
    let re = Regex::new(r"([^/]+)/?$")?;
    let Some(caps) = re.captures(url) else {
        return Err(anyhow!("could not parse URL"));
    };
    let url_end = &caps[1];

    let split = url_end.split_once("#");
    if split.is_none() {
        return Err(anyhow!("could not find encryption key in url"));
    }
    let (share_id, encryption_key_string_from_url) = split.unwrap();

    let share_id = UUID::parse_string(share_id);

    let Some(share_id) = share_id else {
        return Err(anyhow!("could not parse share id"));
    };
    let wrapping_encryption_key = decode_key(encryption_key_string_from_url)?;

    Ok((share_id, wrapping_encryption_key))
}

pub async fn accept_share(url: &str) -> Result<()> {
    let (share_id, wrapping_encryption_key) = parse_share_url(url)?;

    let share_info_response = get_share_info(share_id).await?;

    let share_info = match share_info_response {
        GetShareInfoResponse::Ok(share_info) => share_info,
        GetShareInfoResponse::NotFound => return Err(anyhow!("share not found")),
    };

    // unwrap the key that was encrypted with the key in the url
    let unwrapped_key =
        unwrap_key(&share_info.wrapped_metadata_key, &wrapping_encryption_key).await?;

    let master_key = get_master_key()?;

    // wrap the key with our own key to be able to decrypt it later
    let new_wrapped_key = wrap_key(&unwrapped_key, &master_key).await?;

    let accept_share_body = PostAcceptShareRequest {
        new_wrapped_metadata_key: new_wrapped_key,
    };
    let accept_share_response = post_accept_share(share_id, accept_share_body).await?;
    match accept_share_response {
        PostAcceptShareResponse::Ok => Ok(()),
        PostAcceptShareResponse::NotFound => {
            Err(anyhow!("Server returned NotFound when accepting node"))
        }
    }
}
