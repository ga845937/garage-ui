//! Get worker variable query handler

use std::sync::Arc;
use crate::application::queries::worker::GetWorkerVariableQuery;
use crate::domain::entities::{MultiNodeResponse, WorkerVariables};
use crate::domain::errors::DomainError;
use crate::domain::repositories::WorkerRepository;

/// Handler for getting worker variables
pub struct GetWorkerVariableHandler {
    repository: Arc<dyn WorkerRepository>,
}

impl GetWorkerVariableHandler {
    pub fn new(repository: Arc<dyn WorkerRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, query: GetWorkerVariableQuery) -> Result<MultiNodeResponse<WorkerVariables>, DomainError> {
        self.repository.get_variable(&query.node, query.variable).await
    }
}
