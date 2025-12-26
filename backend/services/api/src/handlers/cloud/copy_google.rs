use std::str::FromStr;

use axum::{
    Extension,
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use common::{
    db_connect::init_db, encrypt::decrypt, jwt_config::Claims, redis_connection::init_redis,
};
use entities::job::ActiveModel as JobActive;
use entities::{
    cloud_account::{
        ActiveModel as CloudAccountActive, Column as CloudAccountColumn,
        Entity as CloudAccountEntity,
    },
    sea_orm_active_enums::Status,
};
use redis::AsyncTypedCommands;
use reqwest::Client;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::utils::app_errors::AppError;

#[derive(Deserialize)]
pub struct CopyInputs {
    from_drive: String,
    from_file_id: String,
    to_drive: String,
    to_folder_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDetail {
    mime_type: String,
    size: Option<String>,
}

pub async fn copy_file_or_folder(
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CopyInputs>,
) -> Result<Response, AppError> {
    let (mut redis_conn, db) = tokio::join!(init_redis(), init_db());

    let from_uuid = Uuid::from_str(&payload.from_drive);
    let to_uuid = Uuid::from_str(&payload.to_drive);

    match (from_uuid, to_uuid) {
        (Err(err1), Err(err2)) => {
            eprintln!("err1: {:?}, err2: {:?}", err1, err2);
            return Err(AppError::Internal(Some(String::from(
                "Couldn't parse Ids of any drive",
            ))));
        }
        (Err(err), Ok(_)) => {
            eprintln!("err parsing from drive id: {:?}", err);
            return Err(AppError::Internal(Some(String::from(
                "Couldn't parse Id of from drive",
            ))));
        }
        (Ok(_), Err(err)) => {
            eprintln!("error parsing to drive id {:?}", err);
            return Err(AppError::Internal(Some(String::from(
                "Couldn't parse Id of to drive",
            ))));
        }
        (Ok(from_id), Ok(to_id)) => {
            let (from_acc, to_acc) = tokio::join!(
                CloudAccountEntity::find()
                    .filter(CloudAccountColumn::Id.eq(from_id))
                    .filter(CloudAccountColumn::UserId.eq(claims.id))
                    .one(db),
                CloudAccountEntity::find()
                    .filter(CloudAccountColumn::Id.eq(to_id))
                    .filter(CloudAccountColumn::UserId.eq(claims.id))
                    .one(db)
            );

            match (from_acc, to_acc) {
                (Err(err1), Err(err2)) => {
                    eprintln!("error fetching accounts: {:?}, {:?}", err1, err2);
                    return Err(AppError::Internal(Some(String::from(
                        "Error fetching both the transfer accounts",
                    ))));
                }
                (Err(err), Ok(_)) => {
                    eprintln!("error fetching source account: {err:?}");
                    return Err(AppError::Internal(Some(String::from(
                        "Error fetching source account",
                    ))));
                }
                (Ok(_), Err(err)) => {
                    eprintln!("error fetching destination account: {err:?}");
                    return Err(AppError::Internal(Some(String::from(
                        "Error fetching destination account",
                    ))));
                }
                (Ok(some_from_acc), Ok(some_to_acc)) => match (some_from_acc, some_to_acc) {
                    (None, None) => {
                        eprintln!("Neither accounts found under the user id ");
                        return Err(AppError::NotFound(Some(String::from(
                            "Both accounts were not found under your access",
                        ))));
                    }
                    (None, Some(_)) => {
                        eprintln!("Source account not found in db");
                        return Err(AppError::NotFound(Some(String::from(
                            "Source account not found under your access",
                        ))));
                    }
                    (Some(_), None) => {
                        eprintln!("Destination account not found in db");
                        return Err(AppError::NotFound(Some(String::from(
                            "Destination account not found under your access",
                        ))));
                    }
                    (Some(source_acc), Some(destination_acc)) => {
                        if source_acc.token_expired == true {
                            return Err(AppError::UnprocessableEntry(Some(String::from(format!(
                                "Your Source account {:?} needs to be refreshed to perform this action",
                                source_acc.email
                            )))));
                        }
                        if destination_acc.token_expired == true {
                            return Err(AppError::UnprocessableEntry(Some(String::from(format!(
                                "Your destination account {:?} needs to be refreshed to perfrom this action",
                                destination_acc.email
                            )))));
                        }
                        match decrypt(&source_acc.access_token) {
                            Err(err) => {
                                eprintln!("Error decrypting access token: {:?}", err);
                                let mut source_active: CloudAccountActive = source_acc.into();
                                source_active.token_expired = Set(true);
                                source_active.update(db).await.ok();
                                return Err(AppError::Internal(Some(String::from(
                                    "Error decrypting access token please refresh your account",
                                ))));
                            }
                            Ok(token) => {
                                let client = Client::new();
                                let res = client
                                    .get(format!("https://www.googleapis.com/drive/v3/files?q='{}' in parents and trashed=false&fields=mimeType,size&supportsAllDrives=true&includeItemsFromAllDrives=true&spaces=drive", &payload.from_file_id))
                                    .bearer_auth(token)
                                    .send()
                                    .await;

                                match res {
                                    Err(err) => {
                                        eprintln!(
                                            "error receiving response from derive api: {:?}",
                                            err
                                        );
                                        return Err(AppError::BadGateway(Some(String::from(
                                            "Error receiving details of files from google",
                                        ))));
                                    }
                                    Ok(response) => match response.json::<FileDetail>().await {
                                        Err(err) => {
                                            eprintln!(
                                                "error parsing response from google: {:?}",
                                                err
                                            );
                                            return Err(AppError::BadGateway(Some(String::from(
                                                "Error Parsing response from google",
                                            ))));
                                        }
                                        Ok(details) => {
                                            let is_folder = details.mime_type
                                                == String::from(
                                                    "application/vnd.google-apps.folder",
                                                );
                                            let size_i64: Option<i64> = details
                                                .size
                                                .as_deref()
                                                .and_then(|s| s.parse::<i64>().ok());

                                            let id = Uuid::new_v4();

                                            let insert_job = JobActive::insert(
                                                JobActive {
                                                    id: Set(id),
                                                    from_drive: Set(source_acc.id),
                                                    from_file_id: Set(payload.from_file_id.clone()),
                                                    is_folder: Set(is_folder),
                                                    to_drive: Set(destination_acc.id),
                                                    to_folder_id: Set(payload.to_folder_id.clone()),
                                                    user_id: Set(claims.id),
                                                    size: Set(size_i64),
                                                    ..Default::default()
                                                },
                                                db,
                                            )
                                            .await;

                                            match insert_job {
                                                Err(err) => {
                                                    eprintln!("error creating task: {err:?}");
                                                    return Err(AppError::Internal(Some(
                                                        String::from("Error creating task"),
                                                    )));
                                                }
                                                Ok(job) => {
                                                    match redis_conn
                                                        .lpush(
                                                            "copy-google:job",
                                                            job.id.to_string(),
                                                        )
                                                        .await
                                                    {
                                                        Err(err) => {
                                                            eprintln!(
                                                                "error pushing to redis queue: {err:?}"
                                                            );
                                                            let mut edit_job: JobActive =
                                                                job.into();
                                                            edit_job.status = Set(Status::Failed);
                                                            edit_job.update(db).await.ok();
                                                            return Err(AppError::Internal(Some(
                                                                "Error pushing to job queue".into(),
                                                            )));
                                                        }
                                                        Ok(_) => {
                                                            return Ok((
                                                                    StatusCode::OK,
                                                                    axum::Json(json!({
                                                                        "message": "Task added successfully",
                                                                    })),
                                                                )
                                                                    .into_response());
                                                        }
                                                    };
                                                }
                                            }
                                        }
                                    },
                                }
                            }
                        };
                    }
                },
            }
        }
    }
}
