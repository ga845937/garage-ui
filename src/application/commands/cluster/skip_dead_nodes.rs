//! Skip dead nodes command

/// Command to skip dead nodes in layout updates
#[derive(Debug, Clone)]
pub struct SkipDeadNodesCommand {
    /// Expected cluster layout version
    pub version: i64,
    /// Allow missing data (skip dead nodes even if data may be lost)
    pub allow_missing_data: bool,
}

impl SkipDeadNodesCommand {
    pub fn new(version: i64, allow_missing_data: bool) -> Self {
        Self { version, allow_missing_data }
    }
}
