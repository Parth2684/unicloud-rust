use common::jwt_config::decode_jwt;
use futures_channel::mpsc::{UnboundedSender, unbounded};
use futures_util::{SinkExt, StreamExt};
use redis::{Client, TypedCommands};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::{
        Message, Utf8Bytes,
        handshake::server::{Request, Response},
    },
};
use url::Url;

type Tx = UnboundedSender<Message>;
pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

pub async fn accept_connection(
    stream: TcpStream,
    peer_map: PeerMap,
    addr: SocketAddr,
    redis_client: Client,
) {
    let request_url = Arc::new(Mutex::new(None::<Url>));
    let url_store = request_url.clone();

    let callback = move |req: &Request, res: Response| {
        let url = req.uri();
        let full_url = format!("ws://127.0.0.1:8080{:?}", url);
        println!("{full_url:?}");

        match Url::parse(&full_url) {
            Ok(parsed) => {
                if let Ok(mut guard) = url_store.lock() {
                    *guard = Some(parsed);
                } else {
                    return Err(Response::builder()
                        .status(500)
                        .body(Some("Internal lock error".into()))
                        .unwrap());
                }

                Ok(res)
            }

            Err(err) => {
                eprintln!("{err:?}");
                Err(Response::builder()
                    .status(400)
                    .body(Some("Invalid Url".into()))
                    .unwrap())
            }
        }
    };

    let ws_stream = match accept_hdr_async(stream, callback).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("Handshake error: {e:?}");
            return;
        }
    };

    let url_opt = {
        match request_url.lock() {
            Ok(guard) => guard.clone(),
            Err(err) => {
                eprintln!("{err}");
                return;
            }
        }
    };
    match url_opt {
        None => {
            return;
        }
        Some(url) => {
            let mut pairs: HashMap<String, String> = HashMap::new();
            let queries = url.query_pairs();

            for query in queries {
                pairs.insert(query.0.to_string(), query.1.to_string());
            }

            let token = match pairs.get("token") {
                None => return,
                Some(tok) => tok.to_owned(),
            };

            let (tx, rx) = unbounded();
            let mut peermap_clone = match peer_map.lock() {
                Ok(peer) => peer.clone(),
                Err(err) => {
                    eprintln!("{err:?}");
                    return;
                }
            };
            peermap_clone.insert(addr, tx);

            let (mut sender, mut receiver) = ws_stream.split();

            while let Some(msg) = receiver.next().await {
                let claims = match decode_jwt(&token) {
                    Ok(claim) => claim,
                    Err(err) => {
                        eprintln!("error decoding jwt: {}", err);
                        sender
                            .send(Message::Text(Utf8Bytes::from(String::from(
                                "Error Validating User from the websocket server",
                            ))))
                            .await
                            .ok();
                        break;
                    }
                };
                let msg = match msg {
                    Ok(m) => m,
                    Err(e) => {
                        eprintln!("{e:?}");
                        break;
                    }
                };
                if msg.is_text() {
                    let text = msg.to_text();
                    let text = match text {
                        Ok(str) => str.to_owned(),
                        Err(err) => {
                            eprintln!("{err:?}");
                            sender
                                .send(Message::Text(Utf8Bytes::from(format!("Server got {err}"))))
                                .await
                                .ok();
                            break;
                        }
                    };

                    let mut conn = redis_client.get_connection().unwrap();
                    if text == String::from("Refresh Token") {
                        let added = match conn.hset_nx(
                            "dedupe:queue",
                            "userid",
                            claims.id.to_string(),
                        ) {
                            Ok(red) => red,
                            Err(err) => {
                                eprintln!("{err:?}");
                                sender
                                    .send(Message::Text(Utf8Bytes::from(String::from(
                                        "Refreshing token failed due to connection error for redis",
                                    ))))
                                    .await
                                    .ok();
                                break;
                            }
                        };
                        if added {
                            let _ = conn.lpush("refreshtoken:queue", claims.id.to_string());
                        }
                    }
                }
            }
        }
    }
}
