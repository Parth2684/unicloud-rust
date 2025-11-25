use std::{collections::HashMap, fmt::Error, sync::Mutex};

use tokio::net::TcpListener;

use crate::handlers::ws_handle::{PeerMap, accept_connection};

mod handlers;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = String::from("0.0.0.0:8080");
    let state = PeerMap::new(Mutex::new(HashMap::new()));
    let try_socket = TcpListener::bind(&addr).await;
    let listner = try_socket.expect("Failed to bind");
    println!("Listeneing on {:?}", addr);

    while let Ok((stream, addr)) = listner.accept().await {
        tokio::spawn(accept_connection(stream, state.clone(), addr));
    }
    Ok(())
}
