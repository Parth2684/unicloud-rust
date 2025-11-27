use crate::handlers::ws_handle::{PeerMap, accept_connection};
use common::export_envs::ENVS;
use redis::aio::ConnectionManager;
use std::{
    collections::HashMap,
    fmt::Error,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;

mod handlers;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = String::from("0.0.0.0:8080");
    let state = PeerMap::new(Mutex::new(HashMap::new()));
    let try_socket = TcpListener::bind(&addr).await;
    let listner = try_socket.expect("Failed to bind");
    println!("Listeneing on {:?}", addr);
    let redis_url = &ENVS.redis_url.to_owned();
    let redis_client = redis::Client::open(redis_url.as_str()).unwrap();
    let manager = ConnectionManager::new(redis_client).await.unwrap();
    let redis = Arc::new(manager);

    while let Ok((stream, addr)) = listner.accept().await {
        let conn = Arc::clone(&redis);

        tokio::spawn(accept_connection(stream, state.clone(), addr, conn));
    }
    Ok(())
}
