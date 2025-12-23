diesel::table! {
    User (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        password_hash -> Text,
        created_at -> Text,
        updated_at -> Text,
        user_type -> Text,
        storage_limit -> Integer,
        masterkey -> Blob,
        private_key -> Blob,
        public_key  -> Blob,
        root_key  -> Blob,
        root_node -> Integer,
        trash_key  -> Blob,
        trash_node  -> Blob,
    }
}

diesel::joinable!(files -> folders (folder_id));
diesel::joinable!(files -> users (owner_id));
diesel::joinable!(folders -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(files, folders, users,);
