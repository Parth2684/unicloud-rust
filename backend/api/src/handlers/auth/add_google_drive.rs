use axum::response::Redirect;
use url::Url;

use crate::utils::export_envs::ENVS;


pub async fn drive_auth_redirect() -> Redirect {
    let auth_url = Url::parse_with_params("https://accounts.google.com/o/oauth2/v2/auth",[
        ("client_id", *&ENVS.google_drive_client_id.as_str()),
        ("redirect_uri", *&ENVS.google_drive_redirect_url.as_str()),
        ("response_type", "code"),
        ("scope", "drive"),
        ("access_type", "offline"),
        ("prompt", "consent")
    ]);
    match auth_url {
        Ok(uri) => Redirect::to(uri.as_str()),
        Err(err) => {
            eprintln!("Error Parsing uri {:?}",err);
            Redirect::to(&format!("{}/auth/drive", &ENVS.backend_url))
        }
    }
}