//! Copy Object Handler

use std::sync::Arc;
use tracing::info;

use crate::application::commands::object::CopyObjectCommand;
use crate::domain::entities::CopyObjectResult;
use crate::domain::errors::DomainError;
use crate::domain::repositories::ObjectRepository;
use crate::shared::get_trace_id;

/// Handler for CopyObjectCommand
pub struct CopyObjectHandler {
    repository: Arc<dyn ObjectRepository>,
}

impl CopyObjectHandler {
    /// Create a new handler
    pub fn new(repository: Arc<dyn ObjectRepository>) -> Self {
        Self { repository }
    }

    /// Handle the command
    pub async fn handle(&self, command: CopyObjectCommand) -> Result<CopyObjectResult, DomainError> {
        let trace_id = get_trace_id();

        info!(
            trace_id = %trace_id,
            source_bucket = %command.source_bucket(),
            source_key = %command.source_key(),
            dest_bucket = %command.dest_bucket(),
            dest_key = %command.dest_key(),
            "Copying object"
        );

        let result = self.repository
            .copy(
                command.source_bucket(),
                command.source_key(),
                command.dest_bucket(),
                command.dest_key(),
            )
            .await?;

        info!(
            trace_id = %trace_id,
            source_bucket = %command.source_bucket(),
            source_key = %command.source_key(),
            dest_bucket = %command.dest_bucket(),
            dest_key = %command.dest_key(),
            etag = %result.etag,
            "Object copied successfully"
        );

        Ok(result)
    }
}
