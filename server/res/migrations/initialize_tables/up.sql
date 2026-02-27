CREATE TABLE User (
    id                          TEXT        NOT NULL PRIMARY KEY,
    user_type                   TEXT        NOT NULL CHECK (user_type IN ('ADMIN', 'USER', 'RESTRICTED')),
    username                    TEXT        NOT NULL UNIQUE,
    password_hash               TEXT        NOT NULL,
    storage_limit               INTEGER     NOT NULL,
    created_at                  TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    encryption_uninitialized    INTEGER     NOT NULL DEFAULT 0,
    master_key                  BLOB        NOT NULL,
    private_key                 BLOB        NOT NULL,
    public_key                  BLOB        NOT NULL,
    root_key                    BLOB        NOT NULL,
    -- DEFERRABLE INITIALLY DEFERRED only checks foreign key constraint on transaction end (see https://sqlite.org/foreignkeys.html)
    root_node                   TEXT            NULL REFERENCES Node(id) DEFERRABLE INITIALLY DEFERRED,
    trash_key                   BLOB        NOT NULL,
    trash_node                  TEXT            NULL REFERENCES Node(id) DEFERRABLE INITIALLY DEFERRED
);

CREATE TABLE Node (
    id                          TEXT        NOT NULL PRIMARY KEY,
    parent_id                   TEXT            NULL REFERENCES Node(id),
    owner_id                    TEXT        NOT NULL REFERENCES User(id),
    metadata                    BLOB        NOT NULL,
    deleted_on                  TIMESTAMP       NULL,
    metadata_change_counter     INTEGER     NOT NULL DEFAULT 0,
    current_revision            TEXT            NULL REFERENCES Revision(id),
    node_type                   TEXT        NOT NULL CHECK (node_type IN ('FILE', 'FOLDER', 'LINK'))
);

CREATE INDEX IdxNodeOwner ON Node(owner_id);
CREATE INDEX IdxNodeParent ON Node(parent_id);

CREATE TABLE Revision (
    id                          TEXT        NOT NULL PRIMARY KEY,
    file_id                     TEXT        NOT NULL REFERENCES Node(id),
    upload_started_on           TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    upload_ended_on             TIMESTAMP       NULL,
    iv                          BLOB        NOT NULL,
    chunk_count                 INTEGER     NOT NULL
);

CREATE TABLE Share (
    id                          TEXT        NOT NULL PRIMARY KEY,
    node_id                     TEXT        NOT NULL REFERENCES Node(id),
    shared_by                   TEXT        NOT NULL REFERENCES User(id),
    accepted_by                 TEXT            NULL REFERENCES User(id),
    time_shared                 TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    time_accepted               TIMESTAMP       NULL,
    shared_encryption_key       BLOB            NULL,
    accepted_encryption_key     BLOB            NULL
);
