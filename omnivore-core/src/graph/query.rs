use crate::graph::{KnowledgeGraph, Node};

pub struct GraphQuery<'a> {
    _graph: &'a KnowledgeGraph,
}

impl<'a> GraphQuery<'a> {
    pub fn new(graph: &'a KnowledgeGraph) -> Self {
        Self { _graph: graph }
    }

    pub fn find_by_type(&self, _node_type: &str) -> Vec<&'a Node> {
        Vec::new()
    }

    pub fn find_connected(&self, _node_id: &str, _max_depth: usize) -> Vec<&'a Node> {
        Vec::new()
    }
}