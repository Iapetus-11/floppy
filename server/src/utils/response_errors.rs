use std::fmt::Debug;

use poem::{error::ResponseError, http::StatusCode};

#[derive(Debug, thiserror::Error)]
#[error("Forbidden")]
pub struct ForbiddenError;

impl ResponseError for ForbiddenError {
    fn status(&self) -> StatusCode {
        StatusCode::FORBIDDEN
    }
}

pub fn internal_server_error<T: Debug>(err: T) -> poem::Error {
    println!("Unhandled internal server error: {:#?}", err);
    poem::Error::from_status(StatusCode::INTERNAL_SERVER_ERROR)
}
