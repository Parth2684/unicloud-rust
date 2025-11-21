use bytes::Bytes;
use chrono::Utc;
use futures_util::sink::SinkExt;
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use redis::AsyncCommands;
use redis::RedisError;
use std::{sync::Arc, time::Duration};
use tokio_tungstenite::WebSocketStream;
use tungstenite::{Message, protocol::Role};

use crate::AppState;

pub async fn upgrade_to_ws(
    upgraded: Upgraded,
    state: Arc<AppState>,
    cloud_id: String,
    expiry_time: i64,
) -> Result<(), RedisError> {
    let io = TokioIo::new(upgraded);
    let mut ws = WebSocketStream::from_raw_socket(io, Role::Server, None).await;

    {
        let mut conn = match state.redis.get_multiplexed_tokio_connection().await {
            Ok(con) => con,
            Err(err) => {
                eprintln!("{}", err);
                return Err(err);
            }
        };

        let score = expiry_time - 5 * 60;
        let _: () = conn
            .zadd("token_refresh_queue", &cloud_id, score)
            .await
            .unwrap();
    }
    loop {
        tokio::time::sleep(Duration::from_secs(30)).await;

        if ws.send(Message::Ping(Bytes::from(vec![]))).await.is_err() {
            break;
        }
    }
    Ok(())
}
