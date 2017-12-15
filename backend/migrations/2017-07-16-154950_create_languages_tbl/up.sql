CREATE TABLE languages(
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  parent_id TEXT REFERENCES languages (id) ON DELETE CASCADE,
  created_at BIGINT NOT NULL,
  created_by TEXT REFERENCES users (username) ON DELETE SET NULL
);
