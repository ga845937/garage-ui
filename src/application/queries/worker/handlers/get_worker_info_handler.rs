//! Get worker info query handler

use std::sync::Arc;
use crate::application::queries::worker::GetWorkerInfoQuery;
use crate::domain::entities::{MultiNodeResponse, WorkerInfo};
use crate::domain::errors::DomainError;
use crate::domain::repositories::WorkerRepository;

/// Handler for getting worker information
pub struct GetWorkerInfoHandler {
    repository: Arc<dyn WorkerRepository>,
}

impl GetWorkerInfoHandler {
    pub fn new(repository: Arc<dyn WorkerRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, query: GetWorkerInfoQuery) -> Result<MultiNodeResponse<WorkerInfo>, DomainError> {
        self.repository.get_info(&query.node, query.worker_id).await
    }
}
