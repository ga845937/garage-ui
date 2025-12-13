//! Value Objects - Quotas 相關

use serde::{Deserialize, Serialize};
use crate::domain::errors::DomainError;

/// Quotas Value Object
/// 
/// Bucket 配額，限制大小和物件數量
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Quotas {
    max_size: Option<i64>,
    max_objects: Option<i64>,
}

impl Quotas {
    /// 創建新的 Quotas，會進行驗證
    pub fn new(max_size: Option<i64>, max_objects: Option<i64>) -> Result<Self, DomainError> {
        Self::validate(max_size, max_objects)?;
        Ok(Self {
            max_size,
            max_objects,
        })
    }

    /// 無限制配額
    pub fn unlimited() -> Self {
        Self {
            max_size: None,
            max_objects: None,
        }
    }

    /// 驗證配額值
    fn validate(max_size: Option<i64>, max_objects: Option<i64>) -> Result<(), DomainError> {
        // 驗證 max_size 必須為正數
        if let Some(size) = max_size {
            if size <= 0 {
                return Err(DomainError::InvalidBucketName(
                    "Max size must be positive".to_string()
                ));
            }
        }

        // 驗證 max_objects 必須為正數
        if let Some(objects) = max_objects {
            if objects <= 0 {
                return Err(DomainError::InvalidBucketName(
                    "Max objects must be positive".to_string()
                ));
            }
        }

        Ok(())
    }

    pub fn max_size(&self) -> Option<i64> {
        self.max_size
    }

    pub fn max_objects(&self) -> Option<i64> {
        self.max_objects
    }

    /// 檢查是否有任何限制
    pub fn is_unlimited(&self) -> bool {
        self.max_size.is_none() && self.max_objects.is_none()
    }
}

impl Default for Quotas {
    fn default() -> Self {
        Self::unlimited()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_quotas() {
        assert!(Quotas::new(Some(1000), Some(100)).is_ok());
        assert!(Quotas::new(Some(1000), None).is_ok());
        assert!(Quotas::new(None, Some(100)).is_ok());
        assert!(Quotas::new(None, None).is_ok());
    }

    #[test]
    fn test_invalid_quotas() {
        assert!(Quotas::new(Some(0), None).is_err());
        assert!(Quotas::new(Some(-100), None).is_err());
        assert!(Quotas::new(None, Some(0)).is_err());
        assert!(Quotas::new(None, Some(-100)).is_err());
    }

    #[test]
    fn test_unlimited() {
        let quotas = Quotas::unlimited();
        assert!(quotas.is_unlimited());
        assert_eq!(quotas.max_size(), None);
        assert_eq!(quotas.max_objects(), None);
    }
}
