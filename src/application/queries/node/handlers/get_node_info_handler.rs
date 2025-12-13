//! Get node info query handler

use std::sync::Arc;
use crate::application::queries::node::GetNodeInfoQuery;
use crate::domain::entities::{MultiNodeResponse, NodeInfo};
use crate::domain::errors::DomainError;
use crate::domain::repositories::NodeRepository;

/// Handler for getting node information
pub struct GetNodeInfoHandler {
    repository: Arc<dyn NodeRepository>,
}

impl GetNodeInfoHandler {
    pub fn new(repository: Arc<dyn NodeRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, query: GetNodeInfoQuery) -> Result<MultiNodeResponse<NodeInfo>, DomainError> {
        self.repository.get_info(&query.node).await
    }
}
