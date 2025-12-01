use axum::{Extension, Json, http::StatusCode, response::{IntoResponse, Response}};
use common::{db_connect::init_db, jwt_config::Claims, encrypt::decrypt};
use entities::{cloud_account::{Column as CloudAccountColumn, Entity as CloudAccountEntity}, sea_orm_active_enums::Provider};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
use serde_json::json;

use crate::utils::app_errors::AppError;

pub async fn google_drive_file_structure(
    Extension(claims): Extension<Claims>
) -> Result<Response, AppError> {
    let db = init_db().await;
    let cloud_accounts = CloudAccountEntity::find()
        .filter(CloudAccountColumn::UserId.eq(claims.id))
        .filter(CloudAccountColumn::Provider.eq(Provider::Google))
        .all(db)
        .await;
    
    match cloud_accounts {
        Ok(accs) => {
            if accs.len() == 0 {
                return Ok((StatusCode::OK, Json(json!({
                    "message": "No cloud accounts linked",
                    "cloud_accounts": []
                }))).into_response())
            }
            else {
                for acc in accs {
                    let access_token = decrypt(&acc.access_token);
                    
                }
                return Ok((StatusCode::OK, Json(json!({
                    "message": "Fetching Successfull"
                }))).into_response())
            }
        },
        Err(_) => {
            return Err(AppError::Internal(Some(String::from("Error connecting to db"))))
        }
    };
}