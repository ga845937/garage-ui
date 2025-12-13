//! Retry block resync command

/// Command to retry block resync
#[derive(Debug, Clone)]
pub struct RetryBlockResyncCommand {
    /// Target node (or "*" for all nodes)
    pub node: String,
    /// Block hashes to retry (None means retry all)
    pub block_hashes: Option<Vec<String>>,
}

impl RetryBlockResyncCommand {
    pub fn new(node: String, block_hashes: Vec<String>) -> Self {
        Self { node, block_hashes: Some(block_hashes) }
    }
    
    pub fn all(node: String) -> Self {
        Self { node, block_hashes: None }
    }
}
