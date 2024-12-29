use std::{env::Args, error::Error};

use crate::{
    config::Config,
    logic::indexing::local_folder::reindex_local_folder_vault,
    models::vaults::Vault,
    utils::{hex::decode_hex, xid::Xid},
};

use super::arguments::{handle_arg_error, require_arg, CommandError};

pub async fn index_vault(
    _config: Config,
    db: sqlx::Pool<sqlx::Postgres>,
    args: &mut Args,
) -> Result<(), Box<dyn Error>> {
    let command_syntax = "indexvault <vault_id>".to_string();
    let arg_error_handler = handle_arg_error(command_syntax);

    let vault_id =
        require_arg::<String>("vault_id".to_string(), args).map_err(arg_error_handler)?;

    let vault_id = match vault_id.len() {
        24 => {
            Xid::from(decode_hex(&vault_id).map_err(|_| {
                CommandError("Expected valid hex representation of XID bytes".into())
            })?)
        }
        _ => Xid::try_from(vault_id.as_str())
            .map_err(|_| CommandError("The vault_id parameter must be a valid XID".to_string()))?,
    };

    let vault = sqlx::query_as!(
        Vault,
        "SELECT id, name, provider, data FROM vaults WHERE id = $1",
        vault_id.as_bytes()
    )
    .fetch_optional(&db)
    .await?;

    if vault.is_none() {
        return Err(CommandError(
            "The vault_id parameter must refer to an actual configured Vault".to_string(),
        )
        .into());
    }
    let vault = vault.unwrap();

    let file_count = reindex_local_folder_vault(db, vault).await?;

    println!(
        "Successfully reindexed vault {} (found {} files)",
        vault_id.to_string(),
        file_count
    );

    Ok(())
}
