diesel::table! {
    #[allow(non_snake_case)]
    User (id) {
        user_type -> Text,
        id -> Text,
        created_at -> Timestamp,
        username -> Text,
        password_hash -> Text,
        storage_limit -> BigInt,
        storage_used -> BigInt,
        encryption_uninitialized -> Bool,
        master_key -> Binary,
        private_key -> Binary,
        public_key  -> Binary,
        root_key  -> Binary,
        root_node -> Nullable<Text>,
        trash_key  -> Binary,
        trash_node  -> Nullable<Text>,
    }
}

diesel::table! {
    #[allow(non_snake_case)]
    Node (id) {
        id -> Text,
        parent_id -> Nullable<Text>,
        owner_id -> Text,
        metadata -> Binary,
        deleted_on -> Nullable<Timestamp>,
        metadata_change_counter -> BigInt,
        current_revision -> Nullable<Text>,
        node_type -> Text,
    }
}

diesel::table! {
    #[allow(non_snake_case)]
    Revision(id) {
        id -> Text,
        file_id -> Text,
        upload_started_on -> Timestamp,
        upload_ended_on -> Nullable<Timestamp>,
        iv -> Binary,
        chunk_count -> BigInt,
    }
}

diesel::table! {
    #[allow(non_snake_case)]
    Share(id) {
        id -> Text,
        node_id -> Text,
        shared_by -> Text,
        accepted_by -> Nullable<Text>,
        time_shared -> Timestamp,
        time_accepted -> Nullable<Timestamp>,
        shared_encryption_key -> Nullable<Binary>,
        accepted_encryption_key -> Nullable<Binary>
    }
}

diesel::allow_tables_to_appear_in_same_query!(Revision, Node, User,);
