CREATE TABLE files (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    mime_type TEXT NOT NULL,
    folder_id INTEGER,
    owner_id INTEGER NOT NULL,
    FOREIGN KEY (folder_id) REFERENCES folders(id) ON DELETE CASCADE,
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_files_owner ON files(owner_id);
CREATE INDEX idx_files_folder ON files(folder_id);
