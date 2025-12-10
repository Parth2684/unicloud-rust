use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .alter_table(
                Table::alter()
                    .table(CloudAccount::Table)
                    .add_column_if_not_exists(ColumnDef::new(CloudAccount::Image)
                        .string()
                        )   
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .alter_table(Table::alter().table(CloudAccount::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CloudAccount {
    Table,
    Image
}
