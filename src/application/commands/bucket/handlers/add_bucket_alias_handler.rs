//! Add bucket alias command handler

use std::sync::Arc;
use crate::application::commands::bucket::AddBucketAliasCommand;
use crate::domain::entities::BucketDetail;
use crate::domain::errors::DomainError;
use crate::domain::repositories::BucketRepository;
use crate::domain::events::{BucketEvent, BucketAliasAddedEvent, EventBus};

/// Handler for adding bucket aliases
pub struct AddBucketAliasHandler {
    repository: Arc<dyn BucketRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl AddBucketAliasHandler {
    pub fn new(
        repository: Arc<dyn BucketRepository>,
        event_bus: Arc<dyn EventBus>,
    ) -> Self {
        Self { repository, event_bus }
    }

    pub async fn handle(
        &self,
        command: AddBucketAliasCommand,
    ) -> Result<(BucketDetail, BucketEvent), DomainError> {
        // 1. 驗證 Command
        command.validate()?;

        // 2. 呼叫 Garage API 添加 alias（直接返回更新後的 bucket）
        let (bucket_detail, alias_name) = match command.alias_type() {
            crate::application::commands::bucket::AliasType::Global(alias) => {
                let detail = self.repository.add_global_alias(command.bucket_id(), alias).await?;
                (detail, alias.clone())
            }
            crate::application::commands::bucket::AliasType::Local {
                access_key_id,
                alias,
            } => {
                let detail = self.repository.add_local_alias(command.bucket_id(), access_key_id, alias).await?;
                (detail, alias.clone())
            }
        };

        // 3. 生成並發布事件
        let event = BucketEvent::AliasAdded(BucketAliasAddedEvent::new(
            command.bucket_id().to_string(),
            alias_name,
        ));

        // 異步發布事件（不阻塞）
        self.event_bus.publish_bucket(event.clone()).await;

        Ok((bucket_detail, event))
    }
}
