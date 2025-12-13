//! Add bucket alias command

use crate::domain::errors::DomainError;
use super::alias_types::AliasType;

/// Command to add an alias to a bucket
#[derive(Debug, Clone)]
pub struct AddBucketAliasCommand {
    bucket_id: String,
    alias_type: AliasType,
}

impl AddBucketAliasCommand {
    pub fn new_global(bucket_id: String, alias: String) -> Self {
        Self {
            bucket_id,
            alias_type: AliasType::Global(alias),
        }
    }

    pub fn new_local(bucket_id: String, access_key_id: String, alias: String) -> Self {
        Self {
            bucket_id,
            alias_type: AliasType::Local {
                access_key_id,
                alias,
            },
        }
    }

    pub fn bucket_id(&self) -> &str {
        &self.bucket_id
    }

    pub fn alias_type(&self) -> &AliasType {
        &self.alias_type
    }

    /// 驗證 Command 輸入資料
    pub fn validate(&self) -> Result<(), DomainError> {
        // 基本驗證
        if self.bucket_id.is_empty() {
            return Err(DomainError::ValidationError(
                "Bucket ID cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}
