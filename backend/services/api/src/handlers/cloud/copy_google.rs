use std::str::FromStr;

use axum::{
    Extension,
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use common::{db_connect::init_db, jwt_config::Claims, redis_connection::init_redis};
use entities::cloud_account::{Column as CloudAccountColumn, Entity as CloudAccountEntity};
use redis::AsyncTypedCommands;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::utils::app_errors::AppError;

#[derive(Deserialize)]
struct CopyInputs {
    from_drive: String,
    from_file_id: String,
    is_folder: bool,
    to_drive: String,
    to_folder_id: String,
}

pub async fn copy_file_or_folder(
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CopyInputs>,
) -> Result<Response, AppError> {
    let (redis_conn, db) = tokio::join!(init_redis(), init_db(),);

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
                    }
                },
            }
        }
    }

    Ok((
        StatusCode::OK,
        axum::Json(json!({
            "message": "Task added successfully",

        })),
    )
        .into_response())
}
