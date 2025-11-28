use crate::export_envs::ENVS;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;
use tokio::sync::OnceCell;

pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn init_db() -> &'static DatabaseConnection {
    DB.get_or_init(|| async {
        let mut opt = ConnectOptions::new(&ENVS.database_url);
        opt.max_connections(10)
            .min_connections(1)
            .connect_timeout(Duration::from_secs(10))
            .acquire_timeout(Duration::from_secs(10))
            .idle_timeout(Duration::from_secs(300))
            .max_lifetime(Duration::from_secs(1800))
            .sqlx_logging(false) // disable SQLx logging
            .sqlx_logging_level(log::LevelFilter::Info)
            .set_schema_search_path("public");

        match Database::connect(opt).await {
            Ok(db) => db,
            Err(err) => panic!("{err:?}"),
        }
    })
    .await
}
