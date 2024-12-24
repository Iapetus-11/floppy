use poem::{post, Route};

mod login;
mod tokens;

pub fn setup_routes() -> Route {
    Route::new()
        .at(
            "/login/email_and_password/",
            post(login::login_email_and_password),
        )
        .at("/tokens/refresh/", post(tokens::refresh))
}
