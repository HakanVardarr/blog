use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FrontMatter::Table)
                    .col(pk_auto(FrontMatter::Id))
                    .col(string(FrontMatter::Title))
                    .col(date(FrontMatter::Date))
                    .col(string(FrontMatter::Slug))
                    .col(array(
                        FrontMatter::Tags,
                        ColumnType::String(StringLen::N(50)),
                    ))
                    .col(string(FrontMatter::Description))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FrontMatter::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum FrontMatter {
    Table,
    Id,
    Title,
    Date,
    Slug,
    Tags,
    Description,
}
