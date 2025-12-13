//! Get cluster layout query handler

use std::sync::Arc;
use crate::application::queries::cluster::GetClusterLayoutQuery;
use crate::domain::entities::ClusterLayout;
use crate::domain::errors::DomainError;
use crate::domain::repositories::ClusterRepository;

/// Handler for getting cluster layout
pub struct GetClusterLayoutHandler {
    repository: Arc<dyn ClusterRepository>,
}

impl GetClusterLayoutHandler {
    pub fn new(repository: Arc<dyn ClusterRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, _query: GetClusterLayoutQuery) -> Result<ClusterLayout, DomainError> {
        self.repository.get_layout().await
    }
}
