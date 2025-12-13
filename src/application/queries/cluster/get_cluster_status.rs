//! Get cluster status query

/// Query to get cluster status
#[derive(Debug, Clone, Default)]
pub struct GetClusterStatusQuery;

impl GetClusterStatusQuery {
    pub fn new() -> Self {
        Self
    }
}
