CREATE TABLE keys(
  key TEXT PRIMARY KEY,
  description TEXT,
  created_at BIGINT NOT NULL,
  created_by TEXT REFERENCES users (username) ON DELETE SET NULL
);
