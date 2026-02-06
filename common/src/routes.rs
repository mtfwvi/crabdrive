pub mod node {
    use crate::storage::{ChunkIndex, NodeId, RevisionId};

    pub const ROUTE_BY_ID: &str = "/api/node/{id}";
    /// `/api/node/{id}/`
    pub fn by_id(id: NodeId) -> String {
        ROUTE_BY_ID.replace("{id}", &id.to_string())
    }

    pub const ROUTE_MOVE: &str = "/api/node/{id}/move/";
    /// `/api/node/{id}/move/`
    pub fn move_to(id: NodeId) -> String {
        ROUTE_MOVE.replace("{id}", &id.to_string())
    }

    pub const ROUTE_MOVE_TO_TRASH: &str = "/api/node/{id}/move_to_trash/";
    /// `/api/node/{id}/move_to_trash/`
    pub fn move_to_trash(id: NodeId) -> String {
        ROUTE_MOVE_TO_TRASH.replace("{id}", &id.to_string())
    }

    pub const ROUTE_MOVE_OUT_OF_TRASH: &str = "/api/node/{id}/move_out_of_trash/";
    /// `/api/node/{id}/move_out_of_trash/`
    pub fn move_out_of_trash(id: NodeId) -> String {
        ROUTE_MOVE_OUT_OF_TRASH.replace("{id}", &id.to_string())
    }

    pub mod file {
        use super::*;

        pub const ROUTE_CREATE: &str = "/api/node/{id}/create_file/";
        /// `/api/node/{id}/create_file/`
        pub fn create(id: NodeId) -> String {
            ROUTE_CREATE.replace("{id}", &id.to_string())
        }

        pub const ROUTE_UPDATE: &str = "/api/node/{id}/update_file/";
        /// `/api/node/{id}/update_file/`
        pub fn update(id: NodeId) -> String {
            ROUTE_UPDATE.replace("{id}", &id.to_string())
        }

        pub const ROUTE_COMMIT: &str = "/api/node/{id}/versions/{version_id}/commit/";
        /// `/api/node/{id}/versions/{version_id}/commit/`
        pub fn commit(node_id: NodeId, version_id: RevisionId) -> String {
            ROUTE_COMMIT
                .replace("{id}", &node_id.to_string())
                .replace("{version_id}", &version_id.to_string())
        }
    }

    pub mod folder {
        use super::*;

        pub const ROUTE_CREATE: &str = "/api/node/{id}/create_folder/";
        /// `/api/node/{id}/create_folder/`
        pub fn create(id: NodeId) -> String {
            ROUTE_CREATE.replace("{id}", &id.to_string())
        }
    }

    pub const ROUTE_CHILDREN: &str = "/api/node/{id}/children/";
    /// `/api/node/{id}/children/`
    pub fn children(id: NodeId) -> String {
        ROUTE_CHILDREN.replace("{id}", &id.to_string())
    }

    pub const ROUTE_PATH_BETWEEN: &str = "/api/path_between_nodes/";
    /// `/api/path_between_nodes/`
    ///
    /// Arguments are reserved for future use.
    pub fn path_between_nodes(_from_id: NodeId, _to_id: NodeId) -> String {
        ROUTE_PATH_BETWEEN.to_string()
    }

    pub const ROUTE_VERSIONS: &str = "/api/node/{id}/versions/";
    /// `/api/node/{id}/versions/`
    pub fn versions(id: NodeId) -> String {
        ROUTE_VERSIONS.replace("{id}", &id.to_string())
    }

    pub const ROUTE_CHUNKS: &str = "/api/node/{id}/versions/{version_id}/chunks/{chunk_index}/";
    /// `/api/node/{id}/versions/{version_id}/chunks/{chunk_index}/`
    pub fn chunks(node_id: NodeId, version_id: RevisionId, chunk_index: ChunkIndex) -> String {
        ROUTE_CHUNKS
            .replace("{id}", &node_id.to_string())
            .replace("{version_id}", &version_id.to_string())
            .replace("{chunk_index}", &chunk_index.to_string())
    }
}

pub mod auth {
    pub const ROUTE_LOGIN: &str = "/api/auth/login/";
    /// `/api/auth/login/`
    pub fn login() -> String {
        ROUTE_LOGIN.to_string()
    }

    pub const ROUTE_REGISTER: &str = "/api/auth/register/";
    /// `/api/auth/register/`
    pub fn register() -> String {
        ROUTE_REGISTER.to_string()
    }

    pub const ROUTE_LOGOUT: &str = "/api/auth/logout/";
    pub fn logout() -> String {
        ROUTE_LOGOUT.to_string()
    }
}

// - node_id
// pub const NODE_ROUTE_NODEID: &str = "/api/node/{}";

// - node_id
// pub const MOVE_NODE_ROUTE: &str = "/api/node/{}/move";

// - node_id
// pub const MOVE_NODE_TO_TRASH_ROUTE: &str = "/api/node/{}/move_to_trash";

// - node_id
// pub const MOVE_NODE_OUT_OF_TASH_ROUTE: &str = "/api/node/{}/move_out_of_trash";

// - parent_id
// pub const CREATE_FILE_ROUTE: &str = "/api/node/{}/create_file";

// - node_id
// pub const UPDATE_FILE_ROUTE: &str = "/api/node/{}/update_file";

// - node_id
// - version_id
// pub const COMMIT_FILE_ROUTE: &str = "/api/node/{}/versions/{}/commit";

// - parent_id
// pub const CREATE_FOLDER_ROUTE: &str = "/api/node/{}/create_folder";

// - parent_id
// pub const CHILDREN_ROUTE: &str = "/api/node/{}/children";

// pub const PATH_BETWEEN_NODES_ROUTE: &str = "/api/path_between_nodes";

// - node_id
// pub const NODE_VERSIONS_ROUTE: &str = "/api/node/{}/versions";
// - node_id
// - version_id
// - chunk_index
// pub const CHUNK_ROUTE: &str = "/api/node/{}/versions/{}/chunks/{}";

// pub const LOGIN_ROUTE: &str = "/api/auth/login";

// pub const REGISTER_ROUTE: &str = "/api/auth/register";

// pub const LOGOUT_ROUTE: &str = "/api/auth/logout";

// - user_id
// pub const ADMIN_USER_ROUTE_ID: &str = "/api/admin/user/{}";

// pub const ADMIN_USER_ROUTE: &str = "/api/admin/user";
