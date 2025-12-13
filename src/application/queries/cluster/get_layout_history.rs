//! Get layout history query

/// Query to get cluster layout history
#[derive(Debug, Clone, Default)]
pub struct GetLayoutHistoryQuery;

impl GetLayoutHistoryQuery {
    pub fn new() -> Self {
        Self
    }
}
