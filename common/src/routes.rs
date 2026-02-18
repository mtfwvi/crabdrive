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

    pub const ROUTE_PURGE_TREE: &str = "/api/node/{id}/purge_tree/";
    pub fn purge_tree(id: NodeId) -> String {
        ROUTE_PURGE_TREE.replace("{id}", &id.to_string())
    }

    pub const ROUTE_EMPTY_TRASH: &str = "/api/trash/empty/";
    pub fn empty_trash() -> String {
        ROUTE_EMPTY_TRASH.to_string()
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
    pub fn path_between_nodes(from_id: NodeId, to_id: NodeId) -> String {
        format!("{}?from_id={}&to_id={}", ROUTE_PATH_BETWEEN, from_id, to_id)
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
    /// `/api/auth/logout/`
    pub fn logout() -> String {
        ROUTE_LOGOUT.to_string()
    }

    pub const ROUTE_INFO: &str = "/api/auth/info/";
    /// `/api/auth/info/`
    pub fn info() -> String {
        ROUTE_INFO.to_string()
    }
}

pub mod admin {
    use crate::user::UserId;

    pub const ROUTE_USER_BY_ID: &str = "/api/admin/user/{id}/";
    /// `/api/admin/user/{id}/`
    pub fn user_by_id(id: UserId) -> String {
        ROUTE_USER_BY_ID.replace("{id}", &id.to_string())
    }

    pub const ROUTE_USER: &str = "/api/admin/user/";
    /// `/api/admin/user/{id}/`
    pub fn user() -> String {
        ROUTE_USER.to_string()
    }
}
