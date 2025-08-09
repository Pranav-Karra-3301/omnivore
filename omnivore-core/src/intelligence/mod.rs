pub mod embeddings;
pub mod entity;
pub mod relations;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceConfig {
    pub enable_entity_recognition: bool,
    pub enable_relation_extraction: bool,
    pub enable_embeddings: bool,
    pub embedding_model: EmbeddingModel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmbeddingModel {
    Local,
    OpenAI,
    Anthropic,
}

impl Default for IntelligenceConfig {
    fn default() -> Self {
        Self {
            enable_entity_recognition: true,
            enable_relation_extraction: true,
            enable_embeddings: false,
            embedding_model: EmbeddingModel::Local,
        }
    }
}