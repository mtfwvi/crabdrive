diesel::table! {
    #[allow(non_snake_case)]
    User (id) {
        user_type -> Text,
        id -> Integer,
        username -> Text,
        password_hash -> Text,
        storage_limit -> Integer,
        created_at -> Text,
        updated_at -> Text,
        encryption_uninitialized -> Integer,
        master_key -> Blob,
        private_key -> Blob,
        private_key_iv -> Blob,
        public_key  -> Blob,
        root_key  -> Blob,
        root_key_iv -> Blob,
        root_node -> Integer,
        trash_key  -> Blob,
        trash_key_iv -> Blob,
        trash_node  -> Blob,
    }
}

diesel::table! {
    #[allow(non_snake_case)]
    Node (id) {
        id -> Integer,
        parent_id -> Nullable<Integer>,
        owner_id -> Text,
        metadata -> Blob,
        iv -> Blob,
        deleted_on -> Text,
        created_at -> Text,
        metadata_change_counter -> Integer,
        current_revision -> Nullable<Integer>,
        node_type -> Text,
    }
}

diesel::table! {
    #[allow(non_snake_case)]
    Revision(id) {
        id -> Text,
        fid -> Text,
        upload_started_on -> Text,
        upload_ended_on -> Text,
        iv -> Blob,
    }
}

//still need to write foreign keys

diesel::allow_tables_to_appear_in_same_query!(Revision, Node, User,);
