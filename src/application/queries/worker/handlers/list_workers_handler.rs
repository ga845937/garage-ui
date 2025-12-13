//! List workers query handler

use std::sync::Arc;
use crate::application::queries::worker::ListWorkersQuery;
use crate::domain::entities::{MultiNodeResponse, WorkerInfo};
use crate::domain::errors::DomainError;
use crate::domain::repositories::WorkerRepository;

/// Handler for listing workers
pub struct ListWorkersHandler {
    repository: Arc<dyn WorkerRepository>,
}

impl ListWorkersHandler {
    pub fn new(repository: Arc<dyn WorkerRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, query: ListWorkersQuery) -> Result<MultiNodeResponse<Vec<WorkerInfo>>, DomainError> {
        self.repository.list(&query.node, query.busy_only, query.error_only).await
    }
}
