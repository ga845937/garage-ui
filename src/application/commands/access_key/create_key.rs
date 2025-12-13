//! Create access key command

use chrono::{DateTime, Utc};
use crate::domain::errors::DomainError;
use crate::shared::parse_datetime;
use crate::domain::aggregates::AccessKeyAggregate;

/// Command to create a new access key
#[derive(Debug, Clone)]
pub struct CreateKeyCommand {
    name: String,
    expiration: Option<DateTime<Utc>>,
    allow_create_bucket: bool,
}

impl CreateKeyCommand {
    pub fn new(
        name: String,
        expiration: Option<String>,
        allow_create_bucket: Option<bool>,
    ) -> Self {
        Self {
            name,
            expiration: expiration.and_then(|s| parse_datetime(&s)),
            allow_create_bucket: allow_create_bucket.unwrap_or(false),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn expiration(&self) -> Option<DateTime<Utc>> {
        self.expiration
    }

    pub fn allow_create_bucket(&self) -> bool {
        self.allow_create_bucket
    }

    /// 驗證 Command 輸入資料
    /// 使用 Aggregate 的驗證規則，確保一致性
    pub fn validate(&self) -> Result<(), DomainError> {
        // 使用 Aggregate 的驗證規則
        AccessKeyAggregate::validate_name(&self.name)?;
        AccessKeyAggregate::validate_expiration_future(self.expiration)?;
        Ok(())
    }
}