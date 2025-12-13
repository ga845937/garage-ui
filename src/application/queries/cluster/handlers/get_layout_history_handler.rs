//! Get layout history query handler

use std::sync::Arc;
use crate::application::queries::cluster::GetLayoutHistoryQuery;
use crate::domain::entities::ClusterLayoutHistory;
use crate::domain::errors::DomainError;
use crate::domain::repositories::ClusterRepository;

/// Handler for getting cluster layout history
pub struct GetLayoutHistoryHandler {
    repository: Arc<dyn ClusterRepository>,
}

impl GetLayoutHistoryHandler {
    pub fn new(repository: Arc<dyn ClusterRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, _query: GetLayoutHistoryQuery) -> Result<ClusterLayoutHistory, DomainError> {
        self.repository.get_layout_history().await
    }
}
