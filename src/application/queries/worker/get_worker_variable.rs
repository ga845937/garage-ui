//! Get worker variable query

/// Query to get worker variable(s)
#[derive(Debug, Clone)]
pub struct GetWorkerVariableQuery {
    /// Target node (or "*" for all nodes)
    pub node: String,
    /// Variable name (None for all variables)
    pub variable: Option<String>,
}

impl GetWorkerVariableQuery {
    pub fn new(node: String, variable: Option<String>) -> Self {
        Self { node, variable }
    }
    
    pub fn all_variables(node: String) -> Self {
        Self { node, variable: None }
    }
}
