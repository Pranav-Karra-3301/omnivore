use crate::Result;

pub struct VectorStore {}

impl VectorStore {
    pub fn new(_dimension: usize) -> Self {
        Self {}
    }

    pub fn insert(&mut self, _id: String, _vector: Vec<f32>) -> Result<()> {
        Ok(())
    }

    pub fn search(&self, _query: Vec<f32>, _k: usize) -> Result<Vec<(String, f32)>> {
        Ok(Vec::new())
    }
}