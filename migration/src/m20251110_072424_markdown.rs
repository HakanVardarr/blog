use sea_orm_migration::{prelude::*, schema::*};

use crate::m20251110_072405_frontmatter::FrontMatter;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Markdown::Table)
                    .col(pk_auto(Markdown::Id))
                    .col(integer(Markdown::FrontMatterId).not_null())
                    .col(text(Markdown::Html).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_markdown_frontmatter")
                            .from(Markdown::Table, Markdown::FrontMatterId)
                            .to(FrontMatter::Table, FrontMatter::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Markdown::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Markdown {
    Table,
    Id,
    FrontMatterId,
    Html,
}
