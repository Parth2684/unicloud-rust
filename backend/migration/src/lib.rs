pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20251101_100506_create_more_required_tables;
mod m20251104_122027_adding_sub_for_oauth_in_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20251101_100506_create_more_required_tables::Migration),
            Box::new(m20251104_122027_adding_sub_for_oauth_in_users::Migration),
        ]
    }
}
