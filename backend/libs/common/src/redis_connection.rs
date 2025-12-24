use std::sync::Arc;

use redis::aio::ConnectionManager;

use crate::export_envs::ENVS;

pub async fn init_redis() -> ConnectionManager {
    let redis_url = &ENVS.redis_url.to_owned();
    let redis_client = Arc::new(redis::Client::open(redis_url.as_str()).unwrap());
    redis_client.get_connection_manager().await.unwrap()
}
