use std::sync::Arc;

use axum::{Router, routing::get};
use common::export_envs::ENVS;

mod handlers;

pub struct AppState {
    redis: redis::Client,
}

#[tokio::main]
async fn main() {
    let redis_url = &ENVS.redis_url;
    let redis_url = redis_url.to_owned();
    let redis = redis::Client::open(redis_url).unwrap();
    let state = Arc::new(AppState { redis });

    let app = Router::new().route("/", get(|| async { "hello" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
