use axum::{Router, routing::get};

use crate::routers::auth_router::auth_routes;
mod app_errors;
mod db_connect;
mod export_envs;
mod handlers;
mod routers;

#[tokio::main]
async fn main() {
    let app = Router::<()>::new().route("/", get(|| async { "Hello World" }))
        .nest("/api/v1/auth", auth_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
