//! Value Objects - Alias 相關

use serde::{Deserialize, Serialize};
use crate::domain::errors::DomainError;

/// Global Alias Value Object
/// 
/// DNS-compatible 全域別名，在整個集群中唯一
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GlobalAlias(String);

impl GlobalAlias {
    /// 創建新的 GlobalAlias，會進行驗證
    pub fn new(alias: String) -> Result<Self, DomainError> {
        Self::validate(&alias)?;
        Ok(Self(alias))
    }

    /// 驗證 alias 格式
    fn validate(alias: &str) -> Result<(), DomainError> {
        // 長度檢查：1-63 字符（DNS 標準）
        if alias.is_empty() || alias.len() > 63 {
            return Err(DomainError::InvalidBucketName(
                "Alias must be 1-63 characters".to_string()
            ));
        }

        // 必須以字母或數字開頭和結尾
        let first = alias.chars().next().unwrap();
        let last = alias.chars().last().unwrap();
        if !first.is_ascii_alphanumeric() || !last.is_ascii_alphanumeric() {
            return Err(DomainError::InvalidBucketName(
                "Alias must start and end with alphanumeric character".to_string()
            ));
        }

        // 只能包含小寫字母、數字和連字符
        if !alias.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
            return Err(DomainError::InvalidBucketName(
                "Alias must contain only lowercase letters, numbers, and hyphens".to_string()
            ));
        }

        // 不能包含連續的連字符
        if alias.contains("--") {
            return Err(DomainError::InvalidBucketName(
                "Alias cannot contain consecutive hyphens".to_string()
            ));
        }

        Ok(())
    }

    /// 獲取內部值
    pub fn value(&self) -> &str {
        &self.0
    }

    /// 轉換為 String（消耗 self）
    pub fn into_string(self) -> String {
        self.0
    }
}

/// Local Alias Value Object
/// 
/// 綁定到特定 Access Key 的本地別名
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalAlias {
    pub access_key_id: String,
    pub alias: String,
}

impl LocalAlias {
    pub fn new(access_key_id: String, alias: String) -> Result<Self, DomainError> {
        // 驗證 access_key_id 不為空
        if access_key_id.is_empty() {
            return Err(DomainError::InvalidBucketName(
                "Access key ID cannot be empty".to_string()
            ));
        }

        // 驗證 alias
        GlobalAlias::new(alias.clone())?;

        Ok(Self {
            access_key_id,
            alias,
        })
    }

    pub fn access_key_id(&self) -> &str {
        &self.access_key_id
    }

    pub fn alias(&self) -> &str {
        &self.alias
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_global_alias() {
        assert!(GlobalAlias::new("my-bucket".to_string()).is_ok());
        assert!(GlobalAlias::new("bucket123".to_string()).is_ok());
        assert!(GlobalAlias::new("a".to_string()).is_ok());
    }

    #[test]
    fn test_invalid_global_alias() {
        // 空字串
        assert!(GlobalAlias::new("".to_string()).is_err());
        
        // 太長
        assert!(GlobalAlias::new("a".repeat(64)).is_err());
        
        // 包含大寫
        assert!(GlobalAlias::new("MyBucket".to_string()).is_err());
        
        // 包含非法字符
        assert!(GlobalAlias::new("my_bucket".to_string()).is_err());
        assert!(GlobalAlias::new("my.bucket".to_string()).is_err());
        
        // 以連字符開頭或結尾
        assert!(GlobalAlias::new("-bucket".to_string()).is_err());
        assert!(GlobalAlias::new("bucket-".to_string()).is_err());
        
        // 連續連字符
        assert!(GlobalAlias::new("my--bucket".to_string()).is_err());
    }
}
