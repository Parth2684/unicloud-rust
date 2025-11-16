use chrono::{Duration, Utc};
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::ErrorKind,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::export_envs::ENVS;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: Uuid,
    pub quota_type: String,
    pub exp: i64,
}

pub fn create_jwt(id: &str, quota_type: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let timestamp = Utc::now() + Duration::days(7);
    let expiration = timestamp.timestamp();
    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(_err) => return Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken)),
    };
    let claims = Claims {
        id: uuid,
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
    let token: String = token.chars().filter(|c| !c.is_whitespace()).collect();

    let decoding_key = DecodingKey::from_secret(ENVS.jwt_secret.as_bytes());

    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.validate_exp = true;

    let data = decode::<Claims>(&token, &decoding_key, &validation)?;

    Ok(data.claims)
}
