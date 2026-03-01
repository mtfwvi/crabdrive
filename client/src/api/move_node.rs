use crate::api::requests::node::{
    post_move_node, post_move_node_out_of_trash, post_move_node_to_trash,
};
use crate::api::{get_accessible_path, get_trash_node};
use crate::model::node::{DecryptedNode, NodeMetadata};
use crate::utils::encryption::node::encrypt_metadata;
use anyhow::Result;
use anyhow::anyhow;
use crabdrive_common::payloads::node::request::node::MoveNodeData;
use crabdrive_common::payloads::node::response::node::{
    PostMoveNodeOutOfTrashResponse, PostMoveNodeResponse, PostMoveNodeToTrashResponse,
};

async fn create_move_node_request(
    from: &mut DecryptedNode,
    to: &mut DecryptedNode,
    node: &DecryptedNode,
) -> Result<MoveNodeData> {
    let NodeMetadata::V1(ref mut from_metadata) = from.metadata;
    let NodeMetadata::V1(ref mut to_metadata) = to.metadata;

    if !node.parent_id.eq(&Some(from.id)) {
        return Err(anyhow!("the parameters of move node are not correct"));
    }

    let encryption_key = *from_metadata
        .children_key
        .iter()
        .find(|(node_id, key)| node.id.eq(node_id) && node.encryption_key.eq(key))
        .ok_or_else(|| anyhow!("the parent of the node to be moved does not contain its key"))?;

    from_metadata.children_key.retain(|x| !encryption_key.eq(x));

    to_metadata.children_key.push(encryption_key);

    let encrypted_from_metadata = encrypt_metadata(&from.metadata, &from.encryption_key).await?;
    let encrypted_to_metadata = encrypt_metadata(&to.metadata, &to.encryption_key).await?;

    let post_move_node_request = MoveNodeData {
        from_node_change_counter: from.change_count,
        from_node_metadata: encrypted_from_metadata,
        to_node_change_counter: to.change_count,
        to_node_metadata: encrypted_to_metadata,
        to_node_id: to.id,
    };

    Ok(post_move_node_request)
}

pub async fn move_node(
    node: DecryptedNode,
    mut from: DecryptedNode,
    mut to: DecryptedNode,
) -> Result<()> {
    let move_node_data = create_move_node_request(&mut from, &mut to, &node).await?;

    let response = post_move_node(node.id, move_node_data).await?;

    match response {
        PostMoveNodeResponse::Ok => Ok(()),
        PostMoveNodeResponse::NotFound => Err(anyhow!(
            "one of the nodes referenced during the move operation could not be found"
        )),
        PostMoveNodeResponse::Conflict => Err(anyhow!("refresh the page and try again")),
    }
}

pub async fn move_node_to_trash(node: DecryptedNode) -> Result<()> {
    let mut trash_node = get_trash_node().await?;

    let mut path_to_node = get_accessible_path(node.id).await?;
    if path_to_node.len() < 2 {
        return Err(anyhow!(
            "you do not have access to the parent of the node you are trying to move"
        ));
    }
    path_to_node.reverse();

    let parent = &mut path_to_node[1];

    let move_node_data = create_move_node_request(parent, &mut trash_node, &node).await?;

    let response = post_move_node_to_trash(node.id, move_node_data).await?;
    match response {
        PostMoveNodeToTrashResponse::Ok => Ok(()),
        PostMoveNodeToTrashResponse::NotFound => Err(anyhow!(
            "one of the nodes referenced during the move operation could not be found"
        )),
        PostMoveNodeToTrashResponse::Conflict => Err(anyhow!("refresh the page and try again")),
    }
}

pub async fn move_node_out_of_trash(node: DecryptedNode, mut to: DecryptedNode) -> Result<()> {
    let mut trash_node = get_trash_node().await?;

    let move_node_data = create_move_node_request(&mut trash_node, &mut to, &node).await?;

    let response = post_move_node_out_of_trash(node.id, move_node_data).await?;
    match response {
        PostMoveNodeOutOfTrashResponse::Ok => Ok(()),
        PostMoveNodeOutOfTrashResponse::NotFound => Err(anyhow!(
            "one of the nodes referenced during the move operation could not be found"
        )),
        PostMoveNodeOutOfTrashResponse::Conflict => Err(anyhow!("refresh the page and try again")),
    }
}
