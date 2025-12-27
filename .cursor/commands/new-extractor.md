# Create New Extractor

Guide for adding a new content extractor to Omnivore.

## Overview

Extractors are used to pull structured data from HTML pages. They live in `omnivore-core/src/parser/extractors.rs`.

## Step 1: Define the Output Type

```rust
use serde::{Deserialize, Serialize};

/// Data extracted from [describe source].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyExtractedData {
    /// The title or name
    pub title: String,

    /// Optional description
    pub description: Option<String>,

    /// Numeric value if applicable
    pub value: Option<f64>,

    /// Source URL
    pub source_url: String,
}
```

## Step 2: Implement the Extractor

```rust
use scraper::{Html, Selector};

/// Extracts [data type] from HTML.
///
/// # Arguments
/// * `html` - The HTML document to parse
/// * `base_url` - The URL of the page (for resolving relative links)
///
/// # Returns
/// A vector of extracted data items.
///
/// # Examples
/// ```
/// use omnivore_core::parser::extract_my_data;
///
/// let html = r#"<div class="item"><h2>Title</h2></div>"#;
/// let data = extract_my_data(html, "https://example.com");
/// assert_eq!(data.len(), 1);
/// ```
pub fn extract_my_data(html: &str, base_url: &str) -> Vec<MyExtractedData> {
    let document = Html::parse_document(html);

    // Define selectors
    let container_sel = Selector::parse(".item").expect("valid selector");
    let title_sel = Selector::parse("h2, .title").expect("valid selector");
    let desc_sel = Selector::parse(".description, p").expect("valid selector");

    document.select(&container_sel)
        .filter_map(|el| {
            // Extract title (required)
            let title = el.select(&title_sel)
                .next()?
                .text()
                .collect::<String>()
                .trim()
                .to_string();

            if title.is_empty() {
                return None;
            }

            // Extract description (optional)
            let description = el.select(&desc_sel)
                .next()
                .map(|d| d.text().collect::<String>().trim().to_string())
                .filter(|s| !s.is_empty());

            Some(MyExtractedData {
                title,
                description,
                value: None,
                source_url: base_url.to_string(),
            })
        })
        .collect()
}
```

## Step 3: Add Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_my_data_basic() {
        let html = r#"
            <div class="item">
                <h2>Product Name</h2>
                <p>Product description here.</p>
            </div>
        "#;

        let data = extract_my_data(html, "https://example.com/products");

        assert_eq!(data.len(), 1);
        assert_eq!(data[0].title, "Product Name");
        assert_eq!(data[0].description, Some("Product description here.".into()));
    }

    #[test]
    fn test_extract_my_data_multiple() {
        let html = r#"
            <div class="item"><h2>Item 1</h2></div>
            <div class="item"><h2>Item 2</h2></div>
            <div class="item"><h2>Item 3</h2></div>
        "#;

        let data = extract_my_data(html, "https://example.com");

        assert_eq!(data.len(), 3);
    }

    #[test]
    fn test_extract_my_data_empty() {
        let html = "<div>No items here</div>";
        let data = extract_my_data(html, "https://example.com");
        assert!(data.is_empty());
    }

    #[test]
    fn test_extract_my_data_malformed() {
        let html = r#"<div class="item"></div>"#;  // No title
        let data = extract_my_data(html, "https://example.com");
        assert!(data.is_empty());  // Should skip items without title
    }
}
```

## Step 4: Export the Extractor

In `omnivore-core/src/parser/mod.rs`:

```rust
mod extractors;
pub use extractors::{extract_my_data, MyExtractedData};
```

In `omnivore-core/src/lib.rs`:

```rust
pub use parser::{extract_my_data, MyExtractedData};
```

## Step 5: Add CLI Support (Optional)

If the extractor should be available via CLI, update `omnivore-cli`:

```rust
#[derive(ValueEnum, Clone)]
enum ExtractorType {
    Tables,
    Links,
    MyData,  // Add new variant
}

// In the handler:
match extractor_type {
    ExtractorType::MyData => {
        let data = extract_my_data(&html, &url);
        output_json(&data)?;
    }
    // ...
}
```

## Best Practices

1. **Be defensive**: Handle malformed HTML gracefully
2. **Use Option**: For fields that may not exist
3. **Trim text**: Always trim extracted text
4. **Test edge cases**: Empty, malformed, missing elements
5. **Document selectors**: Comment why specific selectors were chosen
