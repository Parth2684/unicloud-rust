use axum::{
    Json,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse},
};
use common::{db_connect::init_db, encrypt::decrypt};
use entities::{
    cloud_account::{self, Column as CloudAccountColumn, Entity as CloudAccountEntity},
    sea_orm_active_enums::Provider,
};
use reqwest::Client;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;
use uuid::Uuid;

use crate::utils::{app_errors::AppError, google_search_folder::google_search_folder};

pub async fn google_get_root(Path(drive_id): Path<String>) -> Result<impl IntoResponse, AppError> {
    let db = init_db().await;
    let drive_id = match Uuid::parse_str(&drive_id) {
        Ok(id) => id,
        Err(err) => {
            eprintln!("{err}");
            return Err(AppError::UnprocessableEntry(Some(String::from(
                "Error during parsing cloud id",
            ))));
        }
    };
    let google_account = CloudAccountEntity::find()
        .filter(CloudAccountColumn::Id.eq(drive_id))
        .filter(CloudAccountColumn::Provider.eq(Provider::Google))
        .one(db)
        .await;

    match google_account {
        Err(err) => {
            eprintln!("{err:?}");
            return Err(AppError::Internal(Some(String::from(
                "Error connecting to database",
            ))));
        }
        Ok(some) => match some {
            None => {
                return Err(AppError::NotFound(Some(String::from(
                    "No Account with the id found",
                ))));
            }
            Some(acc) => match decrypt(&acc.access_token) {
                Err(err) => {
                    eprintln!("{err:?}");
                    let mut acc: cloud_account::ActiveModel = acc.into();
                    acc.token_expired = Set(true);
                    acc.update(db).await.ok();
                    return Err(AppError::Unauthorised(Some(String::from(
                        "Error decrypting token please add the account again",
                    ))));
                }
                Ok(token) => {
                    let client = Client::new();
                    match client
                        .get("https://www.googleapis.com/drive/v3/about?fields=rootFolderId")
                        .bearer_auth(&token)
                        .send()
                        .await
                    {
                        Err(err) => {
                            eprintln!("{err:?}");
                            return Err(AppError::Internal(Some(
                                "Couldn't fetch from google".to_string(),
                            )));
                        }
                        Ok(res) => {
                            let res = res.json::<serde_json::Value>().await.unwrap();
                            match res.get("rootFolderId") {
                                None => {
                                    return Err(AppError::NotFound(Some(String::from(
                                        "Couldn't find the drive root",
                                    ))));
                                }
                                Some(id) => {
                                    let drive_id = id.as_str().unwrap().to_owned();
                                    let files = google_search_folder(&drive_id, &token).await;
                                    match files {
                                        Err(err) => {
                                            eprintln!("{err:?}");
                                            return Err(AppError::Internal(Some(String::from(
                                                "Couldn't find root files of the google drive",
                                            ))));
                                        }
                                        Ok(files) => {
                                            return Ok((
                                                StatusCode::OK,
                                                Json(json!({
                                                    "files": files
                                                })),
                                            )
                                                .into_response());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
        },
    }
}
