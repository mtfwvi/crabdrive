CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    storage_limit INTEGER,
    masterkey BINARY NOT NULL REFERENCES encryptionKey(ekey)
    private_key BINARY NOT NULL REFERENCES encryptionKey(ekey)
    public_key BINARY NOT NULL
    root_key BINARY NOT NULL REFERENCES encryptionKey(ekey)
    root_node INTEGER NULL
    trash_key BINARY NOT NULL REFERENCES encryptionKey(ekey)
    trash_node BINARY NOT NULL REFERENCES encryptionKey(ekey)
    
);


CREATE TABLE encryptionKey(
    ekey BINARY PRIMARY KEY NOT NULL,
    IV TEXT NOT NULL
)

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
