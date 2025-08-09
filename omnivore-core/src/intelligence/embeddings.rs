use crate::Result;

pub struct EmbeddingGenerator {
    dimension: usize,
}

impl EmbeddingGenerator {
    pub fn new(dimension: usize) -> Self {
        Self { dimension }
    }

    pub fn generate(&self, text: &str) -> Result<Vec<f32>> {
        let hash = self.simple_hash(text);
        let mut embedding = vec![0.0f32; self.dimension];
        
        for i in 0..self.dimension {
            embedding[i] = ((hash + i as u64) % 1000) as f32 / 1000.0;
        }

        Ok(embedding)
    }

    fn simple_hash(&self, text: &str) -> u64 {
        text.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64))
    }
}