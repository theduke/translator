CREATE TABLE languages(
  id TEXT PRIMARY KEY,
  code TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  parent_id TEXT REFERENCES languages (id) ON DELETE SET NULL,
  created_at BIGINT NOT NULL,
  created_by TEXT REFERENCES users (id) ON DELETE SET NULL
);
