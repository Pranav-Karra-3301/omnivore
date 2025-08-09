use crate::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub confidence: f32,
}

pub struct RelationExtractor;

impl RelationExtractor {
    pub fn extract(text: &str) -> Result<Vec<Relation>> {
        let relations = Vec::new();
        Ok(relations)
    }
}