use crate::patterns;
use crate::{Error, Result};
use scraper::Html;
use scraper::Selector;
use serde_json::Value;

pub struct HtmlParser {
    document: Html,
}

impl HtmlParser {
    pub fn new(html: &str) -> Self {
        Self {
            document: Html::parse_document(html),
        }
    }

    pub fn select(&self, selector: &str) -> Result<Vec<String>> {
        let selector = Selector::parse(selector)
            .map_err(|e| Error::Parse(format!("Invalid selector: {e:?}")))?;

        Ok(self
            .document
            .select(&selector)
            .map(|el| el.text().collect::<String>())
            .collect())
    }

    pub fn select_attr(&self, selector: &str, attribute: &str) -> Result<Vec<String>> {
        let selector = Selector::parse(selector)
            .map_err(|e| Error::Parse(format!("Invalid selector: {e:?}")))?;

        Ok(self
            .document
            .select(&selector)
            .filter_map(|el| el.value().attr(attribute).map(String::from))
            .collect())
    }

    /// Extract structured data from JSON-LD scripts and microdata.
    pub fn extract_structured_data(&self) -> Result<Vec<Value>> {
        let mut data = Vec::new();

        // Extract JSON-LD using pre-compiled selector
        for element in self.document.select(&patterns::JSON_LD) {
            let text = element.text().collect::<String>();
            if let Ok(json) = serde_json::from_str::<Value>(&text) {
                data.push(json);
            }
        }

        // Extract microdata using pre-compiled selector
        for element in self.document.select(&patterns::MICRODATA) {
            if let Some(item_data) = self.extract_microdata(&element) {
                data.push(item_data);
            }
        }

        Ok(data)
    }

    fn extract_microdata(&self, element: &scraper::ElementRef) -> Option<Value> {
        let mut item = serde_json::Map::new();

        if let Some(itemtype) = element.value().attr("itemtype") {
            item.insert("@type".to_string(), Value::String(itemtype.to_string()));
        }

        // Use safe selector parsing for dynamic selector
        let prop_selector = patterns::parse_selector("[itemprop]")?;
        for prop_element in element.select(&prop_selector) {
            if let Some(prop_name) = prop_element.value().attr("itemprop") {
                let value = if let Some(content) = prop_element.value().attr("content") {
                    Value::String(content.to_string())
                } else {
                    Value::String(prop_element.text().collect::<String>())
                };
                item.insert(prop_name.to_string(), value);
            }
        }

        if !item.is_empty() {
            Some(Value::Object(item))
        } else {
            None
        }
    }

    /// Extract OpenGraph metadata from the document.
    pub fn extract_opengraph(&self) -> serde_json::Map<String, Value> {
        let mut og_data = serde_json::Map::new();

        // Use pre-compiled selector for OpenGraph meta tags
        for element in self.document.select(&patterns::OG_META) {
            if let (Some(property), Some(content)) = (
                element.value().attr("property"),
                element.value().attr("content"),
            ) {
                let key = property.strip_prefix("og:").unwrap_or(property);
                og_data.insert(key.to_string(), Value::String(content.to_string()));
            }
        }

        og_data
    }

    /// Extract Twitter Card metadata from the document.
    pub fn extract_twitter_card(&self) -> serde_json::Map<String, Value> {
        let mut twitter_data = serde_json::Map::new();

        // Use pre-compiled selector for Twitter meta tags
        for element in self.document.select(&patterns::TWITTER_META) {
            if let (Some(name), Some(content)) = (
                element.value().attr("name"),
                element.value().attr("content"),
            ) {
                let key = name.strip_prefix("twitter:").unwrap_or(name);
                twitter_data.insert(key.to_string(), Value::String(content.to_string()));
            }
        }

        twitter_data
    }
}
