use crate::export_envs::ENVS;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;
use tokio::sync::OnceCell;

pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn init_db() -> &'static DatabaseConnection {
    DB.get_or_init(|| async {
        let mut opt = ConnectOptions::new(&ENVS.database_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
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
