use poem::{get, post, Route};

mod login;
mod tokens;
mod vaults;

pub fn setup_routes() -> Route {
    Route::new()
        .at(
            "/login/email_and_password/",
            post(login::login_email_and_password),
        )
        .at("/tokens/refresh/", post(tokens::refresh))
        .at("/vaults/", get(vaults::list_vaults))
        .at("/vaults/:vault_id/files/", get(vaults::list_vault_files))
        .at(
            "/vaults/:vault_id/files/:file_id/",
            get(vaults::download_vault_file),
        )
        .at(
            "/vaults/:vault_id/files/:file_id/access_code/",
            get(vaults::get_vault_file_access_code),
        )
}
