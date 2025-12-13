//! Delete Objects Handler (Batch)

use std::sync::Arc;
use tracing::info;

use crate::application::commands::object::DeleteObjectsCommand;
use crate::domain::entities::{DeleteObjectsResult, DeleteObjectError};
use crate::domain::errors::DomainError;
use crate::domain::repositories::ObjectRepository;
use crate::shared::get_trace_id;

/// Handler for DeleteObjectsCommand
pub struct DeleteObjectsHandler {
    repository: Arc<dyn ObjectRepository>,
}

impl DeleteObjectsHandler {
    /// Create a new handler
    pub fn new(repository: Arc<dyn ObjectRepository>) -> Self {
        Self { repository }
    }

    /// Handle the command
    /// 
    /// 對於以 `/` 結尾的 key（資料夾），會遞迴刪除該資料夾下的所有物件
    pub async fn handle(
        &self,
        command: DeleteObjectsCommand,
    ) -> Result<DeleteObjectsResult, DomainError> {
        let trace_id = get_trace_id();
        let bucket = command.bucket().to_string();
        let keys = command.into_keys();

        // 分離資料夾（以 / 結尾）和普通物件
        let (folder_keys, object_keys): (Vec<String>, Vec<String>) = keys
            .into_iter()
            .partition(|key| key.ends_with('/'));

        let mut all_deleted: Vec<String> = Vec::new();
        let mut all_errors: Vec<DeleteObjectError> = Vec::new();

        // 處理資料夾：遞迴刪除
        for folder_key in folder_keys {
            info!(
                trace_id = %trace_id,
                bucket = %bucket,
                folder = %folder_key,
                "Recursively deleting folder"
            );

            let result = self
                .repository
                .delete_recursive(&bucket, &folder_key)
                .await?;

            all_deleted.extend(result.deleted);
            all_errors.extend(result.errors);
        }

        // 處理普通物件：批次刪除
        if !object_keys.is_empty() {
            info!(
                trace_id = %trace_id,
                bucket = %bucket,
                count = object_keys.len(),
                "Batch deleting objects"
            );

            let result = self
                .repository
                .delete_batch(&bucket, object_keys)
                .await?;

            all_deleted.extend(result.deleted);
            all_errors.extend(result.errors);
        }

        info!(
            trace_id = %trace_id,
            bucket = %bucket,
            total_deleted = all_deleted.len(),
            total_errors = all_errors.len(),
            "Delete operation completed"
        );

        Ok(DeleteObjectsResult {
            deleted: all_deleted,
            errors: all_errors,
        })
    }
}
