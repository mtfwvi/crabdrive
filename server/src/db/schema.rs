use libsqlite3_sys::SQLITE_CHANGESET_FOREIGN_KEY;

diesel::table! {
    User (id) {
        user_type -> Text,
        id -> Integer,
        username -> Text,
        password_hash -> Text,
        storage_limit -> Integer,
        created_at -> Text,
        updated_at -> Text,
        encryption_uninitialized -> Integer,
        masterkey -> Blob,
        private_key -> Blob,
        privatekey_iv -> Blob,
        public_key  -> Blob,
        root_key  -> Blob,
        rootkey_iv -> Blob,
        root_node -> Integer,
        trash_key  -> Blob,
        trashkey_iv -> Blob,
        trash_node  -> Blob,
    }
}

diesel::table! {
    Node (id) {
        id -> Integer,
        parent_id -> Nullable<Integer>,
        owner_id -> Text,
        mdata -> Blob,
        iv -> Blob,
        deleted_on -> Text,
        created_at -> Text,
        metadata_change_counter -> Integer,
        current_revision -> Nullable<Integer>,
        node_type -> Text,
    }
}

diesel::table!{
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
