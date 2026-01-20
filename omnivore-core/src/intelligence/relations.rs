//! Relation extraction from natural language text.
//!
//! This module extracts structured relations (subject-predicate-object triples)
//! from unstructured text using pattern matching.

use crate::patterns;
use crate::Result;
use serde::{Deserialize, Serialize};

/// A relation extracted from text, represented as a triple.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Relation {
    /// The subject of the relation (e.g., "Apple")
    pub subject: String,
    /// The predicate/relationship type (e.g., "founded_by")
    pub predicate: String,
    /// The object of the relation (e.g., "Steve Jobs")
    pub object: String,
    /// Confidence score from 0.0 to 1.0
    pub confidence: f32,
}

/// Extracts relations from text using pattern matching.
pub struct RelationExtractor;

impl RelationExtractor {
    /// Extract relations from the given text.
    ///
    /// # Arguments
    /// * `text` - The text to extract relations from
    ///
    /// # Returns
    /// A vector of extracted relations with confidence scores.
    ///
    /// # Example
    /// ```
    /// use omnivore_core::intelligence::relations::RelationExtractor;
    ///
    /// let text = "Apple was founded by Steve Jobs. The company is located in Cupertino.";
    /// let relations = RelationExtractor::extract(text).unwrap();
    /// ```
    pub fn extract(text: &str) -> Result<Vec<Relation>> {
        let mut relations = Vec::new();

        // Split text into sentences for better extraction
        let sentences: Vec<&str> = text
            .split(['.', '!', '?'])
            .filter(|s| !s.trim().is_empty())
            .collect();

        for sentence in sentences {
            // Extract "is_a" relations (X is a Y)
            relations.extend(Self::extract_is_a(sentence));

            // Extract "has" relations (X has Y)
            relations.extend(Self::extract_has(sentence));

            // Extract employment relations (X works at Y)
            relations.extend(Self::extract_works_at(sentence));

            // Extract founder relations (X founded Y)
            relations.extend(Self::extract_founded(sentence));

            // Extract location relations (X located in Y)
            relations.extend(Self::extract_located_in(sentence));

            // Extract part_of relations (X is part of Y)
            relations.extend(Self::extract_part_of(sentence));

            // Extract acquisition relations (X acquired Y)
            relations.extend(Self::extract_acquired(sentence));

            // Extract role relations (X CEO of Y)
            relations.extend(Self::extract_role_of(sentence));

            // Extract family relations
            relations.extend(Self::extract_family_relation(sentence));

            // Extract creator relations (X invented Y)
            relations.extend(Self::extract_created(sentence));
        }

        // Deduplicate relations
        relations = Self::deduplicate(relations);

        Ok(relations)
    }

    /// Extract "is_a" relations (e.g., "Python is a programming language")
    fn extract_is_a(text: &str) -> Vec<Relation> {
        patterns::IS_A
            .captures_iter(text)
            .filter_map(|cap| {
                let subject = cap.get(1)?.as_str().trim().to_string();
                let object = cap.get(2)?.as_str().trim().to_string();

                // Filter out very short or generic extractions
                if subject.len() < 2 || object.len() < 2 {
                    return None;
                }

                Some(Relation {
                    subject,
                    predicate: "is_a".to_string(),
                    object,
                    confidence: 0.85,
                })
            })
            .collect()
    }

    /// Extract "has" relations (e.g., "The company has many employees")
    fn extract_has(text: &str) -> Vec<Relation> {
        patterns::HAS
            .captures_iter(text)
            .filter_map(|cap| {
                let subject = cap.get(1)?.as_str().trim().to_string();
                let object = cap.get(2)?.as_str().trim().to_string();

                if subject.len() < 2 || object.len() < 2 {
                    return None;
                }

                Some(Relation {
                    subject,
                    predicate: "has".to_string(),
                    object,
                    confidence: 0.75,
                })
            })
            .collect()
    }

    /// Extract employment relations (e.g., "John works at Google")
    fn extract_works_at(text: &str) -> Vec<Relation> {
        patterns::WORKS_AT
            .captures_iter(text)
            .filter_map(|cap| {
                let subject = cap.get(1)?.as_str().trim().to_string();
                let object = cap.get(2)?.as_str().trim().to_string();

                if subject.len() < 2 || object.len() < 2 {
                    return None;
                }

                Some(Relation {
                    subject,
                    predicate: "works_at".to_string(),
                    object,
                    confidence: 0.90,
                })
            })
            .collect()
    }

