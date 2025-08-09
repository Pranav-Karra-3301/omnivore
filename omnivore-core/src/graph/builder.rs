use crate::graph::{Edge, KnowledgeGraph, Node};
use crate::Result;
use std::collections::HashMap;

pub struct GraphBuilder {
    graph: KnowledgeGraph,
}

impl GraphBuilder {
    pub fn new() -> Self {
        Self {
            graph: KnowledgeGraph::new(),
        }
    }

    pub fn add_entity(
        &mut self,
        id: String,
        entity_type: String,
        properties: HashMap<String, serde_json::Value>,
    ) -> Result<()> {
        let node = Node {
            id,
            node_type: entity_type,
            properties,
        };
        self.graph.add_node(node)
    }

    pub fn add_relationship(
        &mut self,
        from: String,
        to: String,
        rel_type: String,
        properties: HashMap<String, serde_json::Value>,
    ) -> Result<()> {
        let edge = Edge {
            from,
            to,
            edge_type: rel_type,
            properties,
        };
        self.graph.add_edge(edge)
    }

    pub fn build(self) -> KnowledgeGraph {
        self.graph
    }
}
