use crate::{handlers::auth::login_with_google::AuthRequest, utils::app_errors::AppError};
use axum::{Extension, extract::Query, response::Redirect};
use chrono::Utc;
use common::db_connect::init_db;
use common::encrypt::encrypt;
use common::export_envs::ENVS;
use common::jwt_config::Claims;
use entities::cloud_account::{
    ActiveModel as CloudAccountActive, Column as CloudAccountColumn, Entity as CloudAccountEntity,
    Model as CloudAccountModel,
};
use entities::users::{Entity as UserEntity, Model as UserModel};
use reqwest::Client;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait, QueryFilter};
use url::Url;
use uuid::Uuid;

pub async fn drive_auth_redirect() -> Redirect {
    let auth_url = Url::parse_with_params(
        "https://accounts.google.com/o/oauth2/v2/auth",
        [
            ("client_id", &ENVS.google_drive_client_id),
            ("redirect_uri", &ENVS.google_drive_redirect_url),
            ("response_type", &"code".to_owned()),
            (
                "scope",
                &"openid email https://www.googleapis.com/auth/drive".to_owned(),
            ),
            ("access_type", &"offline".to_owned()),
            ("prompt", &"consent".to_owned()),
            ("include_granted_scope", &"true".to_owned()),
        ],
    );
    match auth_url {
        Ok(uri) => Redirect::to(uri.as_str()),
        Err(err) => {
            eprintln!("Error Parsing uri {:?}", err);
            Redirect::to(&format!("{}/auth/drive", &ENVS.backend_url))
        }
    }
}

