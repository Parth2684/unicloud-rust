use common::{db_connect::init_db, export_envs::ENVS};
use redis::{AsyncTypedCommands, RedisError};
use std::sync::Arc;
use uuid::Uuid;

use crate::handle_refresh::handle_refresh;
mod handle_refresh;

#[tokio::main]
async fn main() {
    let redis_url = &ENVS.redis_url.to_owned();
    let redis_client = Arc::new(redis::Client::open(redis_url.as_str()).unwrap());
    let mut redis_conn = redis_client.get_connection_manager().await.unwrap();
    let db = init_db().await;

    loop {
        let result: Result<Option<String>, RedisError> = redis_conn
            .brpoplpush("refresh:queue", "refresh:queue", 1.0)
            .await;

        let result = match result {
            Ok(some_str) => match some_str {
                Some(str) => {
                    str
                }
                None => continue,
            },
            Err(err) => {
                eprintln!("{err:?}");
                continue;
            }
        };
        let id = match Uuid::parse_str(&result) {
            Ok(uid) => uid,
            Err(err) => {
                eprintln!("{err:?}");
                continue;
            }
        };

        let should_retry = handle_refresh(id, db).await;
        if !should_retry {
            redis_conn.lrem("refresh:queue", 1, &result).await.ok();
            redis_conn.hdel("dedupe:queue", &result).await.ok();
        }
    }
}
