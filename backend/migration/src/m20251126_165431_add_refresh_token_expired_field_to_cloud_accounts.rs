use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(
                Table::alter()
                    .table(CloudAccount::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(CloudAccount::TokenExpired)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(CloudAccount::UpdatedAt)
                            .date_time()
                            .default(Expr::cust("NOW()")),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(Table::alter().table(CloudAccount::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CloudAccount {
    Table,
    TokenExpired,
    UpdatedAt,
}
