pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20251101_100506_create_more_required_tables;
mod m20251104_122027_adding_sub_for_oauth_in_users;
mod m20251105_154841_add_sub_field_to_cloud_account;
mod m20251111_155841_user_to_users;
mod m20251126_165431_add_refresh_token_expired_field_to_cloud_accounts;
mod m20251210_150222_add_image_to_cloud_account;
mod m20251225_162600_add_transfer_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20251101_100506_create_more_required_tables::Migration),
            Box::new(m20251104_122027_adding_sub_for_oauth_in_users::Migration),
            Box::new(m20251105_154841_add_sub_field_to_cloud_account::Migration),
            Box::new(m20251111_155841_user_to_users::Migration),
            Box::new(m20251126_165431_add_refresh_token_expired_field_to_cloud_accounts::Migration),
            Box::new(m20251210_150222_add_image_to_cloud_account::Migration),
            Box::new(m20251225_162600_add_transfer_table::Migration),
        ]
    }
}
