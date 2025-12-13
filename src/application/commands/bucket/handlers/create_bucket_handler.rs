//! Create bucket command handler

use std::sync::Arc;
use crate::application::commands::bucket::CreateBucketCommand;
use crate::domain::errors::DomainError;
use crate::domain::events::{BucketEvent, BucketCreatedEvent, EventBus};
use crate::domain::repositories::BucketRepository;

/// Create bucket command handler
pub struct CreateBucketHandler {
    repository: Arc<dyn BucketRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl CreateBucketHandler {
    pub fn new(repository: Arc<dyn BucketRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self { repository, event_bus }
    }

    pub async fn handle(&self, command: CreateBucketCommand) -> Result<(String, BucketEvent), DomainError> {
        // 1. 驗證 Command
        command.validate()?;
        
        // 2. 先通過 Garage API 創建 bucket 獲得 ID (Garage 生成 ID)
        let input = crate::domain::repositories::CreateBucketInput {
            global_alias: command.global_alias().cloned(),
            local_alias: command.local_alias().map(|la| {
                crate::domain::repositories::LocalAliasInput {
                    access_key_id: la.access_key_id.clone(),
                    alias: la.alias.clone(),
                    allow_read: la.allow_read,
                    allow_write: la.allow_write,
                    allow_owner: la.allow_owner,
                }
            }),
        };
        
        let bucket_id = self.repository.create_bucket(input).await?;
        
        // 3. 如果有額外配置 (quotas, website),使用 Aggregate 管理
        if command.quotas().is_some() || command.website_config().is_some() {
            // 載入剛創建的 bucket
            let mut aggregate = self.repository.load(&bucket_id).await?;
            
            // 設定 quotas
            if let Some(quotas) = command.quotas() {
                aggregate.set_quotas(quotas.max_size(), quotas.max_objects())?;
            }
            
            // 設定 website
            if let Some(config) = command.website_config() {
                aggregate.enable_website_access(config.clone())?;
            }
            
            // 保存更新
            self.repository.save(&aggregate).await?;
        }
        
        // 4. 生成並發布事件
        let event = BucketEvent::Created(BucketCreatedEvent::new(
            bucket_id.clone(),
            command.global_alias().cloned(),
        ));
        
        // 異步發布事件（不阻塞）
        self.event_bus.publish_bucket(event.clone()).await;

        Ok((bucket_id, event))
    }
}
