use crate::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub text: String,
    pub entity_type: EntityType,
    pub confidence: f32,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EntityType {
    Person,
    Organization,
    Location,
    Date,
    Email,
    Phone,
    Url,
    Money,
    Other,
}

pub struct EntityRecognizer;

impl EntityRecognizer {
    pub fn recognize(text: &str) -> Result<Vec<Entity>> {
        let mut entities = Vec::new();

        let email_regex = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
        for mat in email_regex.find_iter(text) {
            entities.push(Entity {
                text: mat.as_str().to_string(),
                entity_type: EntityType::Email,
                confidence: 0.95,
                start: mat.start(),
                end: mat.end(),
            });
        }

        let url_regex = Regex::new(r"https?://[^\s]+").unwrap();
        for mat in url_regex.find_iter(text) {
            entities.push(Entity {
                text: mat.as_str().to_string(),
                entity_type: EntityType::Url,
                confidence: 0.95,
                start: mat.start(),
                end: mat.end(),
            });
        }

        Ok(entities)
    }
}