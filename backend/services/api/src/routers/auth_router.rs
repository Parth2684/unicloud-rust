use axum::{Router, middleware, routing::get};

use crate::{
    handlers::auth::{
        add_google_drive::{drive_auth_callback, drive_auth_redirect},
        login_with_google::{google_auth_callback, google_auth_redirect},
    },
    utils::middleware::auth_middleware,
};

pub fn auth_routes() -> Router {
    let protected_routes = Router::new()
        .route("/drive", get(drive_auth_redirect))
        .route("/drive/callback", get(drive_auth_callback))
        .layer(middleware::from_fn(auth_middleware));

    Router::new()
        .merge(protected_routes)
        .route("/google", get(google_auth_redirect))
        .route("/google/callback", get(google_auth_callback))
}
