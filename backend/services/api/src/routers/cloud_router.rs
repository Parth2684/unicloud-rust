use axum::{Router, middleware, routing::get};

use crate::{handlers::cloud::google_drive_file_structure::google_drive_file_structure, utils::middleware::auth_middleware};




pub fn cloud_router() -> Router {
    Router::new()
        .route("/get-google-drives", get(google_drive_file_structure))
        .layer(middleware::from_fn(auth_middleware))
} 