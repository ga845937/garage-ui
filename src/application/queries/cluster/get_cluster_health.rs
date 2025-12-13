//! Get cluster health query

/// Query to get cluster health
#[derive(Debug, Clone, Default)]
pub struct GetClusterHealthQuery;

impl GetClusterHealthQuery {
    pub fn new() -> Self {
        Self
    }
}
