use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*, sea_orm::EnumIter,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_type(
                Type::create()
                    .as_enum(Status::Table)
                    .values([Status::Pending, Status::Running, Status::Completed, Status::Failed])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(TransferType::Table)
                    .values([TransferType::GoogleToGoogle, TransferType::MegaToGoogle])
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_type(Type::create()
                .as_enum(LinkType::Table)
                .values([LinkType::Torrent])
                .to_owned()
            )
            .await?;
        
        manager
            .create_table(
                Table::create()
                    .table(Job::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Job::Id)
                            .uuid()
                            .primary_key()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Job::FromDrive).uuid())
                    .col(ColumnDef::new(Job::FromFileId).string())
                    .col(ColumnDef::new(Job::IsFolder).boolean().default(false))
                    .col(ColumnDef::new(Job::ToDrive).uuid().not_null())
                    .col(ColumnDef::new(Job::ToFolderId).string().not_null())
                    .col(
                        ColumnDef::new(Job::CreatedAt)
                            .date_time()
                            .default(Expr::cust("NOW()"))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Job::Status)
                            .enumeration(
                                Status::Table,
                                [Status::Pending, Status::Running, Status::Completed, Status::Failed],
                            )
                            .not_null()
                            .default(Expr::cust("'pending'")),
                    )
                    .col(ColumnDef::new(Job::UserId).uuid().not_null())
                    .col(ColumnDef::new(Job::Size).big_integer())
                    .col(ColumnDef::new(Job::Link).string())
                    .col(ColumnDef::new(Job::LinkType)
                        .enumeration(LinkType::Table, 
                        [LinkType::Torrent]
                        )
                    )
                    .col(ColumnDef::new(Job::TransferType)
                        .enumeration(TransferType::Table, [TransferType::GoogleToGoogle, TransferType::MegaToGoogle])
                        .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-job-user-id")
                            .from(Job::Table, Job::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-job-fromDrive-id")
                            .from(Job::Table, Job::FromDrive)
                            .to(CloudAccount::Table, CloudAccount::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-job-toDrive-id")
                            .from(Job::Table, Job::ToDrive)
                            .to(CloudAccount::Table, CloudAccount::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Job::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Job {
    Table,
    Id,
    FromDrive,
    FromFileId,
    IsFolder,
    ToDrive,
    ToFolderId,
    CreatedAt,
    UserId,
    Status,
    Size,
    Link,
    LinkType,
    TransferType
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum CloudAccount {
    Table,
    Id,
}

#[derive(DeriveIden, EnumIter)]
enum Status {
    Table,
    Pending,
    Running,
    Completed,
    Failed
}

#[derive(DeriveIden)]
enum TransferType {
    Table,
    GoogleToGoogle,
    MegaToGoogle,
}

#[derive(DeriveIden)]
enum LinkType {
    Table,
    Torrent,
}