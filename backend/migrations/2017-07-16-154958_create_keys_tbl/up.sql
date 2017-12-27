CREATE TABLE keys(
  id TEXT PRIMARY KEY,
  key TEXT NOT NULL UNIQUE,
  description TEXT,
  created_at BIGINT NOT NULL,
  created_by TEXT REFERENCES users (username) ON DELETE SET NULL
);
