use axum::{Router, routing::get};

use crate::db_connect::db_connect;
mod db_connect;
mod export_envs;

#[tokio::main]
async fn main() {
    let db = db_connect().await;
    match db {
        Ok(db) => {
            println!("Connected to db");
            db
        },
        Err(err) => panic!("Error conncecting to database, {}", err)
    };
    let app = Router::<()>::new().route("/", get(|| async { "Hello World" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
