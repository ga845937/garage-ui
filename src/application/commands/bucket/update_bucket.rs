//! Update bucket command

use crate::domain::entities::WebsiteConfig;
use crate::domain::value_objects::Quotas;
use crate::domain::errors::DomainError;
use crate::shared::UpdateField;

/// Command to update a bucket
///
/// 使用 UpdateField 來區分三種更新意圖：
/// - `NoChange`: 不更新此欄位
/// - `Clear`: 清除此欄位（設為空值）
/// - `Set(value)`: 設為新值
#[derive(Debug, Clone)]
pub struct UpdateBucketCommand {
    id: String,
    pub website_access: UpdateField<bool>,
    pub website_config: UpdateField<WebsiteConfig>,
    pub quotas: UpdateField<Quotas>,
}

impl UpdateBucketCommand {
    /// 創建一個只更新指定欄位的命令
    pub fn new(id: String) -> Self {
        Self {
            id,
            website_access: UpdateField::NoChange,
            website_config: UpdateField::NoChange,
            quotas: UpdateField::NoChange,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    /// 設置 website_access 更新
    pub fn with_website_access(mut self, access: UpdateField<bool>) -> Self {
        self.website_access = access;
        self
    }

    /// 設置 website_config 更新
    pub fn with_website_config(mut self, config: UpdateField<WebsiteConfig>) -> Self {
        self.website_config = config;
        self
    }

    /// 設置 quotas 更新
    pub fn with_quotas(mut self, quotas: UpdateField<Quotas>) -> Self {
        self.quotas = quotas;
        self
    }

    /// 驗證 Command 資料
    pub fn validate(&self) -> Result<(), DomainError> {
        // website_config 驗證（如果要更新的話）
        if let UpdateField::Set(ref config) = self.website_config {
            if config.index_document.is_empty() {
                return Err(DomainError::ValidationError(
                    "Index document cannot be empty when setting website config".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// 檢查是否有任何變更
    pub fn has_changes(&self) -> bool {
        self.website_access.has_change()
            || self.website_config.has_change()
            || self.quotas.has_change()
    }

    /// 從 gRPC 請求轉換
    pub fn from_grpc_request(
        id: String,
        website_access: UpdateField<bool>,
        website_config: Option<WebsiteConfig>,
        quotas: Option<Quotas>,
    ) -> Self {
        Self {
            id,
            website_access,
            website_config: UpdateField::from_option(website_config),
            quotas: UpdateField::from_option(quotas),
        }
    }
}
