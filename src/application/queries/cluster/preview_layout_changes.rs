//! Preview layout changes query

/// Query to preview layout changes without applying
#[derive(Debug, Clone, Default)]
pub struct PreviewLayoutChangesQuery;

impl PreviewLayoutChangesQuery {
    pub fn new() -> Self {
        Self
    }
}