pub async fn drive_auth_callback(
    Extension(claims): Extension<Claims>,
    Query(params): Query<AuthRequest>,
) -> Result<Redirect, AppError> {
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
        Err(_) => {
            return Err(AppError::Internal(Some(
                "Couldn't receive access tokens from google".to_string(),
            )));
        }
    };

    let json = match res.json::<serde_json::Value>().await {
        Ok(obj) => obj,
        Err(_) => {
            return Err(AppError::Internal(Some(String::from(
                "Couldn't parse response from google",
            ))));
        }
    };

    let access_token = match json.get("access_token") {
        Some(token) => {
            let token = token.as_str();
            match token {
                Some(t) => t.to_owned(),
                None => {
                    return Err(AppError::Internal(Some(String::from(
                        "Error parsing access token",
                    ))));
                }
            }
        }
        None => {
            return Err(AppError::Forbidden(Some(String::from(
                "No token receiverd from google",
            ))));
        }
    };

    let openid_req = client
        .get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(&access_token)
        .send()
        .await;

    let openid_res = match openid_req {
        Ok(res) => match res.json::<serde_json::Value>().await {
            Ok(val) => val,
            Err(err) => return Err(AppError::Internal(Some(err.to_string()))),
        },
        Err(err) => {
            return Err(AppError::Internal(Some(format!(
                "Couldn't retrieve openid from google, {}",
                err
            ))));
        }
    };

    let sub = match openid_res.get("sub") {
        Some(val) => {
            let oid = val.as_str();
            match oid {
                Some(id) => id.to_owned(),
                None => return Err(AppError::Internal(Some(String::from("Error Parsing sub")))),
            }
        }
        None => {
            return Err(AppError::Forbidden(Some(String::from(
                "Couldn't retrieve openid from google",
            ))));
        }
    };
    let email = match openid_res.get("email") {
        Some(mail) => {
            let val = mail.as_str();
            match val {
                Some(m) => m.to_owned(),
                None => {
                    return Err(AppError::Internal(Some(String::from(
                        "Error parsing email",
                    ))));
                }
            }
        }
        None => {
            return Err(AppError::Unauthorised(Some(String::from(
                "Couldn't retrieve email from google",
            ))));
        }
    };

    let image: Option<String> = match openid_res.get("picture") {
        Some(link) => match link.as_str() {
            Some(str) => Some(str.to_owned()),
            None => None,
        },
        None => None,
    };

    let encrypted_access_token = encrypt(&access_token);
    let encrypted_access_token = match encrypted_access_token {
        Ok(token) => token,
        Err(err) => {
            eprintln!("error while encrypting access token {:?}", err);
            return Err(AppError::Internal(Some(
                "Error while encrypting token".to_string(),
            )));
        }
    };

    let expires_in = match json.get("expires_in") {
        None => Utc::now().timestamp() + 3599,
        Some(time) => Utc::now().timestamp() + time.as_i64().unwrap(),
    };
    match json.get("refresh_token") {
        Some(token) => {
            let refresh_token = token.as_str();
            let refresh_token = match refresh_token {
                Some(token) => encrypt(token),
                None => {
                    return Err(AppError::Internal(Some(String::from(
                        "Error parsing refresh token",
                    ))));
                }
            };
            let (cloud_account, user_account): (
                Result<Option<CloudAccountModel>, DbErr>,
                Result<Option<UserModel>, DbErr>,
            ) = tokio::join!(
                async {
                    CloudAccountEntity::find()
                        .filter(CloudAccountColumn::Sub.eq(&sub))
                        .one(db)
                        .await
                },
                async { UserEntity::find_by_id(claims.id).one(db).await }
            );

            let cloud_account = match cloud_account {
                Ok(con) => con,
                Err(err) => return Err(AppError::Internal(Some(err.to_string()))),
            };
            let user_account = match user_account {
                Ok(con) => con,
                Err(err) => return Err(AppError::Internal(Some(err.to_string()))),
            };

            let final_user_account = match user_account {
                Some(acc) => acc,
                None => {
                    return Err(AppError::Forbidden(Some(String::from(
                        "User account not found",
                    ))));
                }
            };
            let encrypted_refresh_token = match refresh_token {
                Ok(token) => token,
                Err(err) => {
                    eprintln!("error encrypting token {:?}", err);
                    return Err(AppError::Internal(Some(String::from(
                        "Error encrypting refresh token",
                    ))));
                }
            };
            match cloud_account {
                Some(acc) => {
                    let mut cloud: CloudAccountActive = acc.into();
                    let owned_email = email.clone();
                    cloud.access_token = Set(encrypted_access_token);
                    cloud.refresh_token = Set(Some(encrypted_refresh_token));
                    cloud.email = Set(owned_email);
                    cloud.expires_in = Set(Some(expires_in));
                    cloud.is_primary = Set(&email == &final_user_account.gmail);
                    cloud.provider = Set(entities::sea_orm_active_enums::Provider::Google);
                    cloud.user_id = Set(claims.id);
                    cloud.image = Set(image);
                    let account: CloudAccountModel = match cloud.update(db).await {
                        Ok(acc) => acc,
                        Err(err) => return Err(AppError::Internal(Some(err.to_string()))),
                    };
                    account
                }
                None => {
                    let uuidv4 = Uuid::new_v4();
                    let owned_email = email.clone();
                    let owned_sub = sub.clone();
                    let insert_cloud = CloudAccountActive {
                        id: Set(uuidv4),
                        email: Set(owned_email),
                        access_token: Set(encrypted_access_token),
                        refresh_token: Set(Some(encrypted_refresh_token)),
                        expires_in: Set(Some(expires_in)),
                        is_primary: Set(&email == &final_user_account.gmail),
                        provider: Set(entities::sea_orm_active_enums::Provider::Google),
                        user_id: Set(claims.id),
                        sub: Set(Some(owned_sub)),
                        image: Set(image),
                        ..Default::default()
                    };
                    let account: CloudAccountModel = match insert_cloud.insert(db).await {
                        Ok(acc) => acc,
                        Err(_) => {
                            return Err(AppError::Internal(Some(String::from(
                                "Couldn't create cloud account",
                            ))));
                        }
                    };
                    account
                }
            };

            Ok(Redirect::to(&format!(
                "{}/home",
                &ENVS.frontend_url.to_string()
            )))
        }
        None => {
            let (cloud_account, user_account): (
                Result<Option<CloudAccountModel>, DbErr>,
                Result<Option<UserModel>, DbErr>,
            ) = tokio::join!(
                async {
                    CloudAccountEntity::find()
                        .filter(CloudAccountColumn::Sub.eq(&sub))
                        .one(db)
                        .await
                },
                async { UserEntity::find_by_id(claims.id).one(db).await }
            );

            let cloud_account = match cloud_account {
                Ok(con) => con,
                Err(err) => return Err(AppError::Internal(Some(err.to_string()))),
            };
            let user_account = match user_account {
                Ok(con) => con,
                Err(err) => return Err(AppError::Internal(Some(err.to_string()))),
            };

            let final_user_account = match user_account {
                Some(acc) => acc,
                None => {
                    return Err(AppError::Forbidden(Some(String::from(
                        "User account not found",
                    ))));
                }
            };

            match cloud_account {
                Some(acc) => {
                    let mut cloud: CloudAccountActive = acc.into();
                    let owned_email = email.clone();
                    cloud.access_token = Set(encrypted_access_token);
                    cloud.email = Set(owned_email);
                    cloud.expires_in = Set(Some(expires_in));
                    cloud.is_primary = Set(&email == &final_user_account.gmail);
                    cloud.provider = Set(entities::sea_orm_active_enums::Provider::Google);
                    cloud.user_id = Set(claims.id);
                    cloud.image = Set(image);
                    match cloud.update(db).await {
                        Ok(_) => Ok(Redirect::to(&format!(
                            "{}/home",
                            &ENVS.frontend_url.to_string()
                        ))),
                        Err(err) => return Err(AppError::Internal(Some(err.to_string()))),
                    }
                }
                None => {
                    let uuidv4 = Uuid::new_v4();
                    let owned_email = email.clone();
                    let owned_sub = sub.clone();
                    let insert_cloud = CloudAccountActive {
                        id: Set(uuidv4),
                        email: Set(owned_email),
                        access_token: Set(encrypted_access_token),
                        refresh_token: Set(None),
                        expires_in: Set(Some(expires_in)),
                        is_primary: Set(&email == &final_user_account.gmail),
                        provider: Set(entities::sea_orm_active_enums::Provider::Google),
                        user_id: Set(claims.id),
                        sub: Set(Some(owned_sub)),
                        image: Set(image),
                        ..Default::default()
                    };
                    match insert_cloud.insert(db).await {
                        Ok(_) => Ok(Redirect::to(&format!(
                            "{}/home",
                            &ENVS.frontend_url.to_string()
                        ))),
                        Err(_) => {
                            return Err(AppError::Internal(Some(String::from(
                                "Couldn't create cloud account",
                            ))));
                        }
                    }
                }
            }
        }
    }
}
