CREATE TABLE shares (
    id INTEGER PRIMARY KEY NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id INTEGER NOT NULL,
    shared_with_user_id INTEGER,
    share_token TEXT UNIQUE,
    permission TEXT NOT NULL,
    created_at TEXT NOT NULL,
    expires_at TEXT,
    FOREIGN KEY (shared_with_user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_shares_resource ON shares(resource_type, resource_id);
CREATE INDEX idx_shares_token ON shares(share_token);
CREATE INDEX idx_shares_user ON shares(shared_with_user_id);
