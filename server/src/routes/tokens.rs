use std::time::Duration;

use chrono::Utc;
use poem::{
    handler,
    http::HeaderMap,
    web::{
        headers::{HeaderMapExt, UserAgent},
        Data, Json, RemoteAddr,
    },
};
use serde::Deserialize;

use crate::{
    config::Config,
    models::users::UserRefreshToken,
    utils::{
        response_errors::ForbiddenError,
        security::ensure_execution_time,
        user_security::{self, generate_access_token, hash_refresh_token},
    },
};

#[derive(Deserialize)]
struct RefreshTokenData {
    refresh_token: String,
}

#[handler]
pub async fn refresh(
    db: Data<&sqlx::Pool<sqlx::Postgres>>,
    config: Data<&Config>,
    data: Json<RefreshTokenData>,
    headers: &HeaderMap,
    remote_address: &RemoteAddr,
) -> poem::Result<Json<user_security::UserAccessToken>> {
    let user_agent = headers.typed_get::<UserAgent>().map(|ua| ua.to_string());

    // TODO: Support X-Forwarded-For / CF-Connecting-IP
    let remote_address = remote_address.to_string();

    let access_token: Result<user_security::UserAccessToken, poem::Error> = ensure_execution_time(Duration::from_millis(1000), || {
        Box::pin(async {
            let hashed_refresh_token = hash_refresh_token(&data.refresh_token);

            let refresh_token = sqlx::query_as!(
                UserRefreshToken,
                "SELECT id, user_id, token_hash, created_at, last_used_at, user_agent, remote_address FROM user_refresh_tokens WHERE token_hash = $1",
                hashed_refresh_token,
            )
            .fetch_optional(db.0)
            .await
            .unwrap();

            if refresh_token.is_none() {
                return Err(ForbiddenError.into());
            }
            let refresh_token = refresh_token.unwrap();

            sqlx::query!(
                "UPDATE user_refresh_tokens SET last_used_at = $2, user_agent = $3, remote_address = $4 WHERE id = $1",
                refresh_token.id.as_bytes(), Utc::now(), user_agent, remote_address
            )
            .execute(db.0)
            .await
            .unwrap();

            Ok(generate_access_token(&config.jwt_signing_key, &refresh_token.user_id))
        })
    })
    .await;

    Ok(Json(access_token?))
}
