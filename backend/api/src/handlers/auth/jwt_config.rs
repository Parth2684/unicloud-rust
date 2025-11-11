use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::export_envs::ENVS;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub quota_type: String,
    pub exp: i64,
}

pub fn create_jwt(sub: &str, quota_type: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let timestamp = Utc::now() + Duration::days(7);
    let expiration = timestamp.timestamp();
    let claims = Claims {
        sub: sub.to_owned(),
        quota_type: quota_type.to_owned(),
        exp: expiration,
    };

    let encoding_key = EncodingKey::from_secret(&ENVS.jwt_secret.as_bytes());
    let token = encode(&Header::default(), &claims, &encoding_key);
    match token {
        Ok(token) => Ok(token),
        Err(err) => {
            eprintln!("{err:?}");
            Err(err)
        }
    }
}

pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(&ENVS.jwt_secret.as_bytes());
    let data = decode(token, &decoding_key, &Validation::default());
    let result: Result<Claims, jsonwebtoken::errors::Error> = match data {
        Ok(data) => Ok(data.claims),
        Err(err) => {
            eprintln!("{err:?}");
            Err(err)
        }
    };
    result
}
