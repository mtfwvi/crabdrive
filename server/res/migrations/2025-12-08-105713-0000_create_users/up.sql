CREATE TABLE users (
    user_type TEXT NOT NULL,
    id TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    storage_limit INTEGER,
    user_type TEXT,
    created_at TIMESTAMP,
    encryption_uninitialized INTEGER DEFAULT 0,
    masterkey BLOB NOT NULL,
    masterkey_iv BLOB NOT NULL,
    privatekey BLOB NOT NULL,
    privatekey_iv BLOB NOT NULL,
    public_key BLOB NOT NULL,
    rootkey BLOB NOT NULL,
    rootkey_iv BLOB NOT NULL,
    root_node INTEGER NULL,
    trashkey BLOB NOT NULL,
    trashkey_iv BLOB NOT NULL,
    trash_node BLOB NOT NULL 
);

CREATE INDEX idx_users_username ON users(username);

CREATE TABLE revision(
    id TEXT,
    FOREIGN KEY file_id REFERENCES Node(id),
    upload_started_on TIMESTAMP NOT NULL,
    upload_ended_on TIMESTAMP
    iv BLOB
);

CREATE TABLE Node(
    id TEXT PRIMARY KEY NOT NULL,
    parent_id TEXT DEFAULT NULL, 
    owner_id TEXT NOT NULL,
    mdata BLOB NOT NULL,
    iv TEXT NOT NULL REFERENCES encryptionKey(IV)
    deleted_on TIMESTAMP,
    metadata_change_counter INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY current_revision REFERENCES revision(id),
    node_type TEXT
);

-- find out how to show creation time on default when creating a user, file, etc