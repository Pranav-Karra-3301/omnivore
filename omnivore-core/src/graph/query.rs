use crate::graph::{KnowledgeGraph, Node};
use std::collections::HashSet;

/// Query interface for the knowledge graph.
pub struct GraphQuery<'a> {
    graph: &'a KnowledgeGraph,
}

impl<'a> GraphQuery<'a> {
    /// Create a new query interface for the given graph.
    pub fn new(graph: &'a KnowledgeGraph) -> Self {
        Self { graph }
    }

    /// Find all nodes of a specific type.
    ///
    /// # Arguments
    /// * `node_type` - The type of nodes to find (e.g., "Person", "Organization", "URL")
    ///
    /// # Returns
    /// A vector of references to nodes matching the specified type.
    pub fn find_by_type(&self, node_type: &str) -> Vec<&'a Node> {
        self.graph
            .graph
            .node_weights()
            .filter(|node| node.node_type == node_type)
            .collect()
    }

    /// Find all nodes connected to a given node within a specified depth.
    ///
    /// Uses breadth-first search to traverse the graph from the starting node.
    ///
    /// # Arguments
    /// * `node_id` - The ID of the starting node
    /// * `max_depth` - Maximum traversal depth (0 = only the node itself)
    ///
    /// # Returns
    /// A vector of references to connected nodes, excluding the starting node.
    pub fn find_connected(&self, node_id: &str, max_depth: usize) -> Vec<&'a Node> {
        let mut results = Vec::new();

        // Get the starting node index
        let start_idx = match self.graph.node_index.get(node_id) {
            Some(idx) => *idx,
            None => return results, // Node not found
        };

        if max_depth == 0 {
            // Only return the node itself
            if let Some(node) = self.graph.graph.node_weight(start_idx) {
                return vec![node];
            }
            return results;
        }

        // Track visited nodes and their distances
        let mut visited: HashSet<petgraph::graph::NodeIndex> = HashSet::new();
        let mut current_depth_nodes = vec![start_idx];
        visited.insert(start_idx);

        for _ in 0..max_depth {
            let mut next_depth_nodes = Vec::new();

            for node_idx in current_depth_nodes {
                // Get all neighbors (both incoming and outgoing edges)
                for neighbor_idx in self.graph.graph.neighbors_undirected(node_idx) {
                    if !visited.contains(&neighbor_idx) {
                        visited.insert(neighbor_idx);
                        next_depth_nodes.push(neighbor_idx);

                        if let Some(node) = self.graph.graph.node_weight(neighbor_idx) {
                            results.push(node);
                        }
                    }
                }
            }

            if next_depth_nodes.is_empty() {
                break; // No more nodes to explore
            }

            current_depth_nodes = next_depth_nodes;
        }

        results
    }

    /// Find all nodes matching a property value.
    ///
    /// # Arguments
    /// * `property_name` - The name of the property to search
    /// * `property_value` - The value to match (as JSON string)
    ///
    /// # Returns
    /// A vector of references to nodes with matching property values.
    pub fn find_by_property(&self, property_name: &str, property_value: &str) -> Vec<&'a Node> {
        self.graph
            .graph
            .node_weights()
            .filter(|node| {
                node.properties
                    .get(property_name)
                    .map(|v| {
                        // Compare as string for simplicity
                        match v {
                            serde_json::Value::String(s) => s == property_value,
                            _ => *v == property_value,
                        }
                    })
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Get all unique node types in the graph.
    pub fn get_node_types(&self) -> Vec<String> {
        let mut types: HashSet<String> = HashSet::new();
        for node in self.graph.graph.node_weights() {
            types.insert(node.node_type.clone());
        }
        types.into_iter().collect()
    }

    /// Get all unique edge types in the graph.
    pub fn get_edge_types(&self) -> Vec<String> {
        let mut types: HashSet<String> = HashSet::new();
        for edge in self.graph.graph.edge_weights() {
            types.insert(edge.edge_type.clone());
        }
        types.into_iter().collect()
    }

    /// Count nodes by type.
    ///
    /// # Returns
    /// A HashMap mapping node types to their counts.
    pub fn count_by_type(&self) -> std::collections::HashMap<String, usize> {
        let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for node in self.graph.graph.node_weights() {
            *counts.entry(node.node_type.clone()).or_insert(0) += 1;
        }
        counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{Edge, KnowledgeGraph, Node};
    use std::collections::HashMap;

    fn create_test_graph() -> KnowledgeGraph {
        let mut graph = KnowledgeGraph::new();

        // Add nodes
        graph
            .add_node(Node {
                id: "person1".to_string(),
                node_type: "Person".to_string(),
                properties: {
                    let mut props = HashMap::new();
                    props.insert("name".to_string(), serde_json::json!("Alice"));
                    props
                },
            })
            .unwrap();

        graph
            .add_node(Node {
                id: "person2".to_string(),
                node_type: "Person".to_string(),
                properties: {
                    let mut props = HashMap::new();
                    props.insert("name".to_string(), serde_json::json!("Bob"));
                    props
                },
            })
            .unwrap();

        graph
            .add_node(Node {
                id: "org1".to_string(),
                node_type: "Organization".to_string(),
                properties: {
                    let mut props = HashMap::new();
                    props.insert("name".to_string(), serde_json::json!("Acme Corp"));
                    props
                },
            })
            .unwrap();

        // Add edges
        graph
            .add_edge(Edge {
                from: "person1".to_string(),
                to: "org1".to_string(),
                edge_type: "works_at".to_string(),
                properties: HashMap::new(),
            })
            .unwrap();

        graph
            .add_edge(Edge {
                from: "person2".to_string(),
                to: "org1".to_string(),
                edge_type: "works_at".to_string(),
                properties: HashMap::new(),
            })
            .unwrap();

        graph
    }

    #[test]
    fn test_find_by_type() {
        let graph = create_test_graph();
        let query = GraphQuery::new(&graph);

        let people = query.find_by_type("Person");
        assert_eq!(people.len(), 2);

        let orgs = query.find_by_type("Organization");
        assert_eq!(orgs.len(), 1);

        let empty = query.find_by_type("NonExistent");
        assert!(empty.is_empty());
    }

    #[test]
    fn test_find_connected() {
        let graph = create_test_graph();
        let query = GraphQuery::new(&graph);

        // person1 is connected to org1
        let connected = query.find_connected("person1", 1);
        assert_eq!(connected.len(), 1);
        assert_eq!(connected[0].node_type, "Organization");

        // org1 is connected to person1 and person2
        let connected_to_org = query.find_connected("org1", 1);
        assert_eq!(connected_to_org.len(), 2);
    }

    #[test]
    fn test_find_connected_nonexistent() {
        let graph = create_test_graph();
        let query = GraphQuery::new(&graph);

        let connected = query.find_connected("nonexistent", 1);
        assert!(connected.is_empty());
    }

    #[test]
    fn test_find_by_property() {
        let graph = create_test_graph();
        let query = GraphQuery::new(&graph);

        let alice = query.find_by_property("name", "Alice");
        assert_eq!(alice.len(), 1);
        assert_eq!(alice[0].id, "person1");
    }

    #[test]
    fn test_get_node_types() {
        let graph = create_test_graph();
        let query = GraphQuery::new(&graph);

        let types = query.get_node_types();
        assert_eq!(types.len(), 2);
        assert!(types.contains(&"Person".to_string()));
        assert!(types.contains(&"Organization".to_string()));
    }

    #[test]
    fn test_count_by_type() {
        let graph = create_test_graph();
        let query = GraphQuery::new(&graph);

        let counts = query.count_by_type();
        assert_eq!(*counts.get("Person").unwrap(), 2);
        assert_eq!(*counts.get("Organization").unwrap(), 1);
    }
}
