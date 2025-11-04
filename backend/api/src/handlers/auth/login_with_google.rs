use axum::{extract::Query, response::Redirect};
use entities::user;
use sea_orm::ActiveValue::Set;
use sea_orm::DbBackend;
use sea_orm::Insert;
use sea_orm::QueryTrait;
use url::Url;
use reqwest::Client;
use entities::user::Entity as UserEntity;
use entities::user::Column as UserColumn;
use crate::db_connect::init_db;
use crate::{export_envs::ENVS};
use sea_orm::EntityTrait;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use uuid::Uuid;
use chrono::prelude::*;
use sea_orm::ActiveModelTrait;

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
            eprintln!("{err:?}");
            Redirect::to("/auth/google")
        }
    }
}

struct AuthRequest {
    code: String,
    state: Option<String>
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
    let access_token = match json {
        Ok(val) => match val.get("access_token") {
            Some(token) => token.to_string(),
            None => {
                eprintln!("Access token not received");
                return
            }
        },
        Err(err) => {
            eprintln!("{err:?}");
            return
        }
    };
    
    let db = init_db().await;
   
    let user_info = client.get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(access_token)
        .send()
        .await;
    
    let user_info = match user_info {
        Ok(info) => match info.json::<serde_json::Value>().await {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Error parsing user info: {err}");
                return
            }
        },
        Err(err) => {
            eprintln!("{err:?}");
            return
        }
    };
    
    let email = user_info.get("email")
        .expect("Email is required for signing in");
    let name = user_info.get("given_name")
        .expect("Name is required for signing in");
    let image = user_info.get("picture")
        .expect("Picture is required for signing in");
    let sub = user_info.get("sub")
        .expect("Sub should be provided from google");
    
    let db_user =  UserEntity::find()
        .filter(UserColumn::Gmail.eq(email.as_str()))
        .one(db)
        .await; 
    
    let db_user = match db_user {
        Ok(optional_user) => optional_user,
        Err(err) => {
            println!("{err}");
            return
        }
    };
    
    let final_user = match db_user {
        Some(user) => user,
        None => {
            let uuidv4 = Uuid::new_v4(); 
            let utc = Utc::now().naive_utc();
            let insert_user = user::ActiveModel {
                id: Set(uuidv4),
                gmail: Set(email.to_string()),
                created_at: Set(utc),
                image: Set(Some(image.to_string())),
                sub: Set(sub.to_string()),
                name: Set(name.to_string())
            };
            let user:user::Model = match insert_user.insert(db).await {
                Ok(user) => user,
                Err(err) => {
                    eprintln!("{err}");
                    return
                }
            };
            
            user
        }
    }; 
    
} 