//! Apply layout command handler

use std::sync::Arc;
use crate::application::commands::cluster::ApplyLayoutCommand;
use crate::domain::entities::ApplyLayoutResult;
use crate::domain::errors::DomainError;
use crate::domain::repositories::ClusterRepository;

/// Handler for applying cluster layout
pub struct ApplyLayoutHandler {
    repository: Arc<dyn ClusterRepository>,
}

impl ApplyLayoutHandler {
    pub fn new(repository: Arc<dyn ClusterRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: ApplyLayoutCommand) -> Result<ApplyLayoutResult, DomainError> {
        self.repository.apply_layout(command.version).await
    }
}
