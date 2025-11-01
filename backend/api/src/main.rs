use axum::{
    Router, routing::{get}
};


#[tokio::main]
async fn main() {
    let _add = Router::<()>::new()
        .route("/", get(|| async { "Hello World" }));
    
    let _listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
}
