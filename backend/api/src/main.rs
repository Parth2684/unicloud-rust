use axum::{Router, routing::get};
mod export_envs;
mod db_connect;

#[tokio::main]
async fn main() {
    let envs = export_envs::get_envs();
    match envs {
        Result::Ok(vars) => vars,
        Result::Err(err) => panic!("Error loading Envs, {:?}", err),
    };
    db_connect::db_connect();
    let app = Router::<()>::new().route("/", get(|| async { "Hello World" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
