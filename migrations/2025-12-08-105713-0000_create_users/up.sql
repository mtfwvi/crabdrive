CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    storage_limit INTEGER,
    masterkey BLOB NOT NULL REFERENCES encryptionKey(ekey),
    private_key BLOB NOT NULL REFERENCES encryptionKey(ekey),
    public_key BLOB NOT NULL,
    root_key BLOB NOT NULL REFERENCES encryptionKey(ekey),
    root_node INTEGER NULL,
    trash_key BLOB NOT NULL REFERENCES encryptionKey(ekey),
    trash_node BLOB NOT NULL REFERENCES encryptionKey(ekey)
);

-- Indices
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);