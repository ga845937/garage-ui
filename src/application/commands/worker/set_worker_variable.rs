//! Set worker variable command

/// Command to set a worker variable
#[derive(Debug, Clone)]
pub struct SetWorkerVariableCommand {
    /// Target node (or "*" for all nodes)
    pub node: String,
    /// Variable name
    pub variable: String,
    /// Variable value
    pub value: String,
}

impl SetWorkerVariableCommand {
    pub fn new(node: String, variable: String, value: String) -> Self {
        Self { node, variable, value }
    }
}
