use super::entities;
use super::frontmatter::*;

use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use serde::Serialize;
use std::{fs, io::Read};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MarkdownParseError {
    #[error("Front matter must start with `---` and end with `---`")]
    InvalidFrontMatter,
    #[error(transparent)]
    FrontMatterParseError(#[from] FrontMatterParseError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Serialize)]
pub struct Markdown {
    pub front_matter: FrontMatter,
    pub html: String,
}

impl Markdown {
    pub fn new<P>(path: P) -> Result<Self, MarkdownParseError>
    where
        P: AsRef<std::path::Path>,
    {
        let mut file = fs::File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let trimmed = content.trim();
        if !trimmed.starts_with("---") {
            return Err(MarkdownParseError::InvalidFrontMatter);
        }
        let rest = &trimmed[3..];
        let end_pos = rest
            .find("---")
            .ok_or(MarkdownParseError::InvalidFrontMatter)?;
        let front_matter_content = &rest[..end_pos];
        let body_content = &rest[end_pos + 3..].trim();

        let front_matter = FrontMatter::new(front_matter_content)?;

        let html = markdown::to_html(body_content);

        Ok(Self { front_matter, html })
    }

    pub async fn insert_to_db(&self, db: &DatabaseConnection) -> anyhow::Result<i32> {
        let fm_active = entities::front_matter::ActiveModel {
            title: Set(self.front_matter.title.clone()),
            date: Set(self.front_matter.date),
            slug: Set(self.front_matter.slug.clone()),
            tags: Set(self.front_matter.tags.clone()),
            description: Set(self.front_matter.description.clone()),
            ..Default::default()
        };

        let fm_inserted = entities::front_matter::Entity::insert(fm_active)
            .exec(db)
            .await?;

        let md_active = entities::markdown::ActiveModel {
            front_matter_id: Set(fm_inserted.last_insert_id), // FK
            html: Set(self.html.clone()),
            ..Default::default()
        };
        let md_inserted = entities::markdown::Entity::insert(md_active)
            .exec(db)
            .await?;

        Ok(md_inserted.last_insert_id)
    }
}
