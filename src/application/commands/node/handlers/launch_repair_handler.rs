//! Launch repair command handler

use std::sync::Arc;
use crate::application::commands::node::LaunchRepairCommand;
use crate::domain::entities::MultiNodeResponse;
use crate::domain::errors::DomainError;
use crate::domain::repositories::NodeRepository;

/// Handler for launching repair operations
pub struct LaunchRepairHandler {
    repository: Arc<dyn NodeRepository>,
}

impl LaunchRepairHandler {
    pub fn new(repository: Arc<dyn NodeRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: LaunchRepairCommand) -> Result<MultiNodeResponse<()>, DomainError> {
        self.repository.launch_repair(&command.node, &command.repair_type).await
    }
}
