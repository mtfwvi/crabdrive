// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Integer,
        name -> Text,
        file_path -> Text,
        file_size -> Integer,
        mime_type -> Text,
        folder_id -> Nullable<Integer>,
        owner_id -> Integer,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    folders (id) {
        id -> Integer,
        name -> Text,
        parent_id -> Nullable<Integer>,
        owner_id -> Integer,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        password_hash -> Text,
        created_at -> Text,
        updated_at -> Text,
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
