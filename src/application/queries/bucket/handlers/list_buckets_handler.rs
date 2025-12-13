//! List buckets query handler

use std::sync::Arc;
use crate::application::queries::bucket::ListBucketsQuery;
use crate::domain::entities::Bucket;
use crate::domain::errors::DomainError;
use crate::domain::repositories::BucketRepository;
use crate::shared::paginate;

/// List buckets query handler
pub struct ListBucketsHandler {
    repository: Arc<dyn BucketRepository>,
}

impl ListBucketsHandler {
    pub fn new(repository: Arc<dyn BucketRepository>) -> Self {
        Self { repository }
    }

    /// 執行查詢，返回 (分頁後資料, 總筆數)
    pub async fn handle(&self, query: ListBucketsQuery) -> Result<(Vec<Bucket>, usize), DomainError> {
        let all_buckets = self.repository.list().await?;
        let total = all_buckets.len();
        
        let paginated = paginate(
            &all_buckets,
            query.page as usize,
            query.page_size as usize,
        );

        let paginated_id: Vec<String> = paginated.iter().map(|b| b.id.clone()).collect();

        let task: Vec<_> = paginated_id.iter().map(|id| self.repository.get_detail(id.as_str())).collect();

        let detail = futures::future::try_join_all(task).await?;

        let data = detail.into_iter().map(|d| Bucket {
            id: d.id,
            global_aliases: d.global_aliases,
            local_aliases: d.local_aliases,
            objects: d.objects,
            bytes: d.bytes,
            created: d.created,
        }).collect(); 
        
        Ok((data, total))
    }
}
