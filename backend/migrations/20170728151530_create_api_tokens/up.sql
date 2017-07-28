-- Your SQL goes here
CREATE TABLE api_tokens(
  token TEXT PRIMARY KEY,
  created_at BIGINT NOT NULL,
  created_by TEXT references users(username) ON DELETE CASCADE
);