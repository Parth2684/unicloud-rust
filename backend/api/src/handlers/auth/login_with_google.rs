use axum::{extract::Query, response::Redirect};
use url::Url;
use reqwest::Client;

use crate::export_envs::ENVS;

struct AuthRequest {
    code: String,
    state: String
}


pub fn google_auth_redirect() -> Redirect {
    let client_id = &ENVS.google_client_id;
    let redirect_url = &ENVS.google_client_redirect_url;
    
    let auth_url = Url::parse_with_params("https://accounts.google.com/o/oath2/v2/auth", &[
        ("client_id", client_id.as_str()),
        ("redirect_url", redirect_url.as_str()),
        ("response_type", "code"),
        ("scope", "openid email profile"),
        ("acces_type", "offline"),
        ("prompt", "consent")
    ]);
    match auth_url {
        Ok(url) => Redirect::to(url.as_str()),
        Err(err) => {
            println!("{err:?}");
            Redirect::to("/auth/google")
        }
    }
}


pub async fn google_auth_callback(Query(params): Query<AuthRequest>) {
    let client = Client::new();
    let token_url = String::from("https://oauth2.googleapis.com/token");
    let client_id = &ENVS.google_client_id;
    let client_secret = &ENVS.google_client_secret;
    let redirect_url = &ENVS.google_client_redirect_url;
    
    
} 