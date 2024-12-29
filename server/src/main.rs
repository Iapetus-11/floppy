use config::{load_config, Config};
use core::panic;
use logic::indexing::local_folder::setup_local_folder_vault_watchers;
use poem::{
    listener::TcpListener,
    middleware::{AddData, CatchPanic, Cors, NormalizePath, TrailingSlash},
    EndpointExt, Server,
};
use std::{
    env::{self},
    error::Error,
};

mod cli;
mod config;
mod logic;
mod models;
mod routes;
mod utils;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.database_pool_size)
        .connect(&config.database_url)
        .await?;

    let mut args = env::args();
    args.next();

    match args.next().unwrap_or("".to_string()).as_str() {
        "" => {
            println!("Please specify one of the following commands: serve, createuser, createvault")
        }
        "serve" => run_server(config, pool).await?,
        "createuser" | "create_user" => cli::create_user(config, pool, &mut args).await?,
        "createvault" | "create_vault" => cli::create_vault(config, pool, &mut args).await?,
        "indexvault" | "index_vault" => cli::index_vault(config, pool, &mut args).await?,
        cmd => panic!("Unknown command {:#?}", cmd),
    }

    Ok(())
}

async fn run_api(config: Config, pool: sqlx::Pool<sqlx::Postgres>) -> Result<(), Box<dyn Error>> {
    let routes = routes::setup_routes()
        .with(NormalizePath::new(TrailingSlash::Always))
        .with(AddData::new(config.clone()))
        .with(AddData::new(pool.clone()))
        .with(CatchPanic::new())
        .with(Cors::new().allow_origin(config.frontend_url));

    println!(
        "Starting server on http://{0} ...",
        config.server_host_address
    );

    Server::new(TcpListener::bind(config.server_host_address))
        .run(routes)
        .await?;

    Ok(())
}

async fn run_server(
    config: Config,
    pool: sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Box<dyn Error>> {
    setup_local_folder_vault_watchers(pool.clone()).await?;

    run_api(config, pool.clone()).await?;

    Ok(())
}
