
CREATE TABLE users(
    id TEXT NOT NULL PRIMARY KEY,
    role TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME
);

CREATE TABLE tokens(
    id TEXT NOT NULL PRIMARY KEY,
    kind TEXT NOT NULL,
    token TEXT NOT NULL UNIQUE,
    valid_until DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    user_id TEXT NOT NULL REFERENCES users (id)
);

CREATE TABLE languages(
    id TEXT NOT NULL PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    parent_id TEXT REFERENCES languages (id)
);

CREATE TABLE keys(
    id TEXT NOT NULL PRIMARY KEY,
    key TEXT NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    creator_id TEXT NOT NULL REFERENCES users (id)
);

CREATE UNIQUE INDEX keys_unique ON keys (key) WHERE deleted_at IS NULL;

CREATE TABLE translations(
    id TEXT NOT NULL PRIMARY KEY,
    version INTEGER UNSIGNED NOT NULL,
    translation TEXT NOT NULL,
    comment TEXT,
    language_id TEXT NOT NULL REFERENCES languages (id),
    key_id TEXT NOT NULL REFERENCES keys (id),
    creator_id TEXT NOT NULL REFERENCES users (id),
    approver_id TEXT NOT NULL REFERENCES users (id),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    CONSTRAINT translations_unique UNIQUE (language_id, key_id, version)
);

CREATE TABLE translation_requests(
    id TEXT NOT NULL PRIMARY KEY,
    translation TEXT NOT NULL,
    comment TEXT,
    language_id TEXT NOT NULL REFERENCES languages (id),
    key_id TEXT NOT NULL REFERENCES keys (id),
    creator_id TEXT NOT NULL REFERENCES users (id),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
