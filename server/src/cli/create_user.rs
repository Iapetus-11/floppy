use std::{env::Args, error::Error};

use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use chrono::{DateTime, Utc};
use rand::rngs::OsRng;
use serde_json::json;

use crate::{
    cli::arguments::{handle_arg_error, require_arg, CommandError},
    config::Config,
    utils::xid::Xid,
};

pub async fn create_user(
    _config: Config,
    db: sqlx::Pool<sqlx::Postgres>,
    args: &mut Args,
) -> Result<(), Box<dyn Error>> {
    let command_syntax = "createuser <name> <email> <password>".to_string();
    let arg_error_handler = handle_arg_error(command_syntax);

    let name = require_arg::<String>("name".into(), args).map_err(&arg_error_handler)?;
    let email = require_arg::<String>("email".into(), args).map_err(&arg_error_handler)?;
    let password = require_arg::<String>("password".into(), args).map_err(&arg_error_handler)?;

    let email_already_in_use: (bool,) =
        sqlx::query_as("SELECT EXISTS(SELECT FROM users WHERE email ILIKE $1)")
            .bind(&email)
            .fetch_one(&db)
            .await?;

    if email_already_in_use.0 {
        println!("Email address {:?} is already in use", email);
        return Err(CommandError("Email address already in use".to_string()).into());
    }

    let mut db = db.begin().await?;

    let user_id = Xid::new();

    sqlx::query!(
        "INSERT INTO users (id, created_at, last_login_at, name, email) VALUES ($1, $2, $3, $4, $5)",
        user_id.as_bytes(), Utc::now(), <Option<DateTime<Utc>>>::None, name, email,
    ).execute(&mut *db)
    .await?;

    let user_identity_id = Xid::new();

    let hashed_password: String = {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string()
    };

    sqlx::query!(
        "INSERT INTO user_identities (id, user_id, provider, data) VALUES ($1, $2, $3, $4)",
        user_identity_id.as_bytes(),
        user_id.as_bytes(),
        "email_and_password",
        json!({"password": hashed_password})
    )
    .execute(&mut *db)
    .await?;

    db.commit().await?;

    println!(
        "Successfully created user {} ({} - {})",
        user_id.to_string(),
        name,
        email
    );

    Ok(())
}
