use crate::api::requests::auth::get_self_user_info;
use anyhow::Result;
use crabdrive_common::payloads::auth::response::info::SelfUserInfo;

// this is currently unused but should be used for displaying the quota of the user
#[allow(dead_code)]
pub async fn get_self_info() -> Result<SelfUserInfo> {
    get_self_user_info().await
}
