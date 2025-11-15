use hex::FromHex;
use once_cell::sync::Lazy;

pub struct Envs {
    pub database_url: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_client_redirect_url: String,
    pub backend_url: String,
    pub jwt_secret: String,
    pub frontend_url: String,
    pub environment: String,
    pub google_drive_client_id: String,
    pub google_drive_client_secret: String,
    pub google_drive_redirect_url: String,
    pub encryption_key: [u8; 32],
}

pub static ENVS: Lazy<Envs> = Lazy::new(|| {
    dotenvy::dotenv().ok();
    let database_url =
        dotenvy::var("DATABASE_URL").expect("DATABASE_URL not found in the environment");
    let google_client_id =
        dotenvy::var("GOOGLE_CLIENT_ID").expect("Google Client Id for Oauth2 must be provided");
    let google_client_secret = dotenvy::var("GOOGLE_CLIENT_SECRET")
        .expect("Google Client Secret for Oauth2 must be provided");
    let google_client_redirect_url = dotenvy::var("GOOGLE_CLIENT_REDIRECT_URL")
        .expect("Goodle Client Redirect Url must be provided");
    let backend_url = dotenvy::var("BACKEND_URL").expect("Backend Url must be provided");
    let jwt_secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET must be provided");
    let frontend_url = dotenvy::var("FRONTEND_URL").expect("fronend url is not present in env");
    let environment =
        dotenvy::var("ENVIRONMENT").expect("Environment variable is not defined in env");
    let google_drive_client_id = dotenvy::var("GOOGLE_DRIVE_CLIENT_ID")
        .expect("Google Client Id for google drive is not provided");
    let google_drive_client_secret = dotenvy::var("GOOGLE_DRIVE_CLIENT_SECRET")
        .expect("Google Client secret got google drive is not provided");
    let google_drive_redirect_url = dotenvy::var("GOOGLE_DRIVE_CLIENT_REDIRECT_URL")
        .expect("Google drive client redirect url is not provided");
    let encryption_key = dotenvy::var("ENCRYPTION_KEY")
        .expect("encryption key is necessary for encryption in aes-256");
    let encryption_key = match <[u8; 32]>::from_hex(encryption_key) {
        Ok(key) => key,
        Err(err) => {
            panic!("Put a correct encryption key in env {err:?}");
        }
    };

    Envs {
        database_url,
        google_client_id,
        google_client_secret,
        google_client_redirect_url,
        backend_url,
        jwt_secret,
        frontend_url,
        environment,
        google_drive_client_id,
        google_drive_client_secret,
        google_drive_redirect_url,
        encryption_key,
    }
});
