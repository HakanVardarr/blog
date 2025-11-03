use chrono::NaiveDate;
use thiserror::Error;

macro_rules! take_field {
    ($missing:expr, $key:literal, $value:expr, $metadata:expr, $field:ident) => {{
        $missing.remove(
            $missing
                .iter()
                .position(|&s| s == $key)
                .ok_or(MetadataParseError::DuplicateKey($key))?,
        );
        $metadata.$field = $value;
    }};
}

#[derive(Debug, Default)]
pub struct Metadata {
    title: String,
    date: NaiveDate,
    slug: String,
    tags: Vec<String>,
    description: String,
}

#[derive(Debug, Error)]
pub enum MetadataParseError {
    /// Triggered when a required metadata field (like `title` or `date`) is missing.
    #[error(
        "Missing required field: `{0}`. Please make sure all mandatory metadata keys are included."
    )]
    MissingKey(&'static str),
    /// Triggered when multiple required fields are missing at once.
    #[error("Missing required fields: {0:?}. Please add these keys to your metadata block.")]
    MissingKeys(Vec<&'static str>),
    /// Triggered when there is a duplicate key in the metadata.
    #[error("There is duplicate key: `{0}` in the metadata block please remove one of them.")]
    DuplicateKey(&'static str),
    /// Triggered when the date format is invalid.
    #[error("Invalid date format. Please use the format: YYYY-MM-DD (e.g., 2025-11-03).")]
    InvalidDateTime,
}

impl Metadata {
    pub fn new(raw: &str) -> Result<Self, MetadataParseError> {
        let mut metadata = Metadata::default();
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
                "title" => take_field!(missing, "title", value.replace("\"", ""), metadata, title),
                "date" => {
                    let parsed_date = NaiveDate::parse_from_str(value, "%Y-%m-%d")
                        .map_err(|_| MetadataParseError::InvalidDateTime)?;
                    take_field!(missing, "date", parsed_date, metadata, date)
                }
                "slug" => take_field!(missing, "slug", value.replace("\"", ""), metadata, slug),
                "tags" => {
                    let tags = value[1..value.len() - 1]
                        .split(',')
                        .map(|s| s.replace("\"", "").trim().into())
                        .collect();
                    take_field!(missing, "tags", tags, metadata, tags)
                }
                "description" => {
                    take_field!(
                        missing,
                        "description",
                        value.replace("\"", ""),
                        metadata,
                        description
                    )
                }
                _ => unreachable!(),
            }
        }

        if !missing.is_empty() {
            if missing.len() == 1 {
                return Err(MetadataParseError::MissingKey(missing[0]));
            } else {
                return Err(MetadataParseError::MissingKeys(missing));
            }
        }

        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_full_metadata() {
        let raw = r#"
title: "Rust Blog"
date: 2025-11-03
slug: "rust-blog"
tags: ["rust", "axum", "markdown"]
description: "A test blog metadata."
"#;

        let meta = Metadata::new(raw).unwrap();
        assert_eq!(meta.title, "Rust Blog");
        assert_eq!(meta.date, NaiveDate::from_ymd_opt(2025, 11, 3).unwrap());
        assert_eq!(meta.slug, "rust-blog");
        assert_eq!(meta.tags, vec!["rust", "axum", "markdown"]);
        assert_eq!(meta.description, "A test blog metadata.");
    }

    #[test]
    fn parse_missing_single_key() {
        let raw = r#"
title: "Rust Blog"
slug: "rust-blog"
tags: ["rust", "axum"]
description: "Missing date field."
"#;

        let err = Metadata::new(raw).unwrap_err();
        match err {
            MetadataParseError::MissingKey(key) => assert_eq!(key, "date"),
            _ => panic!("Expected MissingKey error"),
        }
    }

    #[test]
    fn parse_missing_multiple_keys() {
        let raw = r#"
title: "Rust Blog"
tags: ["rust"]
"#;

        let err = Metadata::new(raw).unwrap_err();
        match err {
            MetadataParseError::MissingKeys(keys) => {
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

        let err = Metadata::new(raw).unwrap_err();
        match err {
            MetadataParseError::InvalidDateTime => (),
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

        let err = Metadata::new(raw).unwrap_err();
        match err {
            MetadataParseError::DuplicateKey(key) => assert_eq!(key, "title"),
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

        let meta = Metadata::new(raw).unwrap();
        assert_eq!(meta.title, "Rust Blog");
        assert_eq!(meta.date, NaiveDate::from_ymd_opt(2025, 11, 3).unwrap());
        assert_eq!(meta.slug, "rust-blog");
        assert_eq!(meta.tags, vec!["rust", "axum"]);
        assert_eq!(meta.description, "Handles comments and empty lines.");
    }
}
