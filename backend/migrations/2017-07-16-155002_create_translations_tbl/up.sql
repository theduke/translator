CREATE TABLE translations(
  id TEXT PRIMARY KEY,
  language_id TEXT NOT NULL REFERENCES languages (id) ON DELETE CASCADE,
  key_id TEXT NOT NULL REFERENCES keys (id) ON DELETE CASCADE,
  version INT NOT NULL,
  value TEXT NOT NULL,
  created_at BIGINT NOT NULL,
  updated_at BIGINT NOT NULL,
  created_by TEXT REFERENCES users (id) ON DELETE SET NULL,
  UNIQUE (language_id, key_id)
);
