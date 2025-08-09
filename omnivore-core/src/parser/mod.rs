pub mod extractors;
pub mod html;
pub mod schema;

use crate::{Error, Result};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseRule {
    pub name: String,
    pub selector: String,
    pub attribute: Option<String>,
    pub multiple: bool,
    pub required: bool,
    pub transform: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseConfig {
    pub rules: Vec<ParseRule>,
    pub schema_name: Option<String>,
    pub clean_text: bool,
    pub extract_metadata: bool,
}

pub struct Parser {
    config: ParseConfig,
}

impl Parser {
    pub fn new(config: ParseConfig) -> Self {
        Self { config }
    }

    pub fn parse(&self, html: &str) -> Result<Value> {
        let document = Html::parse_document(html);
        let mut result = serde_json::Map::new();

        for rule in &self.config.rules {
            let value = self.extract_by_rule(&document, rule)?;
            
            if rule.required && value.is_null() {
                return Err(Error::Parse(format!(
                    "Required field '{}' not found",
                    rule.name
                )));
            }

            result.insert(rule.name.clone(), value);
        }

        if self.config.extract_metadata {
            let metadata = self.extract_metadata(&document)?;
            result.insert("_metadata".to_string(), metadata);
        }

        Ok(Value::Object(result))
    }

    fn extract_by_rule(&self, document: &Html, rule: &ParseRule) -> Result<Value> {
        let selector = Selector::parse(&rule.selector)
            .map_err(|e| Error::Parse(format!("Invalid selector '{}': {:?}", rule.selector, e)))?;

        let elements: Vec<_> = document.select(&selector).collect();

        if elements.is_empty() {
            return Ok(Value::Null);
        }

        if rule.multiple {
            let values: Vec<Value> = elements
                .iter()
                .map(|el| self.extract_value(el, &rule.attribute))
                .collect();
            Ok(Value::Array(values))
        } else {
            Ok(self.extract_value(&elements[0], &rule.attribute))
        }
    }

    fn extract_value(&self, element: &scraper::ElementRef, attribute: &Option<String>) -> Value {
        let text = if let Some(attr) = attribute {
            element.value().attr(attr)
                .map(|s| s.to_string())
                .unwrap_or_default()
        } else {
            element.text().collect::<String>()
        };

        let cleaned = if self.config.clean_text {
            self.clean_text(&text)
        } else {
            text
        };

        Value::String(cleaned)
    }

    fn clean_text(&self, text: &str) -> String {
        text.split_whitespace()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn extract_metadata(&self, document: &Html) -> Result<Value> {
        let mut metadata = serde_json::Map::new();

        let title_selector = Selector::parse("title").unwrap();
        if let Some(title) = document.select(&title_selector).next() {
            metadata.insert(
                "title".to_string(),
                Value::String(title.text().collect::<String>()),
            );
        }

        let meta_selector = Selector::parse("meta[name], meta[property]").unwrap();
        let mut meta_tags = HashMap::new();
        
        for element in document.select(&meta_selector) {
            let name = element.value().attr("name")
                .or_else(|| element.value().attr("property"));
            let content = element.value().attr("content");

            if let (Some(n), Some(c)) = (name, content) {
                meta_tags.insert(n.to_string(), c.to_string());
            }
        }

        if !meta_tags.is_empty() {
            metadata.insert("meta_tags".to_string(), serde_json::to_value(meta_tags)?);
        }

        Ok(Value::Object(metadata))
    }

    pub fn extract_text(&self, html: &str) -> String {
        let document = Html::parse_document(html);
        let body_selector = Selector::parse("body").unwrap();
        
        document
            .select(&body_selector)
            .next()
            .map(|body| body.text().collect::<String>())
            .unwrap_or_default()
    }

    pub fn extract_links(&self, html: &str, base_url: &url::Url) -> Result<Vec<url::Url>> {
        let document = Html::parse_document(html);
        let link_selector = Selector::parse("a[href]").unwrap();
        let mut links = Vec::new();

        for element in document.select(&link_selector) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(url) = base_url.join(href) {
                    links.push(url);
                }
            }
        }

        Ok(links)
    }
}