use common::{db_connect::init_db, export_envs::ENVS};
use redis::AsyncTypedCommands;
use std::sync::Arc;
use uuid::Uuid;
mod handle_refresh;

#[tokio::main]
async fn main() {
    let redis_url = &ENVS.redis_url.to_owned();
    let redis_client = Arc::new(redis::Client::open(redis_url.as_str()).unwrap());
    let mut redis_conn = redis_client.get_connection_manager().await.unwrap();
    let db = init_db().await;

    loop {
        let result: (String, String) = redis_conn
            .brpoplpush(String::from("refreshtoken:queue"), "refreshtoken:queue".to_owned(), 0)
            .await;
        let id = result.1;
        let id = match Uuid::parse_str(&id) {
            Ok(id) => id,
            Err(err) => {
                eprintln!("{err:?}");
                return;
            }
        };
    }
}
