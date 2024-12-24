use std::{any::type_name, env, str::FromStr};

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub database_pool_size: u32,
    pub server_host_address: String,
    pub jwt_signing_key: String,
}

fn load_env<T: FromStr>(key: &str) -> T {
    let string = env::var(key).unwrap_or_else(|_| panic!("Please set {key} in your .env"));

    let parsed = string.parse::<T>();

    match parsed {
        Ok(value) => value,
        Err(_) => {
            let type_name = type_name::<T>();
            panic!("Expected {key} to be a valid {type_name} in your .env");
        }
    }
}

pub fn load_config() -> Config {
    dotenv::dotenv().unwrap();

    let database_url: String = load_env("DATABASE_URL");
    let database_pool_size: u32 = load_env("DATABASE_POOL_SIZE");
    let server_host_address: String = load_env("SERVER_HOST_ADDRESS");
    let jwt_signing_key: String = load_env("JWT_SIGNING_KEY");

    Config {
        database_url,
        database_pool_size,
        server_host_address,
        jwt_signing_key,
    }
}
