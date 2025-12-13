//! Get node statistics query

/// Query to get node statistics
#[derive(Debug, Clone)]
pub struct GetNodeStatisticsQuery {
    /// Target node (or "*" for all nodes)
    pub node: String,
}

impl GetNodeStatisticsQuery {
    pub fn new(node: String) -> Self {
        Self { node }
    }
    
    pub fn all_nodes() -> Self {
        Self { node: "*".to_string() }
    }
}
