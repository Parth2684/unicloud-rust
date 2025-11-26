use chrono::Utc;
use common::{
    encrypt::{decrypt, encrypt},
    export_envs::ENVS,
};
use entities::{
    cloud_account::{
        ActiveModel as CloudAccountActive, Column as CloudAccountColumn,
        Entity as CloudAccountEntity, Model,
    },
    sea_orm_active_enums::Provider,
};
use reqwest::StatusCode;
use sea_orm::{ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct GoogleError {
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GoogleResponse {
    access_token: String,
    expires_in: i64,
    refresh_token: Option<String>,
}

pub async fn handle_refresh(id: Uuid, db: &DatabaseConnection) -> Result<(), DbErr> {
    let cloud_accs = CloudAccountEntity::find()
        .filter(CloudAccountColumn::UserId.eq(id))
        .all(db)
        .await;
    let cloud_accs = match cloud_accs {
        Ok(accs) => accs,
        Err(err) => {
            eprintln!("{err:?}");
            return Err(err);
        }
    };

    if cloud_accs.is_empty() {
        Ok(())
    }
    let time = Utc::now();
    let time = time.timestamp() - 300;

    for acc in cloud_accs {
        if acc.provider == Provider::Google && acc.expires_in < Some(time) {
            let refresh_token = match decrypt(&acc.refresh_token) {
                Ok(token) => token,
                Err(err) => return,
            };

            let client = reqwest::Client::new();
            let res = client
                .post(String::from("https://oauth2.googleapis.com/token"))
                .form(&[
                    "client_id",
                    &ENVS.google_drive_client_id.as_str(),
                    "client_secret",
                    &ENVS.google_client_secret.as_str(),
                    "refresh_token",
                    refresh_token.as_str(),
                    "grant_type",
                    "refresh_token",
                ])
                .send()
                .await;

            match res {
                Ok(data) => {
                    let json: Result<GoogleResponse, reqwest::Error> = data.json().await;
                    let mut acc: CloudAccountActive = acc.unwrap().into();
                    let encrypted_token = match encrypt(&json.access_token) {
                        Ok(token) => token,
                        Err(err) => {
                            eprintln!("{err}");
                            return;
                        }
                    };
                    acc.access_token = Set(encrypted_token);
                    acc.updated_at = Set(Utc::now());
                    match acc.update(db).await {
                        Ok(_) => return Ok(()),
                        Err(err) => {
                            eprintln!("{err:?}");
                            return;
                        }
                    }
                }
                Err(err) => {
                    if !err.status() != Some(StatusCode::ACCEPTED) {
                        let err_data: GoogleError = res.json().await.unwrap_or(GoogleError {
                            error: Some("unknown".into()),
                            error_description: None,
                        });
                    }
                    if let Some(err) = err_data.error {
                        if err == "invalid_grant" {
                            let mut acc: CloudAccountActive = acc.unwrap().into();
                            acc.token_expired = Set(true);
                            let _: Model = acc.update(db).await?;
                        }
                    }
                }
                
            }
        }
    }
}
