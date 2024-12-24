CREATE TABLE users (
    id             BYTEA PRIMARY KEY,
    created_at     TIMESTAMPTZ NOT NULL,
    name           VARCHAR NOT NULL,
    last_login_at  TIMESTAMPTZ,
    email          VARCHAR UNIQUE NOT NULL
);

CREATE TABLE user_identities (
    id        BYTEA PRIMARY KEY,
    user_id   BYTEA NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    provider  VARCHAR NOT NULL,
    data      JSONB NOT NULL,

    UNIQUE (user_id, provider)
);

CREATE TABLE user_refresh_tokens (
    id              BYTEA PRIMARY KEY,
    user_id         BYTEA NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    token_hash      BYTEA UNIQUE NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL,
    last_used_at    TIMESTAMPTZ,
    user_agent      VARCHAR,
    remote_address  VARCHAR NOT NULL
);
