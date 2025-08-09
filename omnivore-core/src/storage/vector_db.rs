use crate::{Error, Result};

pub struct VectorStore {
    dimension: usize,
}

impl VectorStore {
    pub fn new(dimension: usize) -> Self {
        Self { dimension }
    }

    pub fn insert(&mut self, _id: String, _vector: Vec<f32>) -> Result<()> {
        Ok(())
    }

    pub fn search(&self, _query: Vec<f32>, _k: usize) -> Result<Vec<(String, f32)>> {
        Ok(Vec::new())
    }
}