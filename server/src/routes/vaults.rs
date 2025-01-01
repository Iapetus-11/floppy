use std::{path::PathBuf, time::Duration};

use crate::{
    models::vaults::{Vault, VaultFile},
    utils::{
        response_errors::ForbiddenError, security::random_string, user_security::AuthenticatedUser,
        xid::Xid,
    },
};
use chrono::Utc;
use poem::{
    error::NotFoundError,
    handler,
    web::{Data, Json, Path, Query},
    Body,
};
use serde::Deserialize;
use serde_json::json;
use sha3::{Digest, Sha3_384};
use tokio::fs::File;

#[handler]
pub async fn list_vaults(
    db: Data<&sqlx::Pool<sqlx::Postgres>>,
    user: AuthenticatedUser,
) -> poem::Result<Json<Vec<Vault>>> {
    let vaults = sqlx::query_as!(
        Vault,
        "SELECT vaults.id, vaults.name, vaults.provider, vaults.data FROM vaults LEFT JOIN user_vault_links ON vaults.id = user_vault_links.vault_id WHERE user_vault_links.user_id = $1",
        user.id.as_bytes(),
    ).fetch_all(db.0)
    .await
    .unwrap();

    Ok(Json(vaults))
}

#[derive(Deserialize)]
struct ListVaultFilesQuery {
    after: Option<Xid>,
    parent_id: Option<Xid>,
    search: Option<String>,
}

#[handler]
pub async fn list_vault_files(
    db: Data<&sqlx::Pool<sqlx::Postgres>>,
    user: AuthenticatedUser,
    Path((vault_id,)): Path<(Xid,)>,
    query: Query<ListVaultFilesQuery>,
) -> poem::Result<Json<Vec<VaultFile>>> {
    let query_after_id = match &query.after {
        None => b"\0\0\0\0\0\0\0\0\0\0\0\0",
        Some(xid) => xid.as_bytes(),
    };

    let vault = sqlx::query_as!(
        Vault,
        "SELECT id, name, provider, data FROM vaults WHERE EXISTS(SELECT FROM user_vault_links WHERE user_vault_links.user_id = $1 AND user_vault_links.vault_id = vaults.id) AND id = $2",
        user.id.as_bytes(), vault_id.as_bytes(),
    )
    .fetch_optional(db.0)
    .await
    .unwrap();

    if vault.is_none() {
        return Err(NotFoundError.into());
    }

    let search = query.search.clone().unwrap_or("".to_string());
    let search = match search.as_str().trim() {
        "" => None,
        str => Some(str),
    };

    let files = sqlx::query_as!(
        VaultFile,
        "SELECT id, vault_id, path_id, name, file_type, parent_id, created_at, size FROM vault_files WHERE vault_id = $1 AND id > $2 AND (($3::BYTEA IS NULL AND parent_id IS NULL) OR parent_id = $3::BYTEA) AND ($4::TEXT IS NULL OR POSITION(LOWER($4::TEXT) IN LOWER(name)) > 0)",
        vault_id.as_bytes(),
        query_after_id,
        query.parent_id.map(|p| p.as_bytes().to_vec()),
        search,
    ).fetch_all(db.0)
    .await
    .unwrap();

    Ok(Json(files))
}

#[derive(Deserialize)]
struct DownloadVaultFileQuery {
    code: Option<String>,
}

#[handler]
pub async fn download_vault_file(
    db: Data<&sqlx::Pool<sqlx::Postgres>>,
    user: Option<AuthenticatedUser>,
    query: Query<DownloadVaultFileQuery>,
    Path((vault_id, file_id)): Path<(Xid, Xid)>,
) -> poem::Result<Body> {
    // Assumes we're dealing with local folder vault

    let vault_file_path_id = match (user, &query.code) {
        (None, None) => Err(ForbiddenError),
        (Some(user), _) => {
            let record = sqlx::query!(
                "SELECT path_id FROM vault_files LEFT JOIN user_vault_links ON user_vault_links.vault_id = vault_files.vault_id WHERE user_vault_links.user_id = $1 AND vault_files.id = $2 AND vault_files.vault_id = $3 AND vault_files.file_type = 'file'",
                user.id.as_bytes(), file_id.as_bytes(), vault_id.as_bytes(),
            ).fetch_optional(db.0).await.unwrap();

            Ok(record.map(|r| r.path_id))
        }
        (None, Some(code)) => {
            let vault_file_code_hash = {
                let mut hasher = Sha3_384::new();
                hasher.update(code.as_bytes());
                hasher.finalize().to_vec()
            };

            // AND vault_file_access_codes.expires_at < NOW()

            let record = sqlx::query!(
                "SELECT path_id FROM vault_files \
                    LEFT JOIN vault_file_access_codes \
                    ON vault_file_access_codes.vault_file_id = vault_files.id \
                WHERE \
                    vault_file_access_codes.code_hash = $1 \
                    AND vault_files.id = $2 AND vault_files.vault_id = $3 AND vault_files.file_type = 'file'",
                vault_file_code_hash, file_id.as_bytes(), vault_id.as_bytes(),
            ).fetch_optional(db.0).await.unwrap();

            Ok(record.map(|r| r.path_id))
        }
    }?;

    if vault_file_path_id.is_none() {
        return Err(NotFoundError.into());
    }
    let vault_file_path_id = vault_file_path_id.unwrap();

    let vault_file_path = PathBuf::from(vault_file_path_id);

    let file_stream = File::open(vault_file_path).await.unwrap();

    Ok(Body::from_async_read(file_stream))
}

#[handler]
pub async fn get_vault_file_access_code(
    db: Data<&sqlx::Pool<sqlx::Postgres>>,
    user: AuthenticatedUser,
    Path((vault_id, file_id)): Path<(Xid, Xid)>,
) -> poem::Result<Json<serde_json::Value>> {
    let vault_file_id = sqlx::query!(
        "SELECT id FROM vault_files LEFT JOIN user_vault_links ON user_vault_links.vault_id = vault_files.vault_id WHERE user_vault_links.user_id = $1 AND vault_files.id = $2 AND vault_files.vault_id = $3 AND vault_files.file_type = 'file'",
        user.id.as_bytes(), file_id.as_bytes(), vault_id.as_bytes(),
    ).fetch_optional(db.0).await.unwrap();

    if vault_file_id.is_none() {
        return Err(NotFoundError.into());
    }
    let vault_file_id = vault_file_id.unwrap().id;

    let vault_file_code = random_string(64);
    let vault_file_code_hash = {
        let mut hasher = Sha3_384::new();
        hasher.update(vault_file_code.as_bytes());
        hasher.finalize().to_vec()
    };

    let expires_at = Utc::now() + Duration::from_secs(60 * 10);

    let vault_file_access_code_id = Xid::new();
    sqlx::query!(
        "INSERT INTO vault_file_access_codes (id, vault_file_id, code_hash, expires_at) VALUES ($1, $2, $3, $4)",
        vault_file_access_code_id.as_bytes(),
        vault_file_id,
        vault_file_code_hash,
        expires_at,
    )
    .execute(db.0)
    .await
    .unwrap();

    Ok(Json(json!({"code": vault_file_code})))
}
