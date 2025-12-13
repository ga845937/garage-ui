//! Revert layout command

/// Command to revert staged layout changes
#[derive(Debug, Clone, Default)]
pub struct RevertLayoutCommand;

impl RevertLayoutCommand {
    pub fn new() -> Self {
        Self
    }
}
