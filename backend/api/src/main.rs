use crate::{routers::auth_router::auth_routes, utils::export_envs::ENVS};
use axum::{Router, http::HeaderValue, routing::get};
use http::Method;
use tower_http::cors::{AllowHeaders, CorsLayer};

mod handlers;
mod routers;
mod utils;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(
            ENVS.frontend_url
                .to_string()
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_credentials(true)
        .allow_headers(AllowHeaders::mirror_request())
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ]);

    let app = Router::<()>::new()
        .route("/", get(|| async { "Hello World" }))
        .nest("/api/v1/auth", auth_routes())
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
