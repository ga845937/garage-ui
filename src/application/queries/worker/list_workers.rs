//! List workers query

/// Query to list workers
#[derive(Debug, Clone)]
pub struct ListWorkersQuery {
    /// Target node (or "*" for all nodes)
    pub node: String,
    /// Only show busy workers
    pub busy_only: bool,
    /// Only show workers with errors
    pub error_only: bool,
}

impl ListWorkersQuery {
    pub fn new(node: String) -> Self {
        Self { 
            node,
            busy_only: false,
            error_only: false,
        }
    }
    
    pub fn all_nodes() -> Self {
        Self::new("*".to_string())
    }
    
    pub fn busy_only(mut self) -> Self {
        self.busy_only = true;
        self
    }
    
    pub fn error_only(mut self) -> Self {
        self.error_only = true;
        self
    }
}
