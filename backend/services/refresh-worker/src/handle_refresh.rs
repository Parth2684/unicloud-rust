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
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait};
use serde::Deserialize;
use uuid::Uuid;


#[derive(Deserialize)]
struct GoogleResponse {
    access_token: String,
    expires_in: i64,
    refresh_token: Option<String>,
}
#[derive(Debug, Deserialize)]
struct GoogleError {
    error: String,
    error_description: String
}

pub async fn handle_refresh(id: Uuid, db: &DatabaseConnection) -> bool {
    let mut should_retry = false;
    let cloud_accs = CloudAccountEntity::find()
        .filter(CloudAccountColumn::UserId.eq(id))
        .all(db)
        .await;
    let cloud_accs = match cloud_accs {
        Ok(accs) => accs,
        Err(err) => {
            eprintln!("{err:?}");
            should_retry = true;
            return true;
        }
    };

    if cloud_accs.is_empty() {
        return false;
    }
    let time = Utc::now();
    let time = time.timestamp() - 300;

    for acc in cloud_accs {
        if acc.provider == Provider::Google && acc.expires_in < Some(time) {
            let encrypt_refresh = match &acc.refresh_token {
                Some(token) => token,
                None => continue
            };
            let refresh_token = match decrypt(encrypt_refresh) {
                Ok(token) => token,
                Err(err) => {
                    eprintln!("{err:?}");
                    continue;
                }
            };

            let client = reqwest::Client::new();
            let res = client
                .post(String::from("https://oauth2.googleapis.com/token"))
                .form(&[
                    ("client_id", &ENVS.google_drive_client_id.as_str()),
                    ("client_secret", &ENVS.google_drive_client_secret.as_str()),
                    ("refresh_token", &refresh_token.as_str()),
                    ("grant_type", &"refresh_token"),
                ])
                .send()
                .await;

            match res {
                Ok(data) => {
                    let json: Result<Result<GoogleResponse, GoogleError>, reqwest::Error> = data.json().await;
                    let final_json = match json {
                        Ok(json) => match json {
                            Ok(res) => res,
                            Err(err) => {
                                if err.error == "invalid_grant" && err.error_description == "Token has been expired or revoked." {
                                    let mut acc: CloudAccountActive = acc.into();
                                    acc.token_expired = Set(true);
                                    let _ = acc.update(db).await.ok();
                                }
                                continue;
                            }
                        },
                        Err(err) => {
                            eprintln!("{err:?}");
                            continue;
                        }
                    } ;
                    let mut acc: CloudAccountActive = acc.into();
                    let access_token = final_json.access_token;
                    let encrypted_token = match encrypt(&access_token) {
                        Ok(token) => token,
                        Err(err) => {
                            eprintln!("{err}");
                            continue;
                        }
                    };
                    let current_time = Utc::now().naive_utc();
                    acc.access_token = Set(encrypted_token);
                    acc.updated_at = Set(Some(current_time));
                    acc.token_expired = Set(false);
                    match acc.update(db).await {
                        Ok(_) => (),
                        Err(err) => {
                            eprintln!("{err:?}");
                            should_retry = true;
                            continue;
                        }
                    }
                }
                Err(err) => {
                    eprintln!("request failure: {err:?}");
                    continue;
                }
            }
        }
    }
    should_retry
}
