-- Your SQL goes here
CREATE TABLE api_tokens(
  token TEXT PRIMARY KEY,
  kind TEXT NOT NULL,
  created_at BIGINT NOT NULL,
  expires_at BIGINT,
  created_by TEXT references users(id) ON DELETE CASCADE
);