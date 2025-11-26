use crate::handlers::ws_handle::{PeerMap, accept_connection};
use common::export_envs::ENVS;
use redis::IntoConnectionInfo;
use std::{cell::RefCell, collections::HashMap, fmt::Error, sync::Mutex};
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

    while let Ok((stream, addr)) = listner.accept().await {
        tokio::spawn(accept_connection(
            stream,
            state.clone(),
            addr,
            redis_client.clone(),
        ));
    }
    Ok(())
}
