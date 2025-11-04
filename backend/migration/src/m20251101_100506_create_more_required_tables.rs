use sea_orm_migration::prelude::{extension::postgres::Type, *};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(QuotaType::Table)
                    .values(["Free", "Bronze", "Silver", "Gold", "Platinum"])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Provider::Table)
                    .values(["Google", "Mega"])
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(CloudAccount::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CloudAccount::Id)
                            .uuid()
                            .not_null()
                            .unique_key()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CloudAccount::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(CloudAccount::Provider)
                            .enumeration(Provider::Table, ["Google", "Mega"])
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CloudAccount::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(CloudAccount::AccessToken)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CloudAccount::RefreshToken).string())
                    .col(
                        ColumnDef::new(CloudAccount::IsPrimary)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(CloudAccount::CreatedAt)
                            .date_time()
                            .default(Expr::cust("NOW()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-cloudaccount-user-id")
                            .from(CloudAccount::Table, CloudAccount::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Quota::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Quota::Id)
                            .primary_key()
                            .not_null()
                            .unique_key()
                            .uuid(),
                    )
                    .col(ColumnDef::new(Quota::UserId).uuid().not_null().unique_key())
                    .col(
                        ColumnDef::new(Quota::FreeQuota)
                            .float()
                            .not_null()
                            .default(5.0),
                    )
                    .col(
                        ColumnDef::new(Quota::AddOnQuota)
                            .float()
                            .not_null()
                            .default(0.0),
                    )
                    .col(
                        ColumnDef::new(Quota::TotalQuota)
                            .float()
                            .not_null()
                            .default(5.0),
                    )
                    .col(
                        ColumnDef::new(Quota::UsedQuota)
                            .float()
                            .not_null()
                            .default(0.0),
                    )
                    .col(
                        ColumnDef::new(Quota::RemainingQuota)
                            .float()
                            .not_null()
                            .default(5.0),
                    )
                    .col(
                        ColumnDef::new(Quota::QuotaType)
                            .enumeration(
                                QuotaType::Table,
                                ["Free", "Bronze", "Silver", "Gold", "Platinum"],
                            )
                            .not_null()
                            .default("Free"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-quota-user-id")
                            .from(Quota::Table, Quota::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CloudAccount::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Quota::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(QuotaType::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(Provider::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CloudAccount {
    Table,
    Id,
    UserId,
    Provider,
    Email,
    AccessToken,
    RefreshToken,
    IsPrimary,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Provider {
    Table,
}

#[derive(DeriveIden)]
enum Quota {
    Table,
    Id,
    UserId,
    FreeQuota,
    AddOnQuota,
    TotalQuota,
    UsedQuota,
    RemainingQuota,
    QuotaType,
}

#[derive(DeriveIden)]
enum QuotaType {
    Table,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}
