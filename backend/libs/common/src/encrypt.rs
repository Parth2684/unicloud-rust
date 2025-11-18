use aes_gcm::{
    AeadCore, Aes256Gcm, Error, Key, KeyInit,
    aead::{AeadMut, OsRng},
};
use thiserror::Error;

use crate::export_envs::ENVS;

#[derive(Debug, Error)]
pub enum DecryptError {
    #[error("AES decryption failed: {0}")]
    Aes(aes_gcm::Error),

    #[error("Invalid UTF-8 sequence in decrypted data: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

pub fn encrypt(plaintext: &str) -> Result<Vec<u8>, Error> {
    let key: &Key<Aes256Gcm> = &ENVS.encryption_key.try_into().unwrap();
    let mut cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher_text = match cipher.encrypt(&nonce, plaintext.as_ref()) {
        Ok(ciphertext) => ciphertext,
        Err(err) => {
            return Err(err);
        }
    };
    let mut concat_nonce_cipher = Vec::new();
    concat_nonce_cipher.extend_from_slice(&nonce);
    concat_nonce_cipher.extend_from_slice(&cipher_text);
    Ok(concat_nonce_cipher)
}

pub fn decrypt(data: Vec<u8>) -> Result<String, DecryptError> {
    let key: &Key<Aes256Gcm> = &ENVS.encryption_key.try_into().unwrap();
    let mut cipher = Aes256Gcm::new(&key);
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = aes_gcm::Nonce::from_slice(nonce_bytes);
    let plaintext_bytes = cipher.decrypt(nonce, ciphertext);
    match plaintext_bytes {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(str) => Ok(str),
            Err(err) => Err(DecryptError::Utf8(err)),
        },
        Err(err) => Err(DecryptError::Aes(err)),
    }
}
