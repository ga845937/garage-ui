//! List access keys query handler

use std::sync::Arc;
use crate::application::queries::access_key::ListKeysQuery;
use crate::domain::entities::AccessKeyListItem;
use crate::domain::errors::DomainError;
use crate::domain::repositories::AccessKeyQueryRepository;
use crate::shared::paginate;

/// Handler for listing access keys
pub struct ListKeysHandler {
    repository: Arc<dyn AccessKeyQueryRepository>,
}

impl ListKeysHandler {
    pub fn new(repository: Arc<dyn AccessKeyQueryRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, query: ListKeysQuery) -> Result<(Vec<AccessKeyListItem>, usize), DomainError> {
        let rows = self.repository.list().await?;

        let filtered: Vec<_> = if query.has_filter() {
            rows.into_iter()
                .filter(|item| query.matches(item))
                .collect()
        } else {
            rows
        };

        let total = filtered.len();
        let paginated = paginate(&filtered, query.page as usize, query.page_size as usize);

        let paginated_id: Vec<String> = paginated.iter().map(|b| b.id.clone()).collect();

        let task: Vec<_> = paginated_id.iter().map(|id| self.repository.find_by_id(id.as_str())).collect();

        let detail = futures::future::try_join_all(task).await?;

        let data = detail.into_iter().map(|d| AccessKeyListItem {
            id: d.id,
            name: d.name,
            created: d.created,
            expiration: d.expiration,
            secret_access_key: d.secret_access_key,
            expired: d.expired,
        }).collect();

        Ok((data, total))
    }
}
