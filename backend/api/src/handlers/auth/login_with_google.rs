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
    
    let res = client.post(token_url)
        .form(&[
            ("code", params.code.as_str()),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("redirect_uri", redirect_url.as_str()),
            ("grant_type", "authorization_code")
        ])
        .send().await;
    let res = match res {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{err:?}");
            return
        }
    };
    let json = res.json::<serde_json::Value>().await;
    let json = match json {
        Ok(val) => val,
        Err(err) => {
            eprintln!("{err:?}"),
            return
        }
    };
    
    let access_token = json.get("access_token");
    let access_token: String = match access_token {
        Some(token) => token,
        None => {
            eprintln!("access token not received");
            return
        }
    };
   
    let user_info = client.get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(access_token.as_str())
        .send()
        .await;
    
    let user_info = match user_info {
        Ok(info) => info,
        Err(err) => {
            eprintln!("{err:?}");
            return
        }
    };
    
    let parsed_user_info = user_info.json::<serde_json::Value>().await;
    let parsed_user_info = match parsed_user_info {
        Ok(val) => val,
        Err(err) => {
            eprintln!("{err:?}");
            return 
        }
    };
} 