use axum::{Extension, extract::Query, response::{IntoResponse, Redirect}};
use reqwest::Client;
use url::Url;
use crate::utils::encrypt::encrypt;

use crate::{handlers::auth::{jwt_config::Claims, login_with_google::AuthRequest}, utils::{app_errors::AppError, db_connect::init_db, export_envs::ENVS}};


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

pub struct OauthReqResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String
}

pub async fn drive_auth_callback(Extension(claims): Extension<Claims>, Query(params): Query<AuthRequest>) -> Result<_, AppError> {
    let client = Client::new();
    let token_url = String::from("https://oauth2.googleapis.com/token");
    let client_id = &ENVS.google_drive_client_id;
    let client_secret = &ENVS.google_drive_client_secret;
    let redirect_uri = &ENVS.google_drive_redirect_url;
    
    let (res, db) = tokio::join!(
        client
            .post(token_url)
            .form(&[
                ("code", params.code.as_str()),
                ("client_id", client_id.as_str()),
                ("client_secret", client_secret.as_str()),
                ("redirect_uri", redirect_uri.as_str()),
                ("grant_type", "authorization_code"),
            ])
            .send(),
        init_db()
    );
    let res = match res {
        Ok(res) => res,
        Err(err) => return Err(AppError::Internal(Some("Couldn't receive access tokens from google".to_string())))
    };
    
    let json = res.json::<serde_json::Value>().await?;
    let access_token = match json.get("access_token"){
        Some(token) => encrypt(&token.to_string()),
        None => return Err(AppError::Forbidden(Some(String::from("No token receiverd from google"))))
    };
    
    let expires_in = json.get("expires_in");
    let refresh_token = match json.get("refresh_token") {
        Some(token) => encrypt(&token.to_string()),
        None => return Err(AppError::Forbidden(Some(String::from("Couldn't receive refresh token from the server"))))
    };
    
    let (access_token, refresh_token) = match (access_token, refresh_token) {
        (Ok(at), Ok(rt)) => {
            return (at, rt);
        },
        (Err(at), Ok(rt)) => {
            return Err(AppError::Internal(Some(String::from("Access TOken was failed to encrypt"))))
        },
        (Err(at), Err(rt)) => {
            return Err(AppError::Internal(Some(String::from("Both toke was failed encrypt")))))
        },
        (Ok(at), Err(rt)) => {
            return Err(AppError::Internal(Some(String::from("Refresh token was failed to encrypt"))))
        }
    };
}