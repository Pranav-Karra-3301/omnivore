use crate::{Error, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extractor {
    pub name: String,
    pub patterns: Vec<Pattern>,
    pub transformers: Vec<Transformer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub regex: String,
    pub capture_groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Transformer {
    Lowercase,
    Uppercase,
    Trim,
    Replace { from: String, to: String },
    Extract { regex: String },
    Split { delimiter: String },
    Join { delimiter: String },
}

impl Extractor {
    pub fn extract(&self, text: &str) -> Result<Vec<HashMap<String, String>>> {
        let mut results = Vec::new();

        for pattern in &self.patterns {
            let regex = Regex::new(&pattern.regex)
                .map_err(|e| Error::Parse(format!("Invalid regex: {}", e)))?;

            for captures in regex.captures_iter(text) {
                let mut extracted = HashMap::new();

                for (i, group_name) in pattern.capture_groups.iter().enumerate() {
                    if let Some(matched) = captures.get(i + 1) {
                        let value = self.apply_transformers(matched.as_str());
                        extracted.insert(group_name.clone(), value);
                    }
                }

                if !extracted.is_empty() {
                    results.push(extracted);
                }
            }
        }

        Ok(results)
    }

    fn apply_transformers(&self, text: &str) -> String {
        let mut result = text.to_string();

        for transformer in &self.transformers {
            result = match transformer {
                Transformer::Lowercase => result.to_lowercase(),
                Transformer::Uppercase => result.to_uppercase(),
                Transformer::Trim => result.trim().to_string(),
                Transformer::Replace { from, to } => result.replace(from, to),
                Transformer::Extract { regex } => {
                    if let Ok(re) = Regex::new(regex) {
                        if let Some(captures) = re.captures(&result) {
                            captures.get(1)
                                .map(|m| m.as_str().to_string())
                                .unwrap_or(result)
                        } else {
                            result
                        }
                    } else {
                        result
                    }
                }
                Transformer::Split { delimiter } => {
                    result.split(delimiter)
                        .next()
                        .unwrap_or(&result)
                        .to_string()
                }
                Transformer::Join { delimiter } => {
                    result.split_whitespace()
                        .collect::<Vec<_>>()
                        .join(delimiter)
                }
            };
        }

        result
    }
}

pub struct EmailExtractor;

impl EmailExtractor {
    pub fn extract(text: &str) -> Vec<String> {
        let email_regex = Regex::new(
            r"(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}"
        ).unwrap();

        email_regex
            .find_iter(text)
            .map(|m| m.as_str().to_string())
            .collect()
    }
}

pub struct PhoneExtractor;

impl PhoneExtractor {
    pub fn extract(text: &str) -> Vec<String> {
        let phone_regex = Regex::new(
            r"(?:\+?1[-.\s]?)?\(?\d{3}\)?[-.\s]?\d{3}[-.\s]?\d{4}"
        ).unwrap();

        phone_regex
            .find_iter(text)
            .map(|m| m.as_str().to_string())
            .collect()
    }
}

pub struct PriceExtractor;

impl PriceExtractor {
    pub fn extract(text: &str) -> Vec<f64> {
        let price_regex = Regex::new(
            r"(?:\$|USD|EUR|£|€)\s*(\d+(?:[.,]\d{1,2})?)"
        ).unwrap();

        price_regex
            .captures_iter(text)
            .filter_map(|cap| {
                cap.get(1)
                    .and_then(|m| m.as_str().replace(',', "").parse::<f64>().ok())
            })
            .collect()
    }
}

pub struct DateExtractor;

impl DateExtractor {
    pub fn extract(text: &str) -> Vec<String> {
        let date_patterns = vec![
            r"\d{4}-\d{2}-\d{2}",
            r"\d{2}/\d{2}/\d{4}",
            r"\d{1,2}\s+(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)[a-z]*\s+\d{4}",
        ];

        let mut dates = Vec::new();

        for pattern in date_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                for mat in regex.find_iter(text) {
                    dates.push(mat.as_str().to_string());
                }
            }
        }

        dates
    }
}