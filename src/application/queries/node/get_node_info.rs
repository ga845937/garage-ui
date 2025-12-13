//! Get node info query

/// Query to get node information
#[derive(Debug, Clone)]
pub struct GetNodeInfoQuery {
    /// Target node (or "*" for all nodes)
    pub node: String,
}

impl GetNodeInfoQuery {
    pub fn new(node: String) -> Self {
        Self { node }
    }
    
    pub fn all_nodes() -> Self {
        Self { node: "*".to_string() }
    }
}
