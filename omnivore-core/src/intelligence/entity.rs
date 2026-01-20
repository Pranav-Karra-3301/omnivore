use crate::patterns;
use crate::Result;
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
    /// Recognize entities in the given text.
    ///
    /// Currently supports:
    /// - Email addresses
    /// - URLs
    /// - Phone numbers
    /// - Dates
    /// - Currency/money
    pub fn recognize(text: &str) -> Result<Vec<Entity>> {
        let mut entities = Vec::new();

        // Extract emails using pre-compiled regex
        for mat in patterns::EMAIL.find_iter(text) {
            entities.push(Entity {
                text: mat.as_str().to_string(),
                entity_type: EntityType::Email,
                confidence: 0.95,
                start: mat.start(),
                end: mat.end(),
            });
        }

        // Extract URLs using pre-compiled regex
        for mat in patterns::URL.find_iter(text) {
            entities.push(Entity {
                text: mat.as_str().to_string(),
                entity_type: EntityType::Url,
                confidence: 0.95,
                start: mat.start(),
                end: mat.end(),
            });
        }

        // Extract phone numbers using pre-compiled regex
        for mat in patterns::PHONE.find_iter(text) {
            entities.push(Entity {
                text: mat.as_str().to_string(),
                entity_type: EntityType::Phone,
                confidence: 0.85, // Lower confidence due to varied formats
                start: mat.start(),
                end: mat.end(),
            });
        }

        // Extract dates using pre-compiled regex
        for mat in patterns::DATE.find_iter(text) {
            entities.push(Entity {
                text: mat.as_str().to_string(),
                entity_type: EntityType::Date,
                confidence: 0.80, // Moderate confidence
                start: mat.start(),
                end: mat.end(),
            });
        }

        // Extract money/currency using pre-compiled regex
        for mat in patterns::PRICE.find_iter(text) {
            entities.push(Entity {
                text: mat.as_str().to_string(),
                entity_type: EntityType::Money,
                confidence: 0.90,
                start: mat.start(),
                end: mat.end(),
            });
        }

        Ok(entities)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_recognition() {
        let text = "Contact us at test@example.com for more info.";
        let entities = EntityRecognizer::recognize(text).unwrap();

        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].text, "test@example.com");
        assert!(matches!(entities[0].entity_type, EntityType::Email));
    }

    #[test]
    fn test_url_recognition() {
        let text = "Visit https://example.com for details.";
        let entities = EntityRecognizer::recognize(text).unwrap();

        assert!(entities
            .iter()
            .any(|e| matches!(e.entity_type, EntityType::Url)));
    }

    #[test]
    fn test_phone_recognition() {
        let text = "Call us at 555-123-4567 or (800) 555-1234.";
        let entities = EntityRecognizer::recognize(text).unwrap();

        let phone_entities: Vec<_> = entities
            .iter()
            .filter(|e| matches!(e.entity_type, EntityType::Phone))
            .collect();
        assert!(!phone_entities.is_empty());
    }

    #[test]
    fn test_date_recognition() {
        let text = "The event is on January 15, 2024 and ends 2024-01-20.";
        let entities = EntityRecognizer::recognize(text).unwrap();

        let date_entities: Vec<_> = entities
            .iter()
            .filter(|e| matches!(e.entity_type, EntityType::Date))
            .collect();
        assert!(!date_entities.is_empty());
    }

    #[test]
    fn test_money_recognition() {
        let text = "The price is $19.99 or 100 USD.";
        let entities = EntityRecognizer::recognize(text).unwrap();

        let money_entities: Vec<_> = entities
            .iter()
            .filter(|e| matches!(e.entity_type, EntityType::Money))
            .collect();
        assert!(!money_entities.is_empty());
    }

    #[test]
    fn test_multiple_entities() {
        let text = "Email john@example.com or visit https://example.com";
        let entities = EntityRecognizer::recognize(text).unwrap();

        assert!(entities.len() >= 2);
    }
}
