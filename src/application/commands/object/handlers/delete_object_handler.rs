//! Delete Object Handler

use std::sync::Arc;
use tracing::info;

use crate::application::commands::object::DeleteObjectCommand;
use crate::domain::errors::DomainError;
use crate::domain::repositories::ObjectRepository;
use crate::shared::get_trace_id;

/// Handler for DeleteObjectCommand
pub struct DeleteObjectHandler {
    repository: Arc<dyn ObjectRepository>,
}

impl DeleteObjectHandler {
    /// Create a new handler
    pub fn new(repository: Arc<dyn ObjectRepository>) -> Self {
        Self { repository }
    }

    /// Handle the command
    pub async fn handle(&self, command: DeleteObjectCommand) -> Result<(), DomainError> {
        let trace_id = get_trace_id();

        info!(
            trace_id = %trace_id,
            bucket = %command.bucket(),
            key = %command.key(),
            "Deleting object"
        );

        self.repository
            .delete(command.bucket(), command.key())
            .await?;

        info!(
            trace_id = %trace_id,
            bucket = %command.bucket(),
            key = %command.key(),
            "Object deleted successfully"
        );

        Ok(())
    }
}
