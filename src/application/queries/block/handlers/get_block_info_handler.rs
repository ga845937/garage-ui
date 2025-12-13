//! Get block info query handler

use std::sync::Arc;
use crate::application::queries::block::GetBlockInfoQuery;
use crate::domain::entities::{MultiNodeResponse, BlockInfo};
use crate::domain::errors::DomainError;
use crate::domain::repositories::BlockRepository;

/// Handler for getting block information
pub struct GetBlockInfoHandler {
    repository: Arc<dyn BlockRepository>,
}

impl GetBlockInfoHandler {
    pub fn new(repository: Arc<dyn BlockRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, query: GetBlockInfoQuery) -> Result<MultiNodeResponse<BlockInfo>, DomainError> {
        self.repository.get_info(&query.node, &query.block_hash).await
    }
}
