use crate::utils::app_errors::AppError;
use crate::utils::db_connect::init_db;
use crate::utils::export_envs::ENVS;
use crate::handlers::auth::jwt_config::create_jwt;
use axum::response::IntoResponse;
use axum::{extract::Query, response::Redirect};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::cookie::CookieJar;
use chrono::prelude::*;
use entities::quota::{Column as QuotaColumn, Entity as QuotaEntity};
use entities::sea_orm_active_enums::QuotaType;
use entities::users::{Column as UserColumn, Entity as UserEntity};
use entities::{cloud_account, quota, users};
use reqwest::Client;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

pub async fn google_auth_redirect() -> Redirect {
    let client_id = &ENVS.google_client_id;
    let redirect_url = &ENVS.google_client_redirect_url;

    let auth_url = Url::parse_with_params(
        "https://accounts.google.com/o/oauth2/v2/auth",
        &[
            ("client_id", client_id.as_str()),
            ("redirect_uri", redirect_url.as_str()),
            ("response_type", "code"),
            ("scope", "openid email profile"),
            ("access_type", "offline"),
            ("prompt", "consent"),
        ],
    );
    match auth_url {
        Ok(url) => Redirect::to(url.as_str()),
        Err(err) => {
            eprintln!("{err:?}");
            Redirect::to(&format!("{}/auth/google", &ENVS.backend_url.as_str()))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    pub code: String,
    pub state: Option<String>,
}

pub async fn google_auth_callback(
    jar: CookieJar,
    Query(params): Query<AuthRequest>,
) -> Result<impl IntoResponse, AppError> {
    let client = Client::new();
    let token_url = String::from("https://oauth2.googleapis.com/token");
    let client_id = &ENVS.google_client_id;
    let client_secret = &ENVS.google_client_secret;
    let redirect_url = &ENVS.google_client_redirect_url;

    let res = client
        .post(token_url)
        .form(&[
            ("code", params.code.as_str()),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("redirect_uri", redirect_url.as_str()),
            ("grant_type", "authorization_code"),
        ])
        .send()
        .await;
    let res = match res {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{err:?}");
            return Err(AppError::Internal(Some(
                "Error while getting token".to_string(),
            )));
        }
    };
    let json = res.json::<serde_json::Value>().await;
    println!("{json:?}");
    let access_token = match &json {
        Ok(val) => match val.get("access_token") {
            Some(token) => {
                token.as_str().unwrap_or_default().to_string()
            }
            None => {
                eprintln!("Access token not received, {:?}", val);
                return Err(AppError::Internal(Some(
                    "Access token not received".to_string(),
                )));
            }
        },
        Err(err) => {
            eprintln!("{err:?}");
            return Err(AppError::Internal(Some(
                "Couldn't Retrieve Token from Google".to_string(),
            )));
        }
    };

    let db = init_db().await;

    let user_info = client
        .get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(access_token)
        .send()
        .await;

    let user_info = match user_info {
        Ok(info) => match info.json::<serde_json::Value>().await {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Error parsing user info: {err}");
                return Err(AppError::Internal(Some(
                    "Error while Parsing user info".to_string(),
                )));
            }
        },
        Err(err) => {
            eprintln!("{err:?}");
            return Err(AppError::Internal(Some(
                "Error while getting user info".to_string(),
            )));
        }
    };

    let email = user_info
        .get("email")
        .expect("Email is required for signing in");
    let name = user_info
        .get("given_name")
        .expect("Name is required for signing in");
    let image = user_info
        .get("picture")
        .expect("Picture is required for signing in");
    let sub = user_info
        .get("sub")
        .expect("Sub should be provided from google");


    let db_user = UserEntity::find()
        .filter(UserColumn::Sub.eq(sub.to_string()))
        .one(db)
        .await;

    let db_user = match db_user {
        Ok(optional_user) => optional_user,
        Err(err) => {
            eprintln!("{err}");
            return Err(AppError::Internal(Some(
                "Database service is probably down".to_string(),
            )));
        }
    };

    let cloud_accounts = cloud_account::Entity::find()
        .filter(cloud_account::Column::Sub.eq(sub.as_str()))
        .one(db)
        .await;

    if let Ok(Some(_)) = cloud_accounts {
        return Err(AppError::Forbidden(Some(
            "You Cannot Signin with this account as it was added by a different account"
                .to_string(),
        )));
    }

    let final_user = match db_user {
        Some(user_found) => {
            let mut update_user: users::ActiveModel = user_found.into();
            update_user.gmail = Set(email.to_string());
            update_user.name = Set(name.to_string());
            update_user.image = Set(Some(image.to_string()));
            let user: users::Model = match update_user.update(db).await {
                Ok(user) => user,
                Err(err) => {
                    eprintln!("{err:?}");
                    return Err(AppError::Internal(Some(String::from(
                        "Error Updating User",
                    ))));
                }
            };
            user
        }
        None => {
            let uuidv4 = Uuid::new_v4();
            let utc = Utc::now().naive_utc();
            let insert_user = users::ActiveModel {
                id: Set(uuidv4),
                gmail: Set(email.to_string()),
                created_at: Set(utc),
                image: Set(Some(image.to_string())),
                sub: Set(sub.to_string()),
                name: Set(name.to_string()),
            };
            let user: users::Model = match insert_user.insert(db).await {
                Ok(user) => user,
                Err(err) => {
                    eprintln!("{err}");
                    return Err(AppError::Internal(Some(String::from(String::from(
                        "Error Creating User Please try again",
                    )))));
                }
            };
            user
        }
    };

    let user_quota = QuotaEntity::find()
        .filter(QuotaColumn::UserId.eq(final_user.id))
        .one(db)
        .await;

    let user_quota: quota::Model = match user_quota {
        Ok(optional_quota) => match optional_quota {
            Some(quota) => quota,
            None => {
                let uuid = Uuid::new_v4();
                let quota_db = quota::ActiveModel {
                    id: Set(uuid),
                    user_id: Set(final_user.id),
                    ..Default::default()
                };
                let quota: quota::Model = match quota_db.insert(db).await {
                    Ok(quota) => quota,
                    Err(err) => {
                        eprintln!("{err:?}");
                        return Err(AppError::Internal(Some(String::from(
                            "Could not create Quota for user",
                        ))));
                    }
                };
                quota
            }
        },
        Err(err) => {
            eprintln!("{err:?}");
            return Err(AppError::Internal(Some(String::from(
                "Could not create a quota for you please try creating account again DBERR",
            ))));
        }
    };

    let quota_type = match user_quota.quota_type {
        QuotaType::Free => "Free",
        QuotaType::Bronze => "Bronze",
        QuotaType::Silver => "Silver",
        QuotaType::Gold => "Gold",
        QuotaType::Platinum => "Platinum",
    };
    let token = create_jwt(&final_user.id.to_string(), quota_type);
    match token {
        Ok(jwt) => {
            let secure = ENVS.environment != "DEVELOPMENT";

            let cookie = Cookie::build(("auth_token", jwt))
                .path("/")
                .domain(&ENVS.frontend_url)
                .http_only(true)
                .same_site(axum_extra::extract::cookie::SameSite::Lax)
                .secure(secure);

            let _ = jar.clone().add(cookie);
            Ok((jar, Redirect::to(&format!("{}/home", &ENVS.frontend_url))).into_response())
        }
        Err(err) => {
            eprintln!("{err:?}");
            Err(AppError::Internal(Some(String::from(
                "Could not generate token",
            ))))
        }
    }
}
