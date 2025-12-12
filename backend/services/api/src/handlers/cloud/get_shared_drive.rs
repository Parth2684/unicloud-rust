use axum::{Extension, Json, extract::Path, http::StatusCode, response::{IntoResponse, Response}};
use common::{db_connect::init_db, encrypt::decrypt, jwt_config::Claims};
use entities::cloud_account::{Entity as CloudAccountEntity, Column as CloudAccountCloumn};
use reqwest::Client;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::utils::app_errors::AppError;

#[derive(Serialize, Deserialize)]
struct Drive {
    id: String,
    name: String
}

#[derive(Serialize, Deserialize)]
struct ApiRes {
    drives: Vec<Drive>
}


pub async fn get_shared_drives (Extension(claims): Extension<Claims>, Path(drive_id): Path<Uuid>) -> Result<Response, AppError> {
    let db = init_db().await;
    let cloud_account = CloudAccountEntity::find()
        .filter(CloudAccountCloumn::UserId.eq(claims.id))
        .filter(CloudAccountCloumn::Id.eq(drive_id))
        .one(db)
        .await;
    
    match cloud_account {
        Ok(acc) => {
            match acc {
                None => Err(AppError::Forbidden(Some(String::from("No such account found")))),
                Some(acc) => {
                    match decrypt(&acc.access_token) {
                        Err(err) => {
                            eprintln!("{err:?}");
                            return Err(AppError::Forbidden(Some(String::from("Couldn't decrypt the token try refreshing or re adding the account"))))
                        }
                        Ok(token) => {
                            let res = Client::new()
                                .get("https://www.googleapis.com/drive/v3/drives?supportsAllDrives=true&fields=drives(id,name),nextPageToken")
                                .bearer_auth(token)
                                .send()
                                .await;
                            match res {
                                Ok(response) => {
                                    let fmt_res = response.json::<ApiRes>().await;
                                    match fmt_res {
                                        Ok(correct) => {
                                            Ok((
                                                StatusCode::OK,
                                                Json(json!({
                                                    "drives": correct.drives
                                                }))
                                                ).into_response())
                                        }
                                        Err(err) => {
                                            eprintln!("{err:?}");
                                            Err(AppError::Internal(None))
                                        }
                                    }
                                }
                                Err(err) => {
                                    eprintln!("{err:?}");
                                    Err(AppError::Internal(Some(String::from("Error parsing response form the google api"))))
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("{err:?}");
            return Err(AppError::Internal(Some(String::from("Couldn't cionnect to database"))))
        }
    }
}

// pub async fn get_shared_files()