use axum::{
    Extension, Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use common::{db_connect::init_db, jwt_config::Claims};
use entities::{
    cloud_account::{Column as CloudAccountColumn, Entity as CloudAccountEntity},
    sea_orm_active_enums::Provider,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use serde_json::json;

use crate::utils::app_errors::AppError;

pub async fn get_cloud_accounts(
    Extension(claims): Extension<Claims>,
) -> Result<Response, AppError> {
    let db = init_db().await;
    let cloud_accounts = CloudAccountEntity::find()
        .filter(CloudAccountColumn::UserId.eq(claims.id))
        .filter(CloudAccountColumn::Provider.eq(Provider::Google))
        .select_only()
        .columns([CloudAccountColumn::Id, CloudAccountColumn::Email, CloudAccountColumn::Provider, CloudAccountColumn::TokenExpired, CloudAccountColumn::Image])
        .all(db)
        .await;

    let cloud_accounts = match cloud_accounts {
        Err(err) => {
            eprintln!("{err:?}");
            return Err(AppError::Internal(Some(String::from(
                "Error Connecting to database",
            ))));
        }
        Ok(acc) => acc,
    };

    let mut need_refresh = Vec::new();
    let mut google_drive_accounts = Vec::new();
    cloud_accounts.iter().for_each(|acc| {
        if acc.token_expired == true {
            need_refresh.push(acc)
        } else {
            google_drive_accounts.push(acc)
        }
    });

    Ok((
        StatusCode::OK,
        Json(json!({
            "need_refresh": need_refresh,
            "google_drive_accounts": google_drive_accounts
        })),
    )
        .into_response())
}
