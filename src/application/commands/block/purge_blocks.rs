//! Purge blocks command

/// Command to purge orphaned blocks
#[derive(Debug, Clone)]
pub struct PurgeBlocksCommand {
    /// Target node (or "*" for all nodes)
    pub node: String,
    /// Block hashes to purge
    pub block_hashes: Vec<String>,
}

impl PurgeBlocksCommand {
    pub fn new(node: String, block_hashes: Vec<String>) -> Self {
        Self { node, block_hashes }
    }
}
