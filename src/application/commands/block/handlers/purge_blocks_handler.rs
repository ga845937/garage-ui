//! Purge blocks command handler

use std::sync::Arc;
use crate::application::commands::block::PurgeBlocksCommand;
use crate::domain::entities::{MultiNodeResponse, PurgeBlocksResult};
use crate::domain::errors::DomainError;
use crate::domain::repositories::BlockRepository;

/// Handler for purging blocks
pub struct PurgeBlocksHandler {
    repository: Arc<dyn BlockRepository>,
}

impl PurgeBlocksHandler {
    pub fn new(repository: Arc<dyn BlockRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: PurgeBlocksCommand) -> Result<MultiNodeResponse<PurgeBlocksResult>, DomainError> {
        self.repository.purge(&command.node, command.block_hashes).await
    }
}
