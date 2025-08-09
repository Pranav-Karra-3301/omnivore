use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSchema {
    pub name: String,
    pub version: String,
    pub node_types: HashMap<String, NodeTypeSchema>,
    pub edge_types: HashMap<String, EdgeTypeSchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeTypeSchema {
    pub name: String,
    pub properties: HashMap<String, PropertySchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeTypeSchema {
    pub name: String,
    pub from_types: Vec<String>,
    pub to_types: Vec<String>,
    pub properties: HashMap<String, PropertySchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertySchema {
    pub property_type: String,
    pub required: bool,
    pub description: Option<String>,
}