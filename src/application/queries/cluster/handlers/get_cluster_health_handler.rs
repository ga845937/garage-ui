//! Get cluster health query handler

use std::sync::Arc;
use crate::application::queries::cluster::GetClusterHealthQuery;
use crate::domain::entities::ClusterHealth;
use crate::domain::errors::DomainError;
use crate::domain::repositories::ClusterRepository;

/// Handler for getting cluster health
pub struct GetClusterHealthHandler {
    repository: Arc<dyn ClusterRepository>,
}

impl GetClusterHealthHandler {
    pub fn new(repository: Arc<dyn ClusterRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, _query: GetClusterHealthQuery) -> Result<ClusterHealth, DomainError> {
        self.repository.get_health().await
    }
}
