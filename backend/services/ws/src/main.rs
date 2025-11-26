use crate::handlers::ws_handle::{PeerMap, accept_connection};
use common::export_envs::ENVS;
use std::{collections::HashMap, fmt::Error, sync::{Arc, Mutex}};
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
    let redis_conn = redis_client.get_connection_manager().await.unwrap();
    let redis_conn = Arc::new(Mutex::new(redis_conn));

    while let Ok((stream, addr)) = listner.accept().await {
        tokio::spawn(accept_connection(
            stream,
            state.clone(),
            addr,
            redis_conn.clone(),
        ));
    }
    Ok(())
}
