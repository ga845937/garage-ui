//! Create metadata snapshot command

/// Command to create a metadata snapshot
#[derive(Debug, Clone)]
pub struct CreateMetadataSnapshotCommand {
    /// Target node (or "*" for all nodes)
    pub node: String,
}

impl CreateMetadataSnapshotCommand {
    pub fn new(node: String) -> Self {
        Self { node }
    }
    
    pub fn all_nodes() -> Self {
        Self { node: "*".to_string() }
    }
}
