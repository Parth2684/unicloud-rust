use common::jwt_config::decode_jwt;
use futures_channel::mpsc::{UnboundedSender, unbounded};
use futures_util::{SinkExt, StreamExt};
use redis::{AsyncTypedCommands, aio::ConnectionManager};
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
    mut conn: Arc<ConnectionManager>,
) {
    let request_url = Arc::new(Mutex::new(None::<Url>));
    let url_store = request_url.clone();

    let callback = move |req: &Request, res: Response| {
        let url = req.uri();
        let full_url = format!("ws://127.0.0.1:8080{}", url);
        println!("{full_url:?}");

        match Url::parse(&full_url) {
            Ok(parsed) => {
                println!("line 39");
                if let Ok(mut guard) = url_store.lock() {
                    *guard = Some(parsed);
                } else {
                    println!("line 43");
                    return Err(Response::builder()
                        .status(500)
                        .body(Some("Internal lock error".into()))
                        .unwrap());
                }
                println!("line 49");
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
    
    println!("72");

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
                println!("query {query:?}");
                pairs.insert(query.0.to_string(), query.1.to_string());
            }

            let token = match pairs.get("token") {
                None => return,
                Some(tok) => tok.to_owned(),
            };
            println!("token 96, {token}");

            let (tx, rx) = unbounded();
            match peer_map.lock() {
                Ok(mut peers) => {
                    peers.insert(addr, tx);
                }
                Err(err) => {
                    eprintln!("{err:?}");
                    return;
                }
            }
            println!("112");

            let (mut sender, mut receiver) = ws_stream.split();

            while let Some(msg) = receiver.next().await {
                println!("116");
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
                    println!("text 1st {:?}", text);
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
                    println!("text before if{}", text);
                    if text == String::from("Refresh Token") {
                        println!("text after if {}", text);
                        let redis_clone = Arc::make_mut(&mut conn);
                        println!("159");
                        // let added: Result<bool, redis::RedisError> = redis::cmd("HSETNX")
                        //     .arg("dedupe:queue")
                        //     .arg("userid")
                        //     .arg(claims.id.to_string())
                        //     .query_async(redis_clone)
                        //     .await;
                        let added = redis_clone
                            .hset_nx("dedupe:queue", claims.id.to_string(), "1")
                            .await;
                        println!("166");
                        match added {
                            Ok(add) => {
                                if add {
                                    // let _: Result<isize, redis::RedisError> = redis::cmd("LPUSH")
                                    //     .arg("refreshtoken:queue")
                                    //     .arg(claims.id.to_string())
                                    //     .query_async(redis_clone)
                                    //     .await;
                                    let _ = redis_clone
                                        .lpush("refresh:queue", claims.id.to_string())
                                        .await;
                                }
                            }
                            Err(err) => {
                                eprintln!("error connecting to redis {err}");
                                break;
                            }
                        }
                    }
                }
            }
            match peer_map.lock() {
                Ok(mut peer) => peer.remove(&addr),
                Err(err) => {
                    eprintln!("{err:?}");
                    return;
                }
            };
        }
    }
}
