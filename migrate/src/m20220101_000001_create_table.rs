use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::UserName).string().not_null())
                    .col(ColumnDef::new(Users::AvatarUrl).string())
                    .col(ColumnDef::new(Feeds::CreatedDate).date_time().not_null())
                    .col(ColumnDef::new(Feeds::UpdateDate).date_time().not_null())
                    .to_owned(),
            ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Feeds::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Feeds::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Feeds::Url).string().not_null())
                    .col(ColumnDef::new(Feeds::Title).string())
                    .col(ColumnDef::new(Feeds::IconUrl).string())
                    .col(ColumnDef::new(Feeds::LastChangeDate).date_time())
                    .col(ColumnDef::new(Feeds::LastPollDate).date_time())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_a1fce996efcd40ab9eca9de5097d5878")
                            .from(Users::Table, Users::Id)
                            .to(Feeds::Table, Feeds::UserId)
                    )
                    .col(ColumnDef::new(Feeds::CreatedDate).date_time().not_null())
                    .col(ColumnDef::new(Feeds::UpdateDate).date_time().not_null())
                    .to_owned(),
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Feeds::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Users {
    Table,
    Id,
    UserName,
    AvatarUrl,
    CreatedDate,
    UpdateDate,
}

#[derive(Iden)]
enum Feeds {
    Table,
    Id,
    Url,
    Title,
    IconUrl,
    LastChangeDate,
    LastPollDate,
    CreatedDate,
    UpdateDate,
    UserId,
}
