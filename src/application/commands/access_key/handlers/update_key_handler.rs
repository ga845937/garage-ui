//! Update access key command handler

use std::sync::Arc;
use crate::application::commands::access_key::UpdateKeyCommand;
use crate::domain::entities::AccessKey;
use crate::domain::errors::DomainError;
use crate::domain::repositories::AccessKeyCommandRepository;

/// Handler for updating access keys
pub struct UpdateKeyHandler {
    repository: Arc<dyn AccessKeyCommandRepository>,
}

impl UpdateKeyHandler {
    pub fn new(repository: Arc<dyn AccessKeyCommandRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: UpdateKeyCommand) -> Result<AccessKey, DomainError> {
        // 1. 驗證 Command
        command.validate()?;
        
        // 2. 檢查是否有變更
        if !command.has_changes() {
            // 沒有變更，直接取得現有資料回傳
            let aggregate = self.repository.get(&command.id).await?;
            return Ok(AccessKey::from_aggregate(aggregate));
        }
        
        // 3. 載入 Aggregate
        let mut aggregate = self.repository.get(&command.id).await?;
        
        // 4. 執行業務邏輯（在 Aggregate 中應用變更）
        aggregate.apply_update(
            command.name,
            command.expiration,
            command.allow_create_bucket,
        )?;
        
        // 5. 持久化
        let aggregate = self.repository.save(&aggregate).await?;
        
        // 6. 轉換為 Read Model 回傳
        Ok(AccessKey::from_aggregate(aggregate))
    }
}
