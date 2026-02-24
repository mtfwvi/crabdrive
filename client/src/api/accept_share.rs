use crate::api::requests::share::{get_share_info, post_accept_share};
use crate::utils::encryption::auth::get_master_key;
use crate::utils::encryption::{unwrap_key, wrap_key};
use crate::utils::share::parse_share_url;
use anyhow::Result;
use anyhow::anyhow;
use crabdrive_common::payloads::node::request::share::PostAcceptShareRequest;
use crabdrive_common::payloads::node::response::share::{
    GetShareInfoResponse, PostAcceptShareResponse,
};

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
