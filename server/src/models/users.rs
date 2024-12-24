use chrono::{DateTime, Utc};
use sqlx::{types::Json, FromRow};

use crate::utils::xid::Xid;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Xid,
    pub created_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub name: String,
    pub email: String,
}

#[derive(Debug, FromRow)]
pub struct UserIdentity {
    pub id: Xid,
    pub provider: String,
    pub data: Json<serde_json::Value>,
}

#[derive(Debug, FromRow)]
pub struct UserRefreshToken {
    pub id: Xid,
    pub user_id: Xid,
    pub token_hash: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub user_agent: Option<String>,
    pub remote_address: String,
}
