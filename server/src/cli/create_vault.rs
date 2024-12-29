use std::{env::Args, error::Error};

use crate::{cli::arguments::CommandError, config::Config, utils::xid::Xid};

use super::arguments::{handle_arg_error, require_arg};

pub async fn create_vault(
    _config: Config,
    db: sqlx::Pool<sqlx::Postgres>,
    args: &mut Args,
) -> Result<(), Box<dyn Error>> {
    let command_syntax = "createvault <name> <provider> <json>".to_string();
    let arg_error_handler = handle_arg_error(command_syntax);

    let name = require_arg::<String>("name".into(), args).map_err(&arg_error_handler)?;
    let provider = require_arg::<String>("provider".into(), args).map_err(&arg_error_handler)?;
    let json_data = require_arg::<String>("data".into(), args).map_err(&arg_error_handler)?;

    let json_data = serde_json::from_str::<serde_json::Value>(&json_data);

    if let Err(json_parse_err) = json_data {
        println!("Please provide valid json\n{:#?}", json_parse_err);
        return Err(CommandError("Invalid json provided to <json> argument".to_string()).into());
    }
    let json_data = json_data.unwrap();

    let vault_name_in_use: (bool,) =
        sqlx::query_as("SELECT EXISTS(SELECT FROM vaults WHERE name = $1)")
            .bind(&name)
            .fetch_one(&db)
            .await?;

    if vault_name_in_use.0 {
        println!("Vault name {} is already in use", &name);
        return Err(CommandError("Vault name already in use".to_string()).into());
    }

    let vault_id = Xid::new();

    sqlx::query!(
        "INSERT INTO vaults (id, name, provider, data) VALUES ($1, $2, $3, $4)",
        vault_id.as_bytes(),
        name,
        provider,
        json_data,
    )
    .execute(&db)
    .await?;

    println!(
        "Successfully created vault {} ({} - {})",
        vault_id.to_string(),
        name,
        provider
    );

    Ok(())
}
