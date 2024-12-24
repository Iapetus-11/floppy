use config::{load_config, Config};
use core::panic;
use poem::{
    listener::TcpListener,
    middleware::{AddData, CatchPanic, NormalizePath, TrailingSlash},
    EndpointExt, Server,
};
use std::{
    env::{self},
    error::Error,
};

mod cli;
mod config;
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
        "" => println!("Please specify one of the following commands: serve, createuser"),
        "serve" => serve_api(config, pool).await?,
        "createuser" => cli::create_user(config, pool, &mut args).await?,
        cmd => panic!("Unknown command {:#?}", cmd),
    }

    Ok(())
}

async fn serve_api(config: Config, pool: sqlx::Pool<sqlx::Postgres>) -> Result<(), Box<dyn Error>> {
    let routes = routes::setup_routes()
        .with(NormalizePath::new(TrailingSlash::Always))
        .with(AddData::new(config.clone()))
        .with(AddData::new(pool))
        .with(CatchPanic::new());

    println!("Starting API on http://{0} ...", config.server_host_address);

    Server::new(TcpListener::bind(config.server_host_address))
        .run(routes)
        .await?;

    Ok(())
}
