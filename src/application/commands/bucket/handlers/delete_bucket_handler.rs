//! Delete bucket command handler

use std::sync::Arc;
use futures::future::try_join_all;
use crate::application::commands::bucket::DeleteBucketCommand;
use crate::domain::errors::DomainError;
use crate::domain::events::EventBus;
use crate::domain::repositories::BucketRepository;

/// Delete bucket command handler
pub struct DeleteBucketHandler {
    repository: Arc<dyn BucketRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl DeleteBucketHandler {
    pub fn new(repository: Arc<dyn BucketRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self { repository, event_bus }
    }

    pub async fn handle(&self, command: DeleteBucketCommand) -> Result<Vec<String>, DomainError> {
        // 1. 驗證 Command
        command.validate()?;

        // 2. 並行刪除所有 buckets
        let delete_tasks = command.ids().iter().map(|id| {
            let id = id.to_string();
            let repository = Arc::clone(&self.repository);
            let event_bus = Arc::clone(&self.event_bus);
            
            async move {
                // 載入 Aggregate 並驗證業務規則
                let aggregate = repository.load(&id).await?;
                let event = aggregate.prepare_delete()?;

                // 執行刪除
                repository.delete_bucket(&id).await?;

                // 異步發布事件（不阻塞）
                event_bus.publish_bucket(event).await;

                Ok::<String, DomainError>(id)
            }
        });

        // 使用 try_join_all 並行執行所有刪除操作
        let deleted_ids = try_join_all(delete_tasks).await?;
        
        Ok(deleted_ids)
    }
}
