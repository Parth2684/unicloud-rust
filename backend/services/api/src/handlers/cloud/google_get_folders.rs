use axum::{Extension, Json, extract::Path, http::StatusCode, response::IntoResponse};
use common::{db_connect::init_db, encrypt::decrypt, jwt_config::Claims};
use entities::{
    cloud_account::{self, Column as CloudAccountColumn, Entity as CloudAccountEntity},
    sea_orm_active_enums::Provider,
};
use reqwest::Client;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::utils::{app_errors::AppError, google_search_folder::google_search_folder};

pub async fn google_get_root(
    Path(drive_id): Path<String>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
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
        .filter(CloudAccountColumn::UserId.eq(claims.id))
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

#[derive(Deserialize)]
pub struct GoogleParams {
    drive_id: String,
    folder_id: String,
}
pub async fn google_get_folders(
    Path(params): Path<GoogleParams>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    let db = init_db().await;
    let google_account = CloudAccountEntity::find()
        .filter(CloudAccountColumn::Id.eq(params.drive_id))
        .filter(CloudAccountColumn::Provider.eq(Provider::Google))
        .filter(CloudAccountColumn::UserId.eq(claims.id))
        .one(db)
        .await;

    match google_account {
        Err(err) => {
            eprintln!("{err:?}");
            return Err(AppError::Internal(Some(
                "Error connecting to database".to_string(),
            )));
        }
        Ok(some_acc) => match some_acc {
            Some(acc) => match decrypt(&acc.access_token) {
                Ok(token) => {
                    let res = google_search_folder(&params.folder_id, &token).await;
                    match res {
                        Err(err) => {
                            eprintln!("{err:?}");
                            return Err(AppError::Internal(Some(
                                "error fetching files under this folder".to_string(),
                            )));
                        }
                        Ok(files) => Ok((
                            StatusCode::OK,
                            Json(json!({
                                "files": files
                            })),
                        )
                            .into_response()),
                    }
                }
                Err(err) => {
                    eprintln!("{err:?}");
                    let mut acc: cloud_account::ActiveModel = acc.into();
                    acc.token_expired = Set(true);
                    acc.update(db).await.ok();
                    return Err(AppError::Unauthorised(Some(String::from(
                        "Error decrypting token please add the account again",
                    ))));
                }
            },
            None => Err(AppError::NotFound(Some(String::from(
                "Could not find such account",
            )))),
        },
    }
}
