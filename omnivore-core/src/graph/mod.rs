pub mod builder;
pub mod query;
pub mod schema;

use crate::{Error, Result};
use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub node_type: String,
    pub properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub edge_type: String,
    pub properties: HashMap<String, serde_json::Value>,
}

pub struct KnowledgeGraph {
    graph: DiGraph<Node, Edge>,
    node_index: HashMap<String, NodeIndex>,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_index: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) -> Result<()> {
        if !self.node_index.contains_key(&node.id) {
            let idx = self.graph.add_node(node.clone());
            self.node_index.insert(node.id, idx);
        }
        Ok(())
    }

    pub fn add_edge(&mut self, edge: Edge) -> Result<()> {
        let from_idx = self.node_index.get(&edge.from)
            .ok_or_else(|| Error::Graph(format!("Node {} not found", edge.from)))?;
        let to_idx = self.node_index.get(&edge.to)
            .ok_or_else(|| Error::Graph(format!("Node {} not found", edge.to)))?;
        
        self.graph.add_edge(*from_idx, *to_idx, edge);
        Ok(())
    }

    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.node_index.get(id)
            .and_then(|idx| self.graph.node_weight(*idx))
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
}