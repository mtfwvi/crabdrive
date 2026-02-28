use crate::test::utils::TestContext;

use crabdrive_common::payloads::node::{
    response::node::*,
    request::share::*,
    response::share::*,
};
use crabdrive_common::routes;
use crabdrive_common::uuid::UUID;

use axum::http::StatusCode;
use pretty_assertions::assert_eq;

#[tokio::test]
pub async fn test_sharing_node() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let folder = user.generate_random_folder().await;

    let payload = PostShareNodeRequest {
        wrapped_metadata_key: user.keys.master_key.clone(),
    };

    let response = user
        .post(routes::node::share::share(folder.id))
        .json(&payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
pub async fn test_share_flow() {
    let ctx = TestContext::new(2).await;
    let user_a = ctx.get_user(0);
    let user_b = ctx.get_user(1);

    let folder = user_a.generate_random_folder().await;

    // 1. User A shares a node
    let share_payload = PostShareNodeRequest {
        wrapped_metadata_key: user_a.keys.master_key.clone(),
    };
    let response = user_a
        .post(routes::node::share::share(folder.id))
        .json(&share_payload)
        .await;
    assert_eq!(response.status_code(), StatusCode::OK);

    let PostShareNodeResponse::Ok(share_id) = response.json() else {
        panic!("Expected Ok");
    };

    // 2. User B accepts the share
    let accept_payload = PostAcceptShareRequest {
        new_wrapped_metadata_key: user_b.keys.master_key.clone(),
    };
    let accept_res = user_b
        .post(routes::node::share::accept_share(share_id))
        .json(&accept_payload)
        .await;
    assert_eq!(accept_res.status_code(), StatusCode::OK);

    // 3. User B can see the accepted share
    let list_res = user_b.get(routes::node::share::get_accepted_shared()).await;
    assert_eq!(list_res.status_code(), StatusCode::OK);

    let GetAcceptedSharedResponse::Ok(nodes) = list_res.json();

    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].1.id, folder.id);

    let node_res = user_b.get(routes::node::by_id(folder.id)).await;
    assert_eq!(node_res.status_code(), StatusCode::OK);
}

