//! Update bucket command handler

use std::sync::Arc;
use crate::application::commands::bucket::UpdateBucketCommand;
use crate::domain::entities::BucketDetail;
use crate::domain::errors::DomainError;
use crate::domain::events::{BucketEvent, BucketUpdatedEvent, EventBus};
use crate::domain::repositories::BucketRepository;
use crate::shared::UpdateField;

/// Update bucket command handler
pub struct UpdateBucketHandler {
    repository: Arc<dyn BucketRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl UpdateBucketHandler {
    pub fn new(repository: Arc<dyn BucketRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self { repository, event_bus }
    }

    pub async fn handle(&self, command: UpdateBucketCommand) -> Result<(BucketDetail, BucketEvent), DomainError> {
        // 1. 驗證 Command
        command.validate()?;
        
        // 2. 載入 Aggregate
        let mut aggregate = self.repository.load(command.id()).await?;

        // 3. 執行業務邏輯
        let mut has_changes = false;

        // 更新 website access 和 config
        match (&command.website_access, &command.website_config) {
            (UpdateField::Set(true), UpdateField::Set(config)) | 
            (UpdateField::NoChange, UpdateField::Set(config)) => {
                aggregate.enable_website_access(config.clone())?;
                has_changes = true;
            }
            (UpdateField::Set(false), _) => {
                aggregate.disable_website_access()?;
                has_changes = true;
            }
            (UpdateField::Clear, _) | (_, UpdateField::Clear) => {
                aggregate.disable_website_access()?;
                has_changes = true;
            }
            _ => {}
        }

        // 更新 quotas
        match &command.quotas {
            UpdateField::Set(quotas) => {
                aggregate.set_quotas(quotas.max_size(), quotas.max_objects())?;
                has_changes = true;
            }
            UpdateField::Clear => {
                aggregate.set_quotas(None, None)?;
                has_changes = true;
            }
            UpdateField::NoChange => {}
        }

        // 4. 如果有變更，保存 Aggregate
        if has_changes {
            self.repository.save(&aggregate).await?;
        }

        // 5. 生成並發布事件
        let final_event = BucketEvent::Updated(BucketUpdatedEvent::new(command.id().to_string()));
        
        // 異步發布事件（不阻塞）
        self.event_bus.publish_bucket(final_event.clone()).await;

        // 6. 轉換為 Read Model 回傳
        let bucket_detail = self.repository.get_detail(command.id()).await?;
        Ok((bucket_detail, final_event))
    }
}
