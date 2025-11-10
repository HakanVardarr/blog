use chrono::NaiveDate;
use serde::Serialize;
use thiserror::Error;

macro_rules! take_field {
    ($missing:expr, $key:literal, $value:expr, $FrontMatter:expr, $field:ident) => {{
        $missing.remove(
            $missing
                .iter()
                .position(|&s| s == $key)
                .ok_or(FrontMatterParseError::DuplicateKey($key))?,
        );
        $FrontMatter.$field = $value;
    }};
}

#[derive(Default, Debug, Serialize)]
pub struct FrontMatter {
    pub title: String,
    pub date: NaiveDate,
    pub slug: String,
    pub tags: Vec<String>,
    pub description: String,
}

#[derive(Debug, Error)]
pub enum FrontMatterParseError {
    /// Triggered when a required FrontMatter field (like `title` or `date`) is missing.
    #[error(
        "Missing required field: `{0}`. Please make sure all mandatory FrontMatter keys are included."
    )]
    MissingKey(&'static str),
    /// Triggered when multiple required fields are missing at once.
    #[error("Missing required fields: {0:?}. Please add these keys to your FrontMatter block.")]
    MissingKeys(Vec<&'static str>),
    /// Triggered when there is a duplicate key in the FrontMatter.
    #[error("There is duplicate key: `{0}` in the FrontMatter block please remove one of them.")]
    DuplicateKey(&'static str),
    /// Triggered when the date format is invalid.
    #[error("Invalid date format. Please use the format: YYYY-MM-DD (e.g., 2025-11-03).")]
    InvalidDateTime,
}

impl FrontMatter {
    pub fn new(raw: &str) -> Result<Self, FrontMatterParseError> {
        let mut front_matter = FrontMatter::default();
        let mut missing = vec!["title", "date", "slug", "tags", "description"];

        for line in raw.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let line_split = line.split(':').map(|s| s.trim()).collect::<Vec<_>>();
            let key = line_split[0];
            let value = line_split[1];

            match key {
                "title" => take_field!(
                    missing,
                    "title",
                    value.replace("\"", ""),
                    front_matter,
                    title
                ),
                "date" => {
                    let parsed_date = NaiveDate::parse_from_str(value, "%Y-%m-%d")
                        .map_err(|_| FrontMatterParseError::InvalidDateTime)?;
                    take_field!(missing, "date", parsed_date, front_matter, date)
                }
                "slug" => take_field!(missing, "slug", value.replace("\"", ""), front_matter, slug),
                "tags" => {
                    let tags = value[1..value.len() - 1]
                        .split(',')
                        .map(|s| s.replace("\"", "").trim().into())
                        .collect();
                    take_field!(missing, "tags", tags, front_matter, tags)
                }
                "description" => {
                    take_field!(
                        missing,
                        "description",
                        value.replace("\"", ""),
                        front_matter,
                        description
                    )
                }
                _ => unreachable!(),
            }
        }

        if !missing.is_empty() {
            if missing.len() == 1 {
                return Err(FrontMatterParseError::MissingKey(missing[0]));
            } else {
                return Err(FrontMatterParseError::MissingKeys(missing));
            }
        }

        Ok(front_matter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_full_frontmatter() {
        let raw = r#"
title: "Rust Blog"
date: 2025-11-03
slug: "rust-blog"
tags: ["rust", "axum", "markdown"]
description: "A test blog FrontMatter."
"#;

        let meta = FrontMatter::new(raw).unwrap();
        assert_eq!(meta.title, "Rust Blog");
        assert_eq!(meta.date, NaiveDate::from_ymd_opt(2025, 11, 3).unwrap());
        assert_eq!(meta.slug, "rust-blog");
        assert_eq!(meta.tags, vec!["rust", "axum", "markdown"]);
        assert_eq!(meta.description, "A test blog FrontMatter.");
    }

    #[test]
    fn parse_missing_single_key() {
        let raw = r#"
title: "Rust Blog"
slug: "rust-blog"
tags: ["rust", "axum"]
description: "Missing date field."
"#;

        let err = FrontMatter::new(raw).unwrap_err();
        match err {
            FrontMatterParseError::MissingKey(key) => assert_eq!(key, "date"),
            _ => panic!("Expected MissingKey error"),
        }
    }

    #[test]
    fn parse_missing_multiple_keys() {
        let raw = r#"
title: "Rust Blog"
tags: ["rust"]
"#;

        let err = FrontMatter::new(raw).unwrap_err();
        match err {
            FrontMatterParseError::MissingKeys(keys) => {
                assert!(keys.contains(&"date"));
                assert!(keys.contains(&"slug"));
                assert!(keys.contains(&"description"));
            }
            _ => panic!("Expected MissingKeys error"),
        }
    }

    #[test]
    fn parse_invalid_date() {
        let raw = r#"
title: "Rust Blog"
date: 11-03-2025
slug: "rust-blog"
tags: ["rust"]
description: "Invalid date format."
"#;

        let err = FrontMatter::new(raw).unwrap_err();
        match err {
            FrontMatterParseError::InvalidDateTime => (),
            _ => panic!("Expected InvalidDateTime error"),
        }
    }

    #[test]
    fn parse_duplicate_key() {
        let raw = r#"
title: "Rust Blog"
title: "Duplicate"
date: 2025-11-03
slug: "rust-blog"
tags: ["rust"]
description: "Duplicate title."
"#;

        let err = FrontMatter::new(raw).unwrap_err();
        match err {
            FrontMatterParseError::DuplicateKey(key) => assert_eq!(key, "title"),
            _ => panic!("Expected DuplicateKey error"),
        }
    }

    #[test]
    fn parse_with_comments_and_empty_lines() {
        let raw = r#"
# This is a comment
title: "Rust Blog"

date: 2025-11-03

# Another comment
slug: "rust-blog"
tags: ["rust", "axum"]

description: "Handles comments and empty lines."
"#;

        let meta = FrontMatter::new(raw).unwrap();
        assert_eq!(meta.title, "Rust Blog");
        assert_eq!(meta.date, NaiveDate::from_ymd_opt(2025, 11, 3).unwrap());
        assert_eq!(meta.slug, "rust-blog");
        assert_eq!(meta.tags, vec!["rust", "axum"]);
        assert_eq!(meta.description, "Handles comments and empty lines.");
    }
}
