use std::{
    collections::HashMap, error::Error, fs, os::unix::fs::MetadataExt, path::PathBuf, str::FromStr, sync::mpsc, thread
};

use chrono::{DateTime, Utc};
use notify::{EventKind, Watcher};

use crate::{
    models::vaults::{Vault, VaultFile},
    utils::{
        folders::{walk_directory, FileType},
        xid::Xid,
    },
};

fn watch_local_folder_vault(
    db: sqlx::Pool<sqlx::Postgres>,
    vault: Vault,
) -> Result<(), Box<dyn Error + Send>> {
    let vault_path = PathBuf::from_str(vault.data["path"].as_str().unwrap()).unwrap();
    fs::create_dir_all(&vault_path).unwrap();

    let async_runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let (fs_events_tx, fs_events_rx) = mpsc::channel::<notify::Result<notify::Event>>();

    let mut watcher = notify::recommended_watcher(fs_events_tx).unwrap();

    watcher
        .watch(&vault_path, notify::RecursiveMode::Recursive)
        .unwrap();

    for event in fs_events_rx {
        let result: Result<(), Box<dyn Error>> = match event {
            Err(error) => {
                Err(error.into())
            },
            Ok(event) => async_runtime.block_on(async {
                struct EventFile {
                    path: String,
                    name: String,
                }

                let event_files = event.paths.iter().filter(|f| f != &&vault_path).map(|p| EventFile {
                    path: p.as_os_str().to_string_lossy().to_string(),
                    name: p.file_name().unwrap().to_string_lossy().to_string(),
                }).collect::<Vec<_>>();

                // We get two modifies for a rename... one to yeet the old file and one for the new one, you have to
                // run some code to check if the path exists & remove/add it.
                match event.kind {
                    EventKind::Create(_) => {
                        for file in event_files {
                            let file_id = Xid::new();

                            sqlx::query!(
                                "INSERT INTO vault_files (id, vault_id, path_id, name) VALUES ($1, $2, $3, $4)",
                                file_id.as_bytes(), vault.id.as_bytes(), file.path, file.name,
                            ).execute(&db)
                            .await?;
                        }
                    }
                    EventKind::Remove(_) => {
                        sqlx::query!(
                            "DELETE FROM vault_files WHERE vault_id = $1 AND path_id = ANY($2)",
                            vault.id.as_bytes(), &event_files.into_iter().map(|f| f.path).collect::<Vec<_>>()
                        ).execute(&db)
                        .await?;
                    },
                    EventKind::Modify(_) => {
                        for path in event.paths {
                            reindex_local_folder_vault_file(&db, vault.id, path).await?;
                        }
                    }
                    _ => {},
                }

                Ok(())
            })
        };

        if let Err(error) = result {
            println!("An error occurred while handling event for path {vault_path:?} for vault {}: {error}", vault.id.to_string())
        }
    }

    Ok(())
}

pub async fn setup_local_folder_vault_watchers(
    db: sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Box<dyn Error>> {
    let vaults = sqlx::query_as!(
        Vault,
        "SELECT id, name, provider, data FROM vaults WHERE provider = 'local_folder'",
    )
    .fetch_all(&db)
    .await?;

    for vault in vaults {
        let db = db.clone();
        thread::spawn(|| watch_local_folder_vault(db, vault));
    }

    Ok(())
}

pub async fn reindex_local_folder_vault_file(
    db: &sqlx::Pool<sqlx::Postgres>,
    vault_id: Xid,
    path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let path_id = path.as_os_str().to_string_lossy().to_string();

    match path.try_exists()? {
        true => {
            let id = Xid::new();
            let name = path.file_name().unwrap().to_string_lossy().to_string();

            let parent = sqlx::query_as!(
                VaultFile,
                "SELECT id, vault_id, path_id, name, file_type, parent_id, created_at, size FROM vault_files WHERE vault_id = $1 AND path_id = $2",
                vault_id.as_bytes(), path_id,
            )
            .fetch_optional(db)
            .await?;

            let parent_id = parent.map(|p: VaultFile| p.id.as_bytes().to_vec());

            sqlx::query!(
                "INSERT INTO vault_files (id, vault_id, path_id, name, file_type, parent_id) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING",
                id.as_bytes(), vault_id.as_bytes(), path_id, name, "file".to_string(), parent_id
            ).execute(db)
            .await?;
        }
        false => {
            sqlx::query!(
                "DELETE FROM vault_files WHERE vault_id = $1 AND path_id = $2",
                vault_id.as_bytes(),
                path_id,
            )
            .execute(db)
            .await?;
        }
    }

    Ok(())
}

pub async fn reindex_local_folder_vault(
    db: sqlx::Pool<sqlx::Postgres>,
    vault: Vault,
) -> Result<usize, Box<dyn Error>> {
    let mut db = db.begin().await?;

    sqlx::query!(
        "DELETE FROM vault_files WHERE vault_id = $1",
        vault.id.as_bytes(),
    )
    .execute(&mut *db)
    .await?;

    let mut file_count: usize = 0;
    let mut parent_map = HashMap::<PathBuf, Xid>::new();

    for (file_path, file_type) in
        walk_directory(PathBuf::from_str(vault.data["path"].as_str().unwrap())?)?
    {
        let id = Xid::new();
        let path_id = file_path.to_string_lossy().to_string();
        let name = file_path.file_name().unwrap().to_string_lossy().to_string();

        // Assumes walk_directory is in order
        let parent_id = parent_map
            .get(&file_path.parent().unwrap().to_path_buf())
            .map(|v| v.as_bytes() as &[u8]);

        let (created_at, file_size) = match file_path.metadata() {
            Err(_) => (None, None),
            Ok(metadata) => (
                metadata
                    .created()
                    .map(|created_at| Some(DateTime::<Utc>::from(created_at)))
                    .unwrap_or(None),
                    Some(metadata.size() as i64),
            )
        };

        sqlx::query!(
            "INSERT INTO vault_files (id, vault_id, path_id, name, file_type, parent_id, created_at, size) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            id.as_bytes(),
            vault.id.as_bytes(),
            path_id,
            name,
            file_type.to_string(),
            parent_id,
            created_at,
            file_size,
        )
        .execute(&mut *db)
        .await?;

        if file_type == FileType::Folder {
            parent_map.insert(file_path, id);
        }

        file_count += 1;
    }

    db.commit().await?;

    Ok(file_count)
}
