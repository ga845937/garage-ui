//! Get bucket query handler

use std::sync::Arc;
use crate::application::queries::bucket::GetBucketQuery;
use crate::domain::entities::BucketDetail;
use crate::domain::errors::DomainError;
use crate::domain::repositories::BucketRepository;

/// Get bucket query handler
pub struct GetBucketHandler {
    repository: Arc<dyn BucketRepository>,
}

impl GetBucketHandler {
    pub fn new(repository: Arc<dyn BucketRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, query: GetBucketQuery) -> Result<BucketDetail, DomainError> {
        self.repository.get_detail(&query.id).await
    }
}
