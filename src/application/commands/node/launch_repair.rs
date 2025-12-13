//! Launch repair command

/// Command to launch a repair operation
#[derive(Debug, Clone)]
pub struct LaunchRepairCommand {
    /// Target node (or "*" for all nodes)
    pub node: String,
    /// Repair type (e.g., "tables", "blocks", "versions", "block_refs", "block_rc", "scrub")
    pub repair_type: String,
}

impl LaunchRepairCommand {
    pub fn new(node: String, repair_type: String) -> Self {
        Self { node, repair_type }
    }
}
