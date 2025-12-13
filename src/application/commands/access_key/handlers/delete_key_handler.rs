//! Delete access key command handler

use std::sync::Arc;
use futures::future::try_join_all;
use crate::application::commands::access_key::DeleteKeyCommand;
use crate::domain::errors::DomainError;
use crate::domain::repositories::AccessKeyCommandRepository;

/// Handler for deleting access keys
pub struct DeleteKeyHandler {
    repository: Arc<dyn AccessKeyCommandRepository>,
}

impl DeleteKeyHandler {
    pub fn new(repository: Arc<dyn AccessKeyCommandRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: DeleteKeyCommand) -> Result<(), DomainError> {
        // 1. 驗證 Command
        command.validate().await?;

        // 2. 並行刪除所有 keys
        let task = command.id().iter().map(|key_id| {
            let repo = Arc::clone(&self.repository);
            let key_id = key_id.clone();

            async move {
                let aggregate = repo.get(&key_id).await?;

                repo.delete(&aggregate).await
            }
        });

        try_join_all(task).await?;

        Ok(())
    }
}
