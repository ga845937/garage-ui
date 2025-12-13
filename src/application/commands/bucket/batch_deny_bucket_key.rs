//! Batch deny bucket key command

use crate::domain::errors::DomainError;
use super::batch_allow_bucket_key::BucketKeyPermissionItem;

/// Command to batch deny keys from accessing buckets
#[derive(Debug, Clone)]
pub struct BatchDenyBucketKeyCommand {
    items: Vec<BucketKeyPermissionItem>,
}

impl BatchDenyBucketKeyCommand {
    pub fn new(items: Vec<BucketKeyPermissionItem>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> &[BucketKeyPermissionItem] {
        &self.items
    }

    /// 驗證 Command 輸入資料
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.items.is_empty() {
            return Err(DomainError::ValidationError(
                "At least one item is required".to_string(),
            ));
        }

        for (i, item) in self.items.iter().enumerate() {
            item.validate().map_err(|e| {
                DomainError::ValidationError(format!("Item {}: {}", i, e))
            })?;

            // 檢查權限至少有一個設為 true
            if !item.permissions.read && !item.permissions.write && !item.permissions.owner {
                return Err(DomainError::ValidationError(
                    format!("Item {}: At least one permission must be set to true to deny", i),
                ));
            }
        }

        Ok(())
    }
}
