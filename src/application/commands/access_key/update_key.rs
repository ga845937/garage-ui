//! Update access key command

use chrono::{DateTime, Utc};
use crate::domain::errors::DomainError;
use crate::shared::{UpdateField, parse_datetime};
use crate::domain::aggregates::AccessKeyAggregate;

/// Command to update an existing access key
/// 
/// 使用 UpdateField 來區分三種更新意圖：
/// - `NoChange`: 不更新此欄位
/// - `Clear`: 清除此欄位（設為空值）
/// - `Set(value)`: 設為新值
#[derive(Debug, Clone)]
pub struct UpdateKeyCommand {
    pub id: String,
    pub name: UpdateField<String>,
    pub expiration: UpdateField<DateTime<Utc>>,
    pub allow_create_bucket: UpdateField<bool>,
}

impl UpdateKeyCommand {
    /// 創建一個只更新指定欄位的命令
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: UpdateField::NoChange,
            expiration: UpdateField::NoChange,
            allow_create_bucket: UpdateField::NoChange,
        }
    }

    /// 設置名稱更新
    pub fn with_name(mut self, name: UpdateField<String>) -> Self {
        self.name = name;
        self
    }

    /// 設置過期時間更新
    pub fn with_expiration(mut self, expiration: UpdateField<DateTime<Utc>>) -> Self {
        self.expiration = expiration;
        self
    }

    /// 設置創建 Bucket 權限更新
    pub fn with_allow_create_bucket(mut self, allow: UpdateField<bool>) -> Self {
        self.allow_create_bucket = allow;
        self
    }

    /// 驗證 Command 資料
    /// 使用 Aggregate 的驗證規則，確保一致性
    pub fn validate(&self) -> Result<(), DomainError> {
        // ID 驗證
        AccessKeyAggregate::validate_id(&self.id)?;

        // 名稱驗證（如果要更新的話）
        if let UpdateField::Set(ref name) = self.name {
            AccessKeyAggregate::validate_name(name)?;
        }

        // 過期時間驗證（如果設定新值，必須是未來時間）
        if let UpdateField::Set(exp) = self.expiration {
            AccessKeyAggregate::validate_expiration_future(Some(exp))?;
        }

        Ok(())
    }

    /// 檢查是否有任何變更
    pub fn has_changes(&self) -> bool {
        self.name.has_change() 
            || self.expiration.has_change() 
            || self.allow_create_bucket.has_change()
    }

    /// 從 gRPC 請求轉換
    /// expiration 已經由 infrastructure 層轉換為 UpdateField<String>
    pub fn from_grpc_request(
        id: String,
        name: Option<String>,
        expiration: UpdateField<String>,
        allow_create_bucket: Option<bool>,
    ) -> Self {
        Self {
            id,
            name: UpdateField::from_option_with_empty_as_clear(name),
            expiration: parse_expiration_field(expiration),
            allow_create_bucket: UpdateField::from_option(allow_create_bucket),
        }
    }
}

/// 解析過期時間欄位
/// 從 UpdateField<String> 轉換為 UpdateField<DateTime<Utc>>
fn parse_expiration_field(field: UpdateField<String>) -> UpdateField<DateTime<Utc>> {
    match field {
        UpdateField::NoChange => UpdateField::NoChange,
        UpdateField::Clear => UpdateField::Clear,
        UpdateField::Set(s) => {
            parse_datetime(&s)
                .map(UpdateField::Set)
                .unwrap_or(UpdateField::NoChange)
        }
    }
}
