use crate::Result;

pub struct GraphDatabase;

impl Default for GraphDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphDatabase {
    pub fn new() -> Self {
        Self
    }

    pub async fn connect(&self, _connection_string: &str) -> Result<()> {
        Ok(())
    }

    pub async fn execute(&self, _query: &str) -> Result<()> {
        Ok(())
    }
}
