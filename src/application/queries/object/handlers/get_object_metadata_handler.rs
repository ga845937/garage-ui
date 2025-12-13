//! Get Object Metadata Handler

use std::sync::Arc;
use tracing::info;

use crate::application::queries::object::GetObjectMetadataQuery;
use crate::domain::entities::ObjectMetadata;
use crate::domain::errors::DomainError;
use crate::domain::repositories::ObjectRepository;
use crate::shared::get_trace_id;

/// Handler for GetObjectMetadataQuery
pub struct GetObjectMetadataHandler {
    repository: Arc<dyn ObjectRepository>,
}

impl GetObjectMetadataHandler {
    /// Create a new handler
    pub fn new(repository: Arc<dyn ObjectRepository>) -> Self {
        Self { repository }
    }

    /// Handle the query
    pub async fn handle(
        &self,
        query: GetObjectMetadataQuery,
    ) -> Result<ObjectMetadata, DomainError> {
        let trace_id = get_trace_id();

        info!(
            trace_id = %trace_id,
            bucket = %query.bucket(),
            key = %query.key(),
            "Getting object metadata"
        );

        let metadata = self
            .repository
            .get_metadata(query.bucket(), query.key())
            .await?;

        info!(
            trace_id = %trace_id,
            bucket = %query.bucket(),
            key = %query.key(),
            content_length = metadata.content_length,
            content_type = %metadata.content_type,
            "Got object metadata"
        );

        Ok(metadata)
    }
}
