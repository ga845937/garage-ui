//! Create access key command handler

use std::sync::Arc;
use crate::application::commands::access_key::CreateKeyCommand;
use crate::domain::entities::AccessKey;
use crate::domain::errors::DomainError;
use crate::domain::repositories::AccessKeyCommandRepository;
use crate::domain::aggregates::AccessKeyAggregate;

/// Handler for creating access keys
pub struct CreateKeyHandler {
    repository: Arc<dyn AccessKeyCommandRepository>,
}

impl CreateKeyHandler {
    pub fn new(repository: Arc<dyn AccessKeyCommandRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: CreateKeyCommand) -> Result<AccessKey, DomainError> {
        // 1. 驗證 Command
        command.validate()?;
        
        // 2. 透過 Aggregate 建立（業務規則驗證在 Aggregate 中）
        let new_aggregate = AccessKeyAggregate::new(
            command.name().to_string(),
            command.expiration(),
            command.allow_create_bucket(),
        )?;

        // 3. 持久化
        let aggregate = self.repository.create(&new_aggregate).await?;
        
        // 4. 轉換為 Read Model 回傳
        Ok(AccessKey::from_aggregate(aggregate))
    }
}