    /// Extract founder relations (e.g., "Steve Jobs founded Apple")
    fn extract_founded(text: &str) -> Vec<Relation> {
        patterns::FOUNDED
            .captures_iter(text)
            .filter_map(|cap| {
                let subject = cap.get(1)?.as_str().trim().to_string();
                let object = cap.get(2)?.as_str().trim().to_string();

                if subject.len() < 2 || object.len() < 2 {
                    return None;
                }

                Some(Relation {
                    subject,
                    predicate: "founded".to_string(),
                    object,
                    confidence: 0.90,
                })
            })
            .collect()
    }

    /// Extract location relations (e.g., "Apple is located in Cupertino")
    fn extract_located_in(text: &str) -> Vec<Relation> {
        patterns::LOCATED_IN
            .captures_iter(text)
            .filter_map(|cap| {
                let subject = cap.get(1)?.as_str().trim().to_string();
                let object = cap.get(2)?.as_str().trim().to_string();

                if subject.len() < 2 || object.len() < 2 {
                    return None;
                }

                Some(Relation {
                    subject,
                    predicate: "located_in".to_string(),
                    object,
                    confidence: 0.85,
                })
            })
            .collect()
    }

    /// Extract part_of relations (e.g., "Chrome is part of Google")
    fn extract_part_of(text: &str) -> Vec<Relation> {
        patterns::PART_OF
            .captures_iter(text)
            .filter_map(|cap| {
                let subject = cap.get(1)?.as_str().trim().to_string();
                let object = cap.get(2)?.as_str().trim().to_string();

                if subject.len() < 2 || object.len() < 2 {
                    return None;
                }

                Some(Relation {
                    subject,
                    predicate: "part_of".to_string(),
                    object,
                    confidence: 0.85,
                })
            })
            .collect()
    }

    /// Extract acquisition relations (e.g., "Microsoft acquired LinkedIn")
    fn extract_acquired(text: &str) -> Vec<Relation> {
        patterns::ACQUIRED
            .captures_iter(text)
            .filter_map(|cap| {
                let subject = cap.get(1)?.as_str().trim().to_string();
                let object = cap.get(2)?.as_str().trim().to_string();

                if subject.len() < 2 || object.len() < 2 {
                    return None;
                }

                Some(Relation {
                    subject,
                    predicate: "acquired".to_string(),
                    object,
                    confidence: 0.90,
                })
            })
            .collect()
    }

    /// Extract role relations (e.g., "Sundar Pichai, CEO of Google")
    fn extract_role_of(text: &str) -> Vec<Relation> {
        patterns::ROLE_OF
            .captures_iter(text)
            .filter_map(|cap| {
                let subject = cap.get(1)?.as_str().trim().to_string();
                let object = cap.get(2)?.as_str().trim().to_string();

                if subject.len() < 2 || object.len() < 2 {
                    return None;
                }

                Some(Relation {
                    subject,
                    predicate: "role_at".to_string(),
                    object,
                    confidence: 0.90,
                })
            })
            .collect()
    }

    /// Extract family relations (e.g., "John is the father of Mary")
    fn extract_family_relation(text: &str) -> Vec<Relation> {
        patterns::FAMILY_RELATION
            .captures_iter(text)
            .filter_map(|cap| {
                let subject = cap.get(1)?.as_str().trim().to_string();
                let object = cap.get(2)?.as_str().trim().to_string();

                if subject.len() < 2 || object.len() < 2 {
                    return None;
                }

                Some(Relation {
                    subject,
                    predicate: "family_relation".to_string(),
                    object,
                    confidence: 0.80,
                })
            })
            .collect()
    }

    /// Extract creator relations (e.g., "Tim Berners-Lee invented the World Wide Web")
    fn extract_created(text: &str) -> Vec<Relation> {
        patterns::CREATED
            .captures_iter(text)
            .filter_map(|cap| {
                let subject = cap.get(1)?.as_str().trim().to_string();
                let object = cap.get(2)?.as_str().trim().to_string();

                if subject.len() < 2 || object.len() < 2 {
                    return None;
                }

                Some(Relation {
                    subject,
                    predicate: "created".to_string(),
                    object,
                    confidence: 0.85,
                })
            })
            .collect()
    }

