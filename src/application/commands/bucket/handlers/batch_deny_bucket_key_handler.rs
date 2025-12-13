//! Batch deny bucket key command handler

use std::sync::Arc;
use futures::future::join_all;
use crate::application::commands::bucket::BatchDenyBucketKeyCommand;
use crate::domain::errors::DomainError;
use crate::domain::repositories::BucketRepository;
use crate::domain::events::{BucketEvent, BucketKeyDeniedEvent, EventBus};

use super::batch_allow_bucket_key_handler::BatchPermissionResult;

/// Handler for batch denying bucket key permissions
pub struct BatchDenyBucketKeyHandler {
    repository: Arc<dyn BucketRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl BatchDenyBucketKeyHandler {
    pub fn new(
        repository: Arc<dyn BucketRepository>,
        event_bus: Arc<dyn EventBus>,
    ) -> Self {
        Self { repository, event_bus }
    }

    pub async fn handle(
        &self,
        command: BatchDenyBucketKeyCommand,
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
                match repo.deny_bucket_key(&bucket_id, &access_key_id, read, write, owner).await {
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
                let event = BucketEvent::KeyDenied(BucketKeyDeniedEvent::new(
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
