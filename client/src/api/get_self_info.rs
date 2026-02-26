use crate::api::requests::auth::get_self_user_info;
use anyhow::Result;
use crabdrive_common::payloads::auth::response::info::SelfUserInfo;

pub async fn get_self_info() -> Result<SelfUserInfo> {
    get_self_user_info().await
}
