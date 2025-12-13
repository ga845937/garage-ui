//! Get worker info query

/// Query to get worker information
#[derive(Debug, Clone)]
pub struct GetWorkerInfoQuery {
    /// Target node (or "*" for all nodes)
    pub node: String,
    /// Worker ID
    pub worker_id: i64,
}

impl GetWorkerInfoQuery {
    pub fn new(node: String, worker_id: i64) -> Self {
        Self { node, worker_id }
    }
}
