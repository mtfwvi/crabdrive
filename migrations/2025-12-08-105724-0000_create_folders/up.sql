CREATE TABLE folders (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    parent_id INTEGER NULL,
    owner_id INTEGER NOT NULL,
    FOREIGN KEY (parent_id) REFERENCES folders(id) ON DELETE CASCADE,
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE,
    is_folder INTEGER DEFAULT 0,
    FOREIGN KEY encryptedMetadata(EncryptedMetadata_mdata, EncryptedMetadata_iv) REFERENCES EncryptedMetadata(mdata, iv)

);

CREATE TABLE EncryptedMetadata(
    mdata BLOB NOT NULL,
    iv TEXT NOT NULL REFERENCES encryptionKey(IV)
);

CREATE INDEX idx_folders_owner ON folders(owner_id);
CREATE INDEX idx_folders_parent ON folders(parent_id);
