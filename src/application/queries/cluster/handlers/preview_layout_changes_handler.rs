//! Preview layout changes query handler

use std::sync::Arc;
use crate::application::queries::cluster::PreviewLayoutChangesQuery;
use crate::domain::entities::ApplyLayoutResult;
use crate::domain::errors::DomainError;
use crate::domain::repositories::ClusterRepository;

/// Handler for previewing layout changes
pub struct PreviewLayoutChangesHandler {
    repository: Arc<dyn ClusterRepository>,
}

impl PreviewLayoutChangesHandler {
    pub fn new(repository: Arc<dyn ClusterRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, _query: PreviewLayoutChangesQuery) -> Result<ApplyLayoutResult, DomainError> {
        self.repository.preview_layout_changes().await
    }
}
