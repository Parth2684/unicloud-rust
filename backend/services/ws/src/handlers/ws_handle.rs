use std::sync::{Arc, Mutex};

use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};
use url::Url;

async fn accept_connection(stream: TcpStream) {
    let request_url = Arc::new(Mutex::new(None::<String>));
    let url_store = request_url.clone();

    let callback = move |req: &Request, res: Response| {
        let url = req.uri().to_string();
    
        match Url::parse(&url) {
            Ok(parsed) => {
                if let Ok(mut guard) = url_store.lock() {
                    *guard = Some(parsed.to_string());
                } else {
                    return Err(Response::builder()
                            .status(500)
                            .body(Some("Internal lock error".into()))
                            .unwrap(),
                    );
                }
    
                Ok(res)
            }
    
            Err(_) => {
                Err(Response::builder()
                        .status(400)
                        .body(Some("Invalid Url".into()))
                        .unwrap(),
                )
            }
        }
    };



    let ws_stream = match tokio_tungstenite::accept_hdr_async(stream, callback).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("Handshake error: {e:?}");
            return;
        }
    };

    // Read parsed URL
    let url = request_url.lock().ok().and_then(|g| g.clone());
    println!("URL = {:?}", url);
}
