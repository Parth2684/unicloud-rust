use std::str::FromStr;

use axum::{
    Extension,
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use common::{jwt_config::Claims, redis_connection::init_redis};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::utils::app_errors::AppError;

#[derive(Deserialize)]
struct CopyInputs {
    from_drive: String,
    from_file_id: String,
    mime_type: String,
    to_drive: String,
    to_folder_id: String,
}

pub async fn copy_file_or_folder(
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CopyInputs>,
) -> Result<Response, AppError> {
    let redis_conn = init_redis().await;
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
        (Ok(from_id), Ok(to_id)) => {}
    }

    Ok((
        StatusCode::OK,
        axum::Json(json!({
            "message": "Task added successfully",

        })),
    )
        .into_response())
}
