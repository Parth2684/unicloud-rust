use chrono::Utc;
use common::{
    encrypt::{decrypt, encrypt},
    export_envs::ENVS,
};
use entities::{
    cloud_account::{
        ActiveModel as CloudAccountActive, Column as CloudAccountColumn,
        Entity as CloudAccountEntity,
    },
    sea_orm_active_enums::Provider,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct GoogleResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
    pub scope: String,
    pub token_type: String,
}

#[derive(Deserialize, Debug)]
pub struct GoogleError {
    pub error: String,
    pub error_description: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)] // <-- IMPORTANT
pub enum GoogleResult {
    Ok(GoogleResponse),
    Err(GoogleError),
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
            return true;
        }
    };
    if cloud_accs.is_empty() {
        return false;
    }
    let time = Utc::now();
    let time = time.timestamp() - 300;

    for acc in cloud_accs {
        if acc.provider == Provider::Google && acc.expires_in < Some(time) && !acc.token_expired {
            let encrypt_refresh = match &acc.refresh_token {
                Some(token) => token,
                None => continue,
            };
            let refresh_token = match decrypt(encrypt_refresh) {
                Ok(token) => token,
                Err(err) => {
                    eprintln!("{err:?}");
                    continue;
                }
            };

            let client = reqwest::Client::new();
            let res: Result<reqwest::Response, reqwest::Error> = client
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
                    let json: Result<GoogleResult, reqwest::Error> = data.json().await;
                    let final_json = match json {
                        Ok(json) => match json {
                            GoogleResult::Ok(res) => res,
                            GoogleResult::Err(err) => {
                                eprintln!("{err:?}");
                                if err.error == "invalid_grant"
                                    && err.error_description == "Token has been expired or revoked."
                                {
                                    let mut acc: CloudAccountActive = acc.into();
                                    acc.token_expired = Set(true);
                                    let _ = acc.update(db).await.ok();
                                }
                                continue;
                            }
                        },
                        Err(err) => {
                            eprintln!("error parsing {err:?}");
                            continue;
                        }
                    };
                    let mut acc: CloudAccountActive = acc.into();
                    let access_token = final_json.access_token;
                    let encrypted_token = match encrypt(&access_token) {
                        Ok(token) => token,
                        Err(err) => {
                            eprintln!("error encoding token {err}");
                            continue;
                        }
                    };
                    match final_json.refresh_token {
                        Some(tok) => match encrypt(&tok) {
                            Ok(refreshed) => {
                                let current_time = Utc::now().naive_utc();
                                acc.access_token = Set(encrypted_token);
                                acc.updated_at = Set(Some(current_time));
                                acc.token_expired = Set(false);
                                acc.expires_in =
                                    Set(Some(final_json.expires_in + Utc::now().timestamp()));
                                acc.refresh_token = Set(Some(refreshed));
                                match acc.update(db).await {
                                    Ok(_) => (),
                                    Err(err) => {
                                        eprintln!("error updating db {err:?}");
                                        should_retry = true;
                                        continue;
                                    }
                                }
                            }
                            Err(err) => {
                                eprintln!("error encrypting refresh token {err}");
                                should_retry = true;
                                continue;
                            }
                        },
                        None => {
                            let current_time = Utc::now().naive_utc();
                            acc.access_token = Set(encrypted_token);
                            acc.updated_at = Set(Some(current_time));
                            acc.token_expired = Set(false);
                            acc.expires_in =
                                Set(Some(final_json.expires_in + Utc::now().timestamp()));
                            match acc.update(db).await {
                                Ok(_) => (),
                                Err(err) => {
                                    eprintln!("line 147 {err:?}");
                                    should_retry = true;
                                    continue;
                                }
                            }
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
