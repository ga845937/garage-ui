//! Get access key query handler

use std::sync::Arc;
use crate::application::queries::access_key::ReadKeyQuery;
use crate::domain::entities::AccessKey;
use crate::domain::errors::DomainError;
use crate::domain::repositories::AccessKeyQueryRepository;

/// Handler for getting an access key by ID
pub struct ReadKeyHandler {
    repository: Arc<dyn AccessKeyQueryRepository>,
}

impl ReadKeyHandler {
    pub fn new(repository: Arc<dyn AccessKeyQueryRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, query: ReadKeyQuery) -> Result<AccessKey, DomainError> {
        self.repository.find_by_id(&query.id).await
    }
}
