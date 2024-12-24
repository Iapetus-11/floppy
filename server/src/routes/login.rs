use std::time::Duration;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::{DateTime, Utc};
use poem::{
    handler,
    http::HeaderMap,
    web::{
        headers::{HeaderMapExt, UserAgent},
        Data, Json, RemoteAddr,
    },
    Error,
};
use serde::Deserialize;

use crate::{
    config::Config,
    models::users::User,
    utils::{
        response_errors::{internal_server_error, ForbiddenError},
        security::ensure_execution_time,
        user_security::{self, hash_refresh_token},
    },
};

#[derive(Deserialize)]
struct EmailAndPasswordLoginData {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
struct PasswordIdentityData {
    password: String,
}

#[handler]
pub async fn login_email_and_password(
    db: Data<&sqlx::Pool<sqlx::Postgres>>,
    config: Data<&Config>,
    data: Json<EmailAndPasswordLoginData>,
    headers: &HeaderMap,
    remote_address: &RemoteAddr,
) -> poem::Result<Json<user_security::UserTokens>> {
    let user_agent = headers.typed_get::<UserAgent>().map(|ua| ua.to_string());

    // TODO: Support X-Forwarded-For / CF-Connecting-IP
    let remote_address = remote_address.to_string();

    let tokens: Result<user_security::UserTokens, Error> = ensure_execution_time(Duration::from_millis(1000), || Box::pin(async {
        let user = sqlx::query_as!(
            User,
            "SELECT id, created_at, last_login_at, name, email FROM users WHERE email ILIKE $1",
            data.email,
        )
        .fetch_optional(db.0)
        .await
        .map_err(internal_server_error)?;

        if user.is_none() {
            return Err(ForbiddenError.into());
        }
        let user = user.unwrap();

        let user_identity = sqlx::query!(
            "SELECT data FROM user_identities WHERE user_id = $1 AND provider = $2",
            user.id.as_bytes(),
            "email_and_password"
        )
        .fetch_optional(db.0)
        .await
        .map_err(internal_server_error)?;

        if user_identity.is_none() {
            return Err(ForbiddenError.into());
        }
        let user_identity = user_identity.unwrap();

        let user_identity_data = PasswordIdentityData::deserialize(user_identity.data).unwrap();
        let hashed_password = PasswordHash::new(&user_identity_data.password).unwrap();

        let password_is_correct = Argon2::default()
            .verify_password(data.password.as_bytes(), &hashed_password)
            .is_ok();

        if !password_is_correct {
            return Err(ForbiddenError.into());
        }

        let tokens = user_security::generate_tokens(&config.jwt_signing_key, &user.id);

        let refresh_token_id = xid::new();
        let hashed_refresh_token = hash_refresh_token(&tokens.refresh_token);

        sqlx::query!(
            "INSERT INTO user_refresh_tokens (id, user_id, token_hash, last_used_at, created_at, user_agent, remote_address) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            refresh_token_id.as_bytes(), user.id.as_bytes(), hashed_refresh_token, <Option<DateTime<Utc>>>::None, Utc::now(), user_agent, remote_address,
        )
        .execute(db.0)
        .await
        .map_err(internal_server_error)?;

        Ok(tokens)
    })).await;

    Ok(Json(tokens?))
}
