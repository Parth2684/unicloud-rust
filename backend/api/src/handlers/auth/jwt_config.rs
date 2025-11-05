use chrono::{Duration, Offset, Utc};
use jsonwebtoken::{Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::export_envs::ENVS;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub quota_type: String,
    pub exp: usize,
}

pub fn create_jwt(sub: &str, quota_type: &str) -> String {
    let expiration = (Utc::now() + Duration::days(7)).unix_timestamp() as usize;
    let claims = Claims {
        sub: sub.to_owned(),
        quota_type: quota_type.to_owned(),
        exp: expiration,
    };

    let token = encode(&Header::default(), claims, &ENVS.jwt_secret);
    match token {
        Ok(token) => token,
        Err(err) => {
            eprintln!("{err:?}");
            return;
        }
    }
}

pub fn decode_jwt(token: &str) -> Option<Claims> {
    let data = decode(token, &ENVS.jwt_secret, &Validation::default());
    let token = match data {
        Ok(data) => data,
        Err(err) => {
            eprintln!("{err:?}");
            return;
        }
    };
}
