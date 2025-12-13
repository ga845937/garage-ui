//! Get block info query

/// Query to get block information
#[derive(Debug, Clone)]
pub struct GetBlockInfoQuery {
    /// Target node (or "*" for all nodes)
    pub node: String,
    /// Block hash
    pub block_hash: String,
}

impl GetBlockInfoQuery {
    pub fn new(node: String, block_hash: String) -> Self {
        Self { node, block_hash }
    }
}
