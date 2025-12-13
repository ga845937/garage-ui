//! List block errors query handler

use std::sync::Arc;
use crate::application::queries::block::ListBlockErrorsQuery;
use crate::domain::entities::{MultiNodeResponse, BlockError};
use crate::domain::errors::DomainError;
use crate::domain::repositories::BlockRepository;

/// Handler for listing block errors
pub struct ListBlockErrorsHandler {
    repository: Arc<dyn BlockRepository>,
}

impl ListBlockErrorsHandler {
    pub fn new(repository: Arc<dyn BlockRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, query: ListBlockErrorsQuery) -> Result<MultiNodeResponse<Vec<BlockError>>, DomainError> {
        self.repository.list_errors(&query.node).await
    }
}
