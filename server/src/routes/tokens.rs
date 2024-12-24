use std::time::Duration;

use poem::{
    handler,
    web::{Data, Json},
};
use serde::Deserialize;

use crate::{
    config::Config,
    models::users::UserRefreshToken,
    utils::{
        response_errors::{internal_server_error, ForbiddenError},
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
) -> poem::Result<Json<user_security::UserAccessToken>> {
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
            .map_err(internal_server_error)?;

            if refresh_token.is_none() {
                return Err(ForbiddenError.into());
            }
            let refresh_token = refresh_token.unwrap();

            Ok(generate_access_token(&config.jwt_signing_key, &refresh_token.user_id))
        })
    })
    .await;

    Ok(Json(access_token?))
}
