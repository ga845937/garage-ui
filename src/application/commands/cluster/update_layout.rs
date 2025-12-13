//! Update layout command

/// Command to update cluster layout
#[derive(Debug, Clone)]
pub struct UpdateLayoutCommand {
    /// Layout role changes
    pub role_changes: Vec<LayoutRoleChange>,
}

/// Layout role change
#[derive(Debug, Clone)]
pub struct LayoutRoleChange {
    /// Node ID
    pub node_id: String,
    /// Zone name
    pub zone: Option<String>,
    /// Storage capacity in bytes
    pub capacity: Option<i64>,
    /// Node tags
    pub tags: Option<Vec<String>>,
    /// Whether to remove the node
    pub remove: bool,
}

impl UpdateLayoutCommand {
    pub fn new(role_changes: Vec<LayoutRoleChange>) -> Self {
        Self { role_changes }
    }
}
