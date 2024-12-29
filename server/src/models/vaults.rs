use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{types::Json, FromRow};

use crate::utils::xid::Xid;

#[allow(dead_code)]
#[derive(Debug, FromRow, Serialize)]
pub struct Vault {
    pub id: Xid,
    pub name: String,
    pub provider: String,
    pub data: Json<serde_json::Value>,
}

#[allow(dead_code)]
#[derive(Debug, FromRow)]
pub struct UserVaultLink {
    pub user_id: Xid,
    pub vault_id: Xid,
    pub is_admin: bool,
}

#[allow(dead_code)]
#[derive(Debug, FromRow, Serialize)]
pub struct VaultFile {
    pub id: Xid,
    pub vault_id: Xid,
    pub path_id: String,
    pub name: String,
    pub file_type: String,
    pub parent_id: Option<Vec<u8>>,
    pub created_at: Option<DateTime<Utc>>,
    pub size: Option<i64>,
}
