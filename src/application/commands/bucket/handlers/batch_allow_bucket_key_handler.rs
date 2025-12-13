//! Batch allow bucket key command handler

use std::sync::Arc;
use futures::future::join_all;
use crate::application::commands::bucket::BatchAllowBucketKeyCommand;
use crate::domain::entities::BucketDetail;
use crate::domain::errors::DomainError;
use crate::domain::repositories::BucketRepository;
use crate::domain::events::{BucketEvent, BucketKeyAllowedEvent, EventBus};

/// Result of a single permission operation
#[derive(Debug, Clone)]
pub struct BatchPermissionResult {
    pub bucket_id: String,
    pub access_key_id: String,
    pub success: bool,
    pub error: Option<String>,
    pub bucket: Option<BucketDetail>,
}

/// Handler for batch allowing bucket key permissions
pub struct BatchAllowBucketKeyHandler {
    repository: Arc<dyn BucketRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl BatchAllowBucketKeyHandler {
    pub fn new(
        repository: Arc<dyn BucketRepository>,
        event_bus: Arc<dyn EventBus>,
    ) -> Self {
        Self { repository, event_bus }
    }

    pub async fn handle(
        &self,
        command: BatchAllowBucketKeyCommand,
    ) -> Result<Vec<BatchPermissionResult>, DomainError> {
        // 1. 驗證 Command
        command.validate()?;

        // 2. 並行處理所有項目
        let futures: Vec<_> = command.items().iter().map(|item| {
            let repo = self.repository.clone();
            let bucket_id = item.bucket_id.clone();
            let access_key_id = item.access_key_id.clone();
            let read = item.permissions.read;
            let write = item.permissions.write;
            let owner = item.permissions.owner;

            async move {
                match repo.allow_bucket_key(&bucket_id, &access_key_id, read, write, owner).await {
                    Ok(bucket) => BatchPermissionResult {
                        bucket_id: bucket_id.clone(),
                        access_key_id: access_key_id.clone(),
                        success: true,
                        error: None,
                        bucket: Some(bucket),
                    },
                    Err(e) => BatchPermissionResult {
                        bucket_id,
                        access_key_id,
                        success: false,
                        error: Some(e.to_string()),
                        bucket: None,
                    },
                }
            }
        }).collect();

        let results = join_all(futures).await;

        // 3. 發布成功項目的事件
        for (item, result) in command.items().iter().zip(results.iter()) {
            if result.success {
                let event = BucketEvent::KeyAllowed(BucketKeyAllowedEvent::new(
                    item.bucket_id.clone(),
                    item.access_key_id.clone(),
                    item.permissions.read,
                    item.permissions.write,
                    item.permissions.owner,
                ));
                self.event_bus.publish_bucket(event).await;
            }
        }

        Ok(results)
    }
}
