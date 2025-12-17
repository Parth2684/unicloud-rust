use axum::{
    Extension, Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::Utc;
use common::{db_connect::init_db, encrypt::decrypt, jwt_config::Claims};
use entities::{
    cloud_account::{
        ActiveModel as CloudAccountActive, Column as CloudAccountColumn,
        Entity as CloudAccountEntity,
    },
    sea_orm_active_enums::Provider,
};
use futures::future::join_all;
use reqwest::Client;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::utils::app_errors::AppError;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct StorageQuota {
    limit: Option<String>,
    usage_in_drive: String,
    usage_in_drive_trash: String,
    usage: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GoogleApiRes {
    storage_quota: StorageQuota,
}
#[derive(Debug, Serialize)]
struct ReturnErrorCloud {
    id: Uuid,
    email: String,
    provider: Provider,
    token_expired: bool,
    image: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReturnSuccessCloud {
    storage_quota: StorageQuota,
    info: ReturnErrorCloud,
}

pub async fn get_cloud_accounts(
    Extension(claims): Extension<Claims>,
) -> Result<Response, AppError> {
    let db = init_db().await;
    let google_accounts = CloudAccountEntity::find()
        .filter(CloudAccountColumn::UserId.eq(claims.id))
        .filter(CloudAccountColumn::Provider.eq(Provider::Google))
        .all(db)
        .await;

    let google_accounts = match google_accounts {
        Err(err) => {
            eprintln!("{err:?}");
            return Err(AppError::Internal(Some(String::from(
                "Error Connecting to database",
            ))));
        }
        Ok(acc) => acc,
    };

    let mut need_refresh: Vec<ReturnErrorCloud> = Vec::new();
    let mut google_drive_accounts: Vec<ReturnSuccessCloud> = Vec::new();

    let future_tasks = google_accounts.into_iter().map(|acc| async {
        match decrypt(&acc.access_token) {
            Ok(token) => match get_info(&token).await {
                Ok(storage_quota) => Ok(ReturnSuccessCloud {
                    storage_quota,
                    info: ReturnErrorCloud {
                        id: acc.id,
                        email: acc.email,
                        provider: Provider::Google,
                        token_expired: false,
                        image: acc.image,
                    },
                }),
                Err(_) => {
                    let mut cloud: CloudAccountActive = acc.clone().into();
                    cloud.token_expired = Set(true);
                    cloud.updated_at = Set(Some(Utc::now().naive_utc()));
                    let _ = cloud.update(db).await;

                    Err(ReturnErrorCloud {
                        id: acc.id,
                        email: acc.email,
                        provider: Provider::Google,
                        token_expired: true,
                        image: acc.image,
                    })
                }
            },
            Err(_) => {
                let mut cloud: CloudAccountActive = acc.clone().into();
                cloud.token_expired = Set(true);
                cloud.updated_at = Set(Some(Utc::now().naive_utc()));
                let _ = cloud.update(db).await;

                Err(ReturnErrorCloud {
                    id: acc.id,
                    email: acc.email,
                    provider: Provider::Google,
                    token_expired: true,
                    image: acc.image,
                })
            }
        }
    });

    let results = join_all(future_tasks).await;

    for res in results {
        match res {
            Ok(acc) => google_drive_accounts.push(acc),
            Err(err_acc) => need_refresh.push(err_acc),
        }
    }
    Ok((
        StatusCode::OK,
        Json(json!({
            "need_refresh": need_refresh,
            "google_drive_accounts": google_drive_accounts
        })),
    )
        .into_response())
}

async fn get_info(token: &str) -> Result<StorageQuota, reqwest::Error> {
    let res = Client::new()
        .get("https://www.googleapis.com/drive/v3/about?fields=storageQuota")
        .bearer_auth(token)
        .send()
        .await?
        .json::<GoogleApiRes>()
        .await?;

    Ok(res.storage_quota)
}
