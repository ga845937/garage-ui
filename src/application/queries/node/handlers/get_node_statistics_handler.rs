//! Get node statistics query handler

use std::sync::Arc;
use crate::application::queries::node::GetNodeStatisticsQuery;
use crate::domain::entities::{MultiNodeResponse, NodeStatistics};
use crate::domain::errors::DomainError;
use crate::domain::repositories::NodeRepository;

/// Handler for getting node statistics
pub struct GetNodeStatisticsHandler {
    repository: Arc<dyn NodeRepository>,
}

impl GetNodeStatisticsHandler {
    pub fn new(repository: Arc<dyn NodeRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, query: GetNodeStatisticsQuery) -> Result<MultiNodeResponse<NodeStatistics>, DomainError> {
        self.repository.get_statistics(&query.node).await
    }
}
