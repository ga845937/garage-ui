//! List block errors query

/// Query to list block errors
#[derive(Debug, Clone)]
pub struct ListBlockErrorsQuery {
    /// Target node (or "*" for all nodes)
    pub node: String,
}

impl ListBlockErrorsQuery {
    pub fn new(node: String) -> Self {
        Self { node }
    }
    
    pub fn all_nodes() -> Self {
        Self { node: "*".to_string() }
    }
}
