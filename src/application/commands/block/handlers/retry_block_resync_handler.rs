//! Retry block resync command handler

use std::sync::Arc;
use crate::application::commands::block::RetryBlockResyncCommand;
use crate::domain::entities::{MultiNodeResponse, RetryResyncResult};
use crate::domain::errors::DomainError;
use crate::domain::repositories::BlockRepository;

/// Handler for retrying block resync
pub struct RetryBlockResyncHandler {
    repository: Arc<dyn BlockRepository>,
}

impl RetryBlockResyncHandler {
    pub fn new(repository: Arc<dyn BlockRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: RetryBlockResyncCommand) -> Result<MultiNodeResponse<RetryResyncResult>, DomainError> {
        match command.block_hashes {
            Some(hashes) => self.repository.retry_resync(&command.node, hashes).await,
            None => self.repository.retry_resync_all(&command.node).await,
        }
    }
}
