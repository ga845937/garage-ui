//! Delete bucket command

use crate::domain::errors::DomainError;

/// Command to delete buckets
/// Supports batch deletion like access_key
#[derive(Debug, Clone)]
pub struct DeleteBucketCommand {
    ids: Vec<String>,
}

impl DeleteBucketCommand {
    pub fn new(ids: Vec<String>) -> Self {
        Self { ids }
    }

    pub fn ids(&self) -> &[String] {
        &self.ids
    }

    /// 驗證 Command 輸入資料
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.ids.is_empty() {
            return Err(DomainError::ValidationError(
                "At least one bucket ID must be provided".to_string(),
            ));
        }

        for id in &self.ids {
            if id.is_empty() {
                return Err(DomainError::ValidationError(
                    "Bucket ID cannot be empty".to_string(),
                ));
            }
        }

        Ok(())
    }
}
