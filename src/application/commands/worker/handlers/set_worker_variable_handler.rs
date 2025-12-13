//! Set worker variable command handler

use std::sync::Arc;
use crate::application::commands::worker::SetWorkerVariableCommand;
use crate::domain::entities::{MultiNodeResponse, SetVariableResult};
use crate::domain::errors::DomainError;
use crate::domain::repositories::WorkerRepository;

/// Handler for setting worker variables
pub struct SetWorkerVariableHandler {
    repository: Arc<dyn WorkerRepository>,
}

impl SetWorkerVariableHandler {
    pub fn new(repository: Arc<dyn WorkerRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: SetWorkerVariableCommand) -> Result<MultiNodeResponse<SetVariableResult>, DomainError> {
        self.repository.set_variable(&command.node, command.variable, command.value).await
    }
}
