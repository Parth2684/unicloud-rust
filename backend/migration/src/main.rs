use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}

// To create a migration file sea-orm-cli migrate generate "name of migration"
// To migrate database and reset schema sea-orm-cli migrate fresh
// To migrate database sea-orm-cli migrate up
// To generate entities sea-orm-cli generate entity -o entities/src --with-serde both --lib
