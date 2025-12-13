//! Revert layout command handler

use std::sync::Arc;
use crate::application::commands::cluster::RevertLayoutCommand;
use crate::domain::entities::ClusterLayout;
use crate::domain::errors::DomainError;
use crate::domain::repositories::ClusterRepository;

/// Handler for reverting staged layout changes
pub struct RevertLayoutHandler {
    repository: Arc<dyn ClusterRepository>,
}

impl RevertLayoutHandler {
    pub fn new(repository: Arc<dyn ClusterRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, _command: RevertLayoutCommand) -> Result<ClusterLayout, DomainError> {
        self.repository.revert_layout().await
    }
}
