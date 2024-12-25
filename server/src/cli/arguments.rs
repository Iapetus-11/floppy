use std::{any::type_name, env::Args, fmt::Display, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArgumentError {
    #[error("Missing argument {}", .arg_name)]
    Missing { arg_name: String },

    #[error("Expected argument {} to be of type {}", .arg_name, .arg_type)]
    Type { arg_name: String, arg_type: String },
}

#[derive(Error, Debug)]
pub struct CommandError(pub String);

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

pub fn require_arg<T: FromStr>(arg_name: String, args: &mut Args) -> Result<T, ArgumentError> {
    match args.next() {
        None => Err(ArgumentError::Missing { arg_name }),
        Some(value) => {
            let parsed = value.parse::<T>();

            match parsed {
                Ok(value) => Ok(value),
                Err(_) => Err(ArgumentError::Type {
                    arg_name,
                    arg_type: type_name::<T>().to_string(),
                }),
            }
        }
    }
}

pub fn handle_arg_error(command_syntax: String) -> Box<dyn Fn(ArgumentError) -> ArgumentError> {
    Box::new(move |error| {
        println!(
            "Command Error: {}. Example usage: {}",
            error, command_syntax
        );
        error
    })
}
