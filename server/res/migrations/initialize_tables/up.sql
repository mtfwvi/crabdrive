CREATE TABLE User (
    user_type TEXT NOT NULL,
    id TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    storage_limit INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    encryption_uninitialized INTEGER NOT NULL DEFAULT 0,
    master_key BLOB NOT NULL,
    private_key BLOB NOT NULL,
    public_key BLOB NOT NULL,
    root_key BLOB NOT NULL,
    root_node TEXT NULL,
    trash_key BLOB NOT NULL,
    trash_node TEXT NULL 
);

CREATE INDEX idx_users_username ON User(username);

CREATE TABLE Node(
    id TEXT PRIMARY KEY NOT NULL,
    parent_id TEXT DEFAULT NULL, 
    owner_id TEXT NOT NULL,
    metadata BLOB NOT NULL,
    deleted_on TIMESTAMP NULL,
    metadata_change_counter INTEGER NOT NULL DEFAULT 0,
    current_revision TEXT NULL,
    node_type TEXT NOT NULL,
    FOREIGN KEY (owner_id) REFERENCES User(id),
    FOREIGN KEY (parent_id) REFERENCES Node(id),
    FOREIGN KEY (current_revision) REFERENCES Revision(id)
);

CREATE TABLE Revision(
    id TEXT PRIMARY KEY NOT NULL,
    file_id TEXT NOT NULL,
    upload_started_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    upload_ended_on TIMESTAMP,
    iv BLOB NOT NULL,
    FOREIGN KEY (file_id) REFERENCES Node(id)
);