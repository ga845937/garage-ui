//! Get cluster status query handler

use std::sync::Arc;
use crate::application::queries::cluster::GetClusterStatusQuery;
use crate::domain::entities::ClusterStatus;
use crate::domain::errors::DomainError;
use crate::domain::repositories::ClusterRepository;

/// Handler for getting cluster status
pub struct GetClusterStatusHandler {
    repository: Arc<dyn ClusterRepository>,
}

impl GetClusterStatusHandler {
    pub fn new(repository: Arc<dyn ClusterRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, _query: GetClusterStatusQuery) -> Result<ClusterStatus, DomainError> {
        self.repository.get_status().await
    }
}
