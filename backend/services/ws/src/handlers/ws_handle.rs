use futures_channel::mpsc::{UnboundedSender, unbounded};
use futures_util::{SinkExt, StreamExt};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::{
        Message,
        handshake::server::{Request, Response},
    },
};
use url::Url;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

pub async fn accept_connection(stream: TcpStream, peer_map: PeerMap, addr: SocketAddr) {
    let request_url = Arc::new(Mutex::new(None::<Url>));
    let url_store = request_url.clone();

    let callback = move |req: &Request, res: Response| {
        let url = req.uri().to_string();

        match Url::parse(&url) {
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

            Err(_) => Err(Response::builder()
                .status(400)
                .body(Some("Invalid Url".into()))
                .unwrap()),
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

            let (outgoing, incoming) = ws_stream.split();
        }
    }
}
