use chrono::Utc;
use common::{encrypt::decrypt, export_envs::ENVS};
use reqwest::StatusCode;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use uuid::Uuid;
use entities::{cloud_account::{ Column as CloudAccountColumn, Entity as CloudAccountEntity }, sea_orm_active_enums::Provider};



pub async fn handle_refresh(id: Uuid, db: &DatabaseConnection) -> Result<(), DbErr>{
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
                Err(err) => return
            };
        
            let client = reqwest::Client::new();
            let res = client
                .post(String::from("https://oauth2.googleapis.com/token"))
                .form(&[
                    "client_id", &ENVS.google_drive_client_id.as_str(),
                    "client_secret", &ENVS.google_client_secret.as_str(),
                    "refresh_token", refresh_token.as_str(),
                    "grant_type", "refresh_token"
                ])
                .send()
                .await;
            
            match res {
                Ok(data) => {
                    
                },
                Err(err) => {
                    if !err.status() != Some(StatusCode::ACCEPTED) {
                        
                    }
                }
            }
                
        }   
    }
}