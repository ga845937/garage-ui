//! Batch allow bucket key command

use crate::domain::errors::DomainError;
use super::allow_bucket_key::BucketKeyPermissionInput;

/// Single item in a batch permission request
#[derive(Debug, Clone)]
pub struct BucketKeyPermissionItem {
    pub bucket_id: String,
    pub access_key_id: String,
    pub permissions: BucketKeyPermissionInput,
}

impl BucketKeyPermissionItem {
    pub fn new(
        bucket_id: String,
        access_key_id: String,
        permissions: BucketKeyPermissionInput,
    ) -> Self {
        Self {
            bucket_id,
            access_key_id,
            permissions,
        }
    }

    pub fn validate(&self) -> Result<(), DomainError> {
        if self.bucket_id.is_empty() {
            return Err(DomainError::ValidationError(
                "Bucket ID cannot be empty".to_string(),
            ));
        }

        if self.access_key_id.is_empty() {
            return Err(DomainError::ValidationError(
                "Access Key ID cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

/// Command to batch allow keys to access buckets
#[derive(Debug, Clone)]
pub struct BatchAllowBucketKeyCommand {
    items: Vec<BucketKeyPermissionItem>,
}

impl BatchAllowBucketKeyCommand {
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
                    format!("Item {}: At least one permission must be set to true", i),
                ));
            }
        }

        Ok(())
    }
}
