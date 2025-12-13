//! Create metadata snapshot command handler

use std::sync::Arc;
use crate::application::commands::node::CreateMetadataSnapshotCommand;
use crate::domain::entities::MultiNodeResponse;
use crate::domain::errors::DomainError;
use crate::domain::repositories::NodeRepository;

/// Handler for creating metadata snapshots
pub struct CreateMetadataSnapshotHandler {
    repository: Arc<dyn NodeRepository>,
}

impl CreateMetadataSnapshotHandler {
    pub fn new(repository: Arc<dyn NodeRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: CreateMetadataSnapshotCommand) -> Result<MultiNodeResponse<()>, DomainError> {
        self.repository.create_metadata_snapshot(&command.node).await
    }
}
