use crate::payloads::node::response::node::NodeInfo;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PostCreateFolderResponse {
    Created(NodeInfo),
    NotFound,
    BadRequest,
    Conflict,
}
