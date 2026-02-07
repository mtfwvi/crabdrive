/// - node_id
pub const NODE_ROUTE_NODEID: &str = "/api/node/{}";

/// - node_id
pub const MOVE_NODE_ROUTE: &str = "/api/node/{}/move";

/// - node_id
pub const MOVE_NODE_TO_TRASH_ROUTE: &str = "/api/node/{}/move_to_trash";

/// - node_id
pub const MOVE_NODE_OUT_OF_TASH_ROUTE: &str = "/api/node/{}/move_out_of_trash";

/// - parent_id
pub const CREATE_FILE_ROUTE: &str = "/api/node/{}/create_file";

/// - node_id
pub const UPDATE_FILE_ROUTE: &str = "/api/node/{}/update_file";

/// - node_id
/// - version_id
pub const COMMIT_FILE_ROUTE: &str = "/api/node/{}/versions/{}/commit";

/// - parent_id
pub const CREATE_FOLDER_ROUTE: &str = "/api/node/{}/create_folder";

/// - parent_id
pub const CHILDREN_ROUTE: &str = "/api/node/{}/children";

pub const PATH_BETWEEN_NODES_ROUTE: &str = "/api/path_between_nodes";

/// - node_id
pub const NODE_VERSIONS_ROUTE: &str = "/api/node/{}/versions";
/// - node_id
/// - version_id
/// - chunk_index
pub const CHUNK_ROUTE: &str = "/api/node/{}/versions/{}/chunks/{}";

pub const LOGIN_ROUTE: &str = "/api/auth/login";

pub const REGISTER_ROUTE: &str = "/api/auth/register";

pub const LOGOUT_ROUTE: &str = "/api/auth/logout";

pub const USER_INFO_ROUTE: &str = "/api/auth/info";

/// - user_id
pub const ADMIN_USER_ROUTE_ID: &str = "/api/admin/user/{}";

pub const ADMIN_USER_ROUTE: &str = "/api/admin/user";
