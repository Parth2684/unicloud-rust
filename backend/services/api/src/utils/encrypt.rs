use aes_gcm::{
    AeadCore, Aes256Gcm, Key, KeyInit,
    aead::{AeadMut, OsRng},
};

use crate::utils::{app_errors::AppError, export_envs::ENVS};

pub fn encrypt(plaintext: &str) -> Result<Vec<u8>, AppError> {
    let key: &Key<Aes256Gcm> = &ENVS.encryption_key.try_into().unwrap();
    let mut cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher_text = match cipher.encrypt(&nonce, plaintext.as_ref()) {
        Ok(ciphertext) => ciphertext,
        Err(err) => {
            return Err(AppError::Internal(Some(err.to_string())));
        }
    };
    let mut concat_nonce_cipher = Vec::new();
    concat_nonce_cipher.extend_from_slice(&nonce);
    concat_nonce_cipher.extend_from_slice(&cipher_text);
    Ok(concat_nonce_cipher)
}

pub fn decrypt(data: Vec<u8>) -> Result<String, AppError> {
    let key: &Key<Aes256Gcm> = &ENVS.encryption_key.try_into().unwrap();
    let mut cipher = Aes256Gcm::new(&key);
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = aes_gcm::Nonce::from_slice(nonce_bytes);
    let plaintext_bytes = cipher.decrypt(nonce, ciphertext);
    match plaintext_bytes {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(str) => Ok(str),
            Err(err) => Err(AppError::Internal(Some(err.to_string()))),
        },
        Err(err) => Err(AppError::Internal(Some(err.to_string()))),
    }
}
