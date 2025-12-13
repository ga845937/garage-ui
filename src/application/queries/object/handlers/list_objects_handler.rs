//! List Objects Handler

use std::sync::Arc;
use tracing::info;

use crate::application::queries::object::ListObjectsQuery;
use crate::domain::entities::ListObjectsResult;
use crate::domain::errors::DomainError;
use crate::domain::repositories::ObjectRepository;
use crate::shared::get_trace_id;

/// Handler for ListObjectsQuery
pub struct ListObjectsHandler {
    repository: Arc<dyn ObjectRepository>,
}

impl ListObjectsHandler {
    /// Create a new handler
    pub fn new(repository: Arc<dyn ObjectRepository>) -> Self {
        Self { repository }
    }

    /// Handle the query
    pub async fn handle(&self, query: ListObjectsQuery) -> Result<ListObjectsResult, DomainError> {
        let trace_id = get_trace_id();

        info!(
            trace_id = %trace_id,
            bucket = %query.bucket(),
            continuation_token = ?query.continuation_token(),
            prefix = ?query.prefix(),
            max_keys = ?query.max_keys(),
            "Listing objects"
        );

        let result = self
            .repository
            .list(
                query.bucket(),
                query.prefix(),
                query.continuation_token(),
                query.max_keys(),
            )
            .await?;

        info!(
            trace_id = %trace_id,
            bucket = %query.bucket(),
            object_count = result.objects.len(),
            is_truncated = result.is_truncated,
            "Listed objects"
        );

        Ok(result)
    }
}
