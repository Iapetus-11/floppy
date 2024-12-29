use crate::{
    models::vaults::{Vault, VaultFile},
    utils::{user_security::AuthenticatedUser, xid::Xid},
};
use poem::{
    error::NotFoundError,
    handler,
    web::{Data, Json, Path, Query},
};
use serde::Deserialize;

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

    let files = match query.parent_id {
        None => {
            sqlx::query_as!(
                VaultFile,
                "SELECT id, vault_id, path_id, name, file_type, parent_id, created_at, size FROM vault_files WHERE vault_id = $1 AND id > $2 AND parent_id IS NULL",
                vault_id.as_bytes(),
                query_after_id,
            ).fetch_all(db.0)
            .await
            .unwrap()
        },
        Some(parent_id) => {
            sqlx::query_as!(
                VaultFile,
                "SELECT id, vault_id, path_id, name, file_type, parent_id, created_at, size FROM vault_files WHERE vault_id = $1 AND id > $2 AND parent_id = $3",
                vault_id.as_bytes(),
                query_after_id,
                parent_id.as_bytes(),
            ).fetch_all(db.0)
            .await
            .unwrap()
        }
    };

    Ok(Json(files))
}