    /// Deduplicate relations, keeping the one with highest confidence
    fn deduplicate(relations: Vec<Relation>) -> Vec<Relation> {
        use std::collections::HashMap;

        let mut seen: HashMap<(String, String, String), Relation> = HashMap::new();

        for relation in relations {
            let key = (
                relation.subject.to_lowercase(),
                relation.predicate.clone(),
                relation.object.to_lowercase(),
            );

            seen.entry(key)
                .and_modify(|existing| {
                    if relation.confidence > existing.confidence {
                        *existing = relation.clone();
                    }
                })
                .or_insert(relation);
        }

        seen.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_is_a() {
        let text = "Python is a programming language";
        let relations = RelationExtractor::extract(text).unwrap();
        assert!(!relations.is_empty());
        let rel = relations.iter().find(|r| r.predicate == "is_a");
        assert!(rel.is_some());
    }

    #[test]
    fn test_extract_works_at() {
        let text = "John Smith works at Microsoft";
        let relations = RelationExtractor::extract(text).unwrap();
        let rel = relations
            .iter()
            .find(|r| r.predicate == "works_at")
            .expect("Should find works_at relation");
        assert!(rel.subject.contains("John"));
        assert!(rel.object.contains("Microsoft"));
    }

    #[test]
    fn test_extract_founded() {
        let text = "Steve Jobs founded Apple in his garage";
        let relations = RelationExtractor::extract(text).unwrap();
        let rel = relations.iter().find(|r| r.predicate == "founded");
        assert!(rel.is_some());
        let rel = rel.unwrap();
        assert!(rel.subject.contains("Steve Jobs"));
    }

    #[test]
    fn test_extract_located_in() {
        let text = "Apple Inc is headquartered in Cupertino, California";
        let relations = RelationExtractor::extract(text).unwrap();
        let rel = relations.iter().find(|r| r.predicate == "located_in");
        assert!(rel.is_some());
    }

    #[test]
    fn test_extract_acquired() {
        let text = "Microsoft acquired LinkedIn for 26 billion dollars";
        let relations = RelationExtractor::extract(text).unwrap();
        let rel = relations.iter().find(|r| r.predicate == "acquired");
        assert!(rel.is_some());
        let rel = rel.unwrap();
        assert!(rel.subject.contains("Microsoft"));
        assert!(rel.object.contains("LinkedIn"));
    }

    #[test]
    fn test_extract_role_of() {
        let text = "Satya Nadella is the CEO of Microsoft";
        let relations = RelationExtractor::extract(text).unwrap();
        let rel = relations.iter().find(|r| r.predicate == "role_at");
        assert!(rel.is_some());
    }

    #[test]
    fn test_extract_created() {
        let text = "Tim Berners Lee invented the World Wide Web";
        let relations = RelationExtractor::extract(text).unwrap();
        let rel = relations.iter().find(|r| r.predicate == "created");
        assert!(rel.is_some());
    }

    #[test]
    fn test_extract_multiple_relations() {
        let text = "Apple was founded by Steve Jobs. The company is located in Cupertino. Microsoft acquired LinkedIn.";
        let relations = RelationExtractor::extract(text).unwrap();
        assert!(relations.len() >= 2);
    }

    #[test]
    fn test_empty_text() {
        let relations = RelationExtractor::extract("").unwrap();
        assert!(relations.is_empty());
    }

    #[test]
    fn test_no_relations() {
        let text = "The weather is nice today.";
        let relations = RelationExtractor::extract(text).unwrap();
        // May or may not extract relations from generic text
        // Just ensure no panic
        let _ = relations;
    }

    #[test]
    fn test_deduplication() {
        let text = "Apple founded Apple. Apple founded Apple.";
        let relations = RelationExtractor::extract(text).unwrap();
        // Should deduplicate identical relations
        let founded_count = relations
            .iter()
            .filter(|r| r.predicate == "founded")
            .count();
        assert!(founded_count <= 1);
    }
}
