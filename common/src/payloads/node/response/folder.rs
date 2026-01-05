use serde::{Deserialize, Serialize};
use crate::payloads::node::response::node::NodeInfo;

#[derive(Serialize, Deserialize, Debug)]
pub enum PostCreateFolderResponse {
    Created(NodeInfo), // TODO change response code
    NotFound,
    BadRequest,
    Conflict,
}