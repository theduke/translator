CREATE TABLE translations(
  language TEXT NOT NULL REFERENCES languages (id) ON DELETE CASCADE,
  key TEXT NOT NULL REFERENCES keys (key) ON DELETE CASCADE,
  value TEXT NOT NULL,
  created_at BIGINT NOT NULL,
  updated_at BIGINT NOT NULL,
  created_by TEXT REFERENCES users (username) ON DELETE SET NULL,
  PRIMARY KEY (language, key)
);
