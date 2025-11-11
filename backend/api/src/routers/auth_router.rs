use axum::{Router, routing::get};

use crate::handlers::auth::login_with_google::{google_auth_callback, google_auth_redirect};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/google", get(google_auth_redirect))
        .route("/google/callback", get(google_auth_callback))
}
