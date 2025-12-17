use axum::{Router, middleware, routing::get};

use crate::{
    handlers::cloud::{
        get_cloud_accounts::get_cloud_accounts, get_shared_drive::get_shared_drives, google_get_folders::{google_get_folders, google_get_root}
    },
    utils::middleware::auth_middleware,
};

pub fn cloud_router() -> Router {
    Router::new()
        .route("/get-cloud-accounts", get(get_cloud_accounts))
        .route("/google/root/{drive_id}", get(google_get_root))
        .route(
            "/google/folder/{drive_id}/{folder_id}",
            get(google_get_folders),
        )
        .route("/google/shared_drive/{drive_id}", get(get_shared_drives))
        .layer(middleware::from_fn(auth_middleware))
}
