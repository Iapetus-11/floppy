CREATE TABLE vaults (
    id        BYTEA PRIMARY KEY,
    name      VARCHAR NOT NULL,
    provider  VARCHAR NOT NULL,
    data      JSONB NOT NULL
);

CREATE TABLE user_vault_links (
    user_id   BYTEA NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    vault_id  BYTEA NOT NULL REFERENCES vaults (id) ON DELETE CASCADE,
    is_admin  BOOLEAN NOT NULL DEFAULT FALSE,

    PRIMARY KEY (user_id, vault_id)
);

CREATE TABLE vault_files (
    id          BYTEA PRIMARY KEY,
    vault_id    BYTEA NOT NULL REFERENCES vaults (id) ON DELETE CASCADE,
    path_id     VARCHAR NOT NULL,
    name        VARCHAR NOT NULL,
    file_type   VARCHAR NOT NULL,
    parent_id   BYTEA NULL REFERENCES vault_files (id) ON DELETE CASCADE,
    created_at  TIMESTAMPTZ NULL,
    size        BIGINT NULL,

    UNIQUE (vault_id, path_id)
);