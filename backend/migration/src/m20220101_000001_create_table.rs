use sea_orm_migration::{prelude::{extension::postgres::Type, *}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_type(
                Type::create()
                    .as_enum(QuotaType::Table)
                    .values(["Free", "Bronze", "Silver", "Gold", "Platinum"])
                    .to_owned()
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id)
                        .uuid()
                        .not_null()
                        .unique_key()
                        .primary_key()
                    )
                    .col(ColumnDef::new(User::Gmail)
                        .string()
                        .not_null()
                        .unique_key()
                    )
                    .col(ColumnDef::new(User::CreatedAt)
                        .date_time()
                        .not_null()
                    )
                    .col(ColumnDef::new(User::Name)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(User::Image)
                        .string()
                    )
                    .col(ColumnDef::new(User::QuotaType)
                        .enumeration(QuotaType::Table, ["Free", "Bronze", "Silver", "Gold", "Platinum"])
                        .not_null()
                        .default("Free")
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(QuotaType::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Gmail,
    CreatedAt,
    Name,
    QuotaType,
    Image
}
#[derive(DeriveIden)]
enum QuotaType {
    Table
}