#[tokio::test]
pub async fn test_sharing_root_and_trash_node() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let payload = PostShareNodeRequest {
        wrapped_metadata_key: user.keys.master_key.clone(),
    };

    let response_root = user
        .post(routes::node::share::share(user.get_root()))
        .json(&payload)
        .await;
    assert_eq!(response_root.status_code(), StatusCode::BAD_REQUEST);

    let response_trash = user
        .post(routes::node::share::share(user.get_trash()))
        .json(&payload)
        .await;
    assert_eq!(response_trash.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
pub async fn test_sharing_deleted_node() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let folder = user.generate_random_folder().await;

    // Manually mark the node as deleted in the DB
    let mut node_entity = user.fetch_node_from_db(folder.id).unwrap();
    node_entity.deleted_on = Some(chrono::Utc::now().naive_utc());
    user.state.node_repository.update_node(&node_entity).unwrap();

    let payload = PostShareNodeRequest {
        wrapped_metadata_key: user.keys.master_key.clone(),
    };

    let response = user
        .post(routes::node::share::share(folder.id))
        .json(&payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
pub async fn test_resharing_a_shared_node() {
    let ctx = TestContext::new(2).await;
    let user_a = ctx.get_user(0);
    let user_b = ctx.get_user(1);

    let folder = user_a.generate_random_folder().await;

    let share_payload = PostShareNodeRequest {
        wrapped_metadata_key: user_a.keys.master_key.clone(),
    };
    let share_res = user_a.post(routes::node::share::share(folder.id)).json(&share_payload).await;

    let PostShareNodeResponse::Ok(share_id) = share_res.json() else {
        panic!("Expected Ok");
    };

    let accept_payload = PostAcceptShareRequest {
        new_wrapped_metadata_key: user_b.keys.master_key.clone(),
    };
    user_b.post(routes::node::share::accept_share(share_id)).json(&accept_payload).await;

    // User B tries to share the folder they do not own
    let reshare_res = user_b
        .post(routes::node::share::share(folder.id))
        .json(&share_payload)
        .await;

    assert_eq!(reshare_res.status_code(), StatusCode::BAD_REQUEST);
}

// info about accepted share

#[tokio::test]
pub async fn test_get_accept_share_info() {
    let ctx = TestContext::new(2).await;
    let user_a = ctx.get_user(0);
    let user_b = ctx.get_user(1);

    let folder = user_a.generate_random_folder().await;

    let share_payload = PostShareNodeRequest {
        wrapped_metadata_key: user_a.keys.master_key.clone(),
    };
    let response = user_a.post(routes::node::share::share(folder.id)).json(&share_payload).await;

    let PostShareNodeResponse::Ok(share_id) = response.json() else {
        panic!("Expected Ok");
    };

    let info_res = user_b.get(routes::node::share::get_share_accept_info(share_id)).await;
    assert_eq!(info_res.status_code(), StatusCode::OK);

    let GetAcceptShareInfoResponse::Ok(info) = info_res.json() else {
        panic!("Expected Ok");
    };
    assert_eq!(info.node_id, folder.id);
}

#[tokio::test]
pub async fn test_get_invalid_accept_share_info() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let info_res = user.get(routes::node::share::get_share_accept_info(UUID::random())).await;
    assert_eq!(info_res.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
pub async fn test_accept_share() {
    let ctx = TestContext::new(2).await;
    let user_a = ctx.get_user(0);
    let user_b = ctx.get_user(1);

    let folder = user_a.generate_random_folder().await;

    let share_payload = PostShareNodeRequest {
        wrapped_metadata_key: user_a.keys.master_key.clone(),
    };
    let response = user_a.post(routes::node::share::share(folder.id)).json(&share_payload).await;

    let PostShareNodeResponse::Ok(share_id) = response.json() else {
        panic!("Expected Ok");
    };

    let accept_payload = PostAcceptShareRequest {
        new_wrapped_metadata_key: user_b.keys.master_key.clone(),
    };
    let accept_res = user_b.post(routes::node::share::accept_share(share_id)).json(&accept_payload).await;

    assert_eq!(accept_res.status_code(), StatusCode::OK);
}

#[tokio::test]
pub async fn test_accept_invalid_share() {
    let ctx = TestContext::new(1).await;
    let user = ctx.get_user(0);

    let accept_payload = PostAcceptShareRequest {
        new_wrapped_metadata_key: user.keys.master_key.clone(),
    };

    let accept_res = user.post(routes::node::share::accept_share(UUID::random())).json(&accept_payload).await;
    assert_eq!(accept_res.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
pub async fn test_accept_already_accepted_share() {
    let ctx = TestContext::new(2).await;
    let user_a = ctx.get_user(0);
    let user_b = ctx.get_user(1);

    let folder = user_a.generate_random_folder().await;

    let share_payload = PostShareNodeRequest {
        wrapped_metadata_key: user_a.keys.master_key.clone(),
    };
    let response = user_a.post(routes::node::share::share(folder.id)).json(&share_payload).await;

    let PostShareNodeResponse::Ok(share_id) = response.json() else {
        panic!("Expected Ok");
    };

    let accept_payload = PostAcceptShareRequest {
        new_wrapped_metadata_key: user_b.keys.master_key.clone(),
    };

    // First accept
    user_b.post(routes::node::share::accept_share(share_id)).json(&accept_payload).await;

    // Second accept attempt
    let accept_res_again = user_b.post(routes::node::share::accept_share(share_id)).json(&accept_payload).await;

    // Server returns BAD_REQUEST since it's already accessible
    assert_eq!(accept_res_again.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
pub async fn test_get_accepted_shared_nodes() {
    let ctx = TestContext::new(2).await;
    let user_a = ctx.get_user(0);
    let user_b = ctx.get_user(1);

    // Initial state check
    let empty_res = user_b.get(routes::node::share::get_accepted_shared()).await;
    assert_eq!(empty_res.status_code(), StatusCode::OK);

    let GetAcceptedSharedResponse::Ok(nodes) = empty_res.json();
    assert!(nodes.is_empty());

    // Share a node and accept it
    let folder = user_a.generate_random_folder().await;
    let share_payload = PostShareNodeRequest {
        wrapped_metadata_key: user_a.keys.master_key.clone(),
    };
    let response = user_a.post(routes::node::share::share(folder.id)).json(&share_payload).await;

    let PostShareNodeResponse::Ok(share_id) = response.json() else {
        panic!("Expected Ok");
    };

    let accept_payload = PostAcceptShareRequest {
        new_wrapped_metadata_key: user_b.keys.master_key.clone(),
    };
    user_b.post(routes::node::share::accept_share(share_id)).json(&accept_payload).await;

    // Retrieve again
    let list_res = user_b.get(routes::node::share::get_accepted_shared()).await;
    assert_eq!(list_res.status_code(), StatusCode::OK);

    let GetAcceptedSharedResponse::Ok(accepted_nodes) = list_res.json();

    assert_eq!(accepted_nodes.len(), 1);
    assert_eq!(accepted_nodes[0].1.id, folder.id);
}


#[tokio::test]
pub async fn test_accessible_path_shared() {
    let ctx = TestContext::new(2).await;
    let user_a = ctx.get_user(0);
    let user_b = ctx.get_user(1);

    let parent_folder = user_a.generate_random_folder().await;
    let child_folder = user_a.generate_folder_in(parent_folder.id).await;

    // User A shares the parent folder
    let share_payload = PostShareNodeRequest {
        wrapped_metadata_key: user_a.keys.master_key.clone(),
    };
    let share_res = user_a
        .post(routes::node::share::share(parent_folder.id))
        .json(&share_payload)
        .await;

    let PostShareNodeResponse::Ok(share_id) = share_res.json() else {
        panic!("Expected Ok");
    };

    // User B accepts the share
    let accept_payload = PostAcceptShareRequest {
        new_wrapped_metadata_key: user_b.keys.master_key.clone(),
    };
    user_b.post(routes::node::share::accept_share(share_id))
        .json(&accept_payload)
        .await;

    // User B queries the path for the child folder
    let request = user_b.get(routes::node::accessible_path(child_folder.id)).await;
    request.assert_status_ok();

    let nodes = match request.json::<GetAccessiblePathResponse>() {
        GetAccessiblePathResponse::Ok(encrypted_nodes) => encrypted_nodes,
        GetAccessiblePathResponse::NotFound => panic!("Invalid node!"),
    };

    let node_ids = nodes.iter().map(|f| f.id).collect::<Vec<UUID>>();

    // The path should stop at parent_folder because User B does not have access above it
    assert_eq!(node_ids, vec![parent_folder.id, child_folder.id]);
}
