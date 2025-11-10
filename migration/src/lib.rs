pub use sea_orm_migration::prelude::*;

mod m20251110_072405_frontmatter;
mod m20251110_072424_markdown;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251110_072405_frontmatter::Migration),
            Box::new(m20251110_072424_markdown::Migration),
        ]
    }
}
