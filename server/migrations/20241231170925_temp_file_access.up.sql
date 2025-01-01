CREATE TABLE vault_file_access_codes (
    id             BYTEA PRIMARY KEY,
    vault_file_id  BYTEA NOT NULL REFERENCES vault_files (id),
    code_hash      BYTEA NOT NULL,
    expires_at     TIMESTAMPTZ NOT NULL
);