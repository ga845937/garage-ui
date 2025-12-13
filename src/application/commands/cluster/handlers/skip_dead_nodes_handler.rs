//! Skip dead nodes command handler

use std::sync::Arc;
use crate::application::commands::cluster::SkipDeadNodesCommand;
use crate::domain::entities::SkipDeadNodesResult;
use crate::domain::errors::DomainError;
use crate::domain::repositories::ClusterRepository;

/// Handler for skipping dead nodes in layout updates
pub struct SkipDeadNodesHandler {
    repository: Arc<dyn ClusterRepository>,
}

impl SkipDeadNodesHandler {
    pub fn new(repository: Arc<dyn ClusterRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: SkipDeadNodesCommand) -> Result<SkipDeadNodesResult, DomainError> {
        self.repository.skip_dead_nodes(command.version, command.allow_missing_data).await
    }
}
