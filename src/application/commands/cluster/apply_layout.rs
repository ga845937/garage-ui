//! Apply layout command

/// Command to apply cluster layout changes
#[derive(Debug, Clone)]
pub struct ApplyLayoutCommand {
    /// Expected cluster layout version
    pub version: i64,
}

impl ApplyLayoutCommand {
    pub fn new(version: i64) -> Self {
        Self { version }
    }
}
