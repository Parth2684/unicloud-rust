use axum::{Router, routing::get};
mod db_connect;
mod export_envs;
mod handlers;
mod app_errors;

#[tokio::main]
async fn main() {
    let app = Router::<()>::new().route("/", get(|| async { "Hello World" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
