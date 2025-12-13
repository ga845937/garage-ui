//! Access Key Aggregate Root
//!
//! 聚合根，負責管理 Access Key 的完整生命週期和業務規則
//! 
//! Domain 層不依賴 Infrastructure 或 Application 層

use chrono::{DateTime, Utc};
use crate::domain::errors::DomainError;
use crate::shared::UpdateField;

/// Access Key Aggregate Root
///
/// 封裝所有 Access Key 相關的業務規則和不變條件

// ============ Value Objects ============

#[derive(Debug, Clone)]
pub struct BucketPermissionVO {
    owner: bool,
    read: bool,
    write: bool,
}

impl BucketPermissionVO {
    pub fn new(owner: bool, read: bool, write: bool) -> Self {
        Self { owner, read, write }
    }

    pub fn owner(&self) -> bool {
        self.owner
    }

    pub fn read(&self) -> bool {
        self.read
    }

    pub fn write(&self) -> bool {
        self.write
    }
}

#[derive(Debug, Clone)]
pub struct BucketVO {
    id: String,
    global_aliases: Vec<String>,
    local_aliases: Vec<String>,
    permissions: BucketPermissionVO,
}

impl BucketVO {
    pub fn new(
        id: String,
        global_aliases: Vec<String>,
        local_aliases: Vec<String>,
        permissions: BucketPermissionVO,
    ) -> Self {
        Self { id, global_aliases, local_aliases, permissions }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn global_aliases(&self) -> &Vec<String> {
        &self.global_aliases
    }

    pub fn local_aliases(&self) -> &Vec<String> {
        &self.local_aliases
    }

    pub fn permissions(&self) -> &BucketPermissionVO {
        &self.permissions
    }
}

// ============ Aggregate ============

#[derive(Debug, Clone)]
pub struct AccessKeyAggregate {
    id: String,
    buckets: Vec<BucketVO>,
    created: DateTime<Utc>,
    expiration: Option<DateTime<Utc>>,
    name: String,
    can_create_bucket: bool,
    secret_access_key: String,
}

impl AccessKeyAggregate {
    // ============ Factory Methods ============

    /// 創建新的 Access Key（用於 Create 操作）
    /// 驗證名稱並初始化新的 Aggregate
    pub fn new(
        name: String,
        expiration: Option<DateTime<Utc>>,
        can_create_bucket: bool,
    ) -> Result<Self, DomainError> {
        // 業務驗證
        Self::validate_name(&name)?;
        
        Ok(Self {
            id: String::new(), // 由 Repository 填充
            buckets: vec![],
            created: Utc::now(),
            expiration,
            name,
            can_create_bucket,
            secret_access_key: String::new(),
        })
    }

    /// 從持久化儲存重建 Aggregate（不做驗證）
    /// 只在 Infrastructure 層的 Repository 中使用
    pub fn reconstitute(
        id: String,
        name: String,
        buckets: Vec<BucketVO>,
        created: DateTime<Utc>,
        expiration: Option<DateTime<Utc>>,
        can_create_bucket: bool,
        secret_access_key: String,
    ) -> Self {
        Self {
            id,
            name,
            buckets,
            created,
            expiration,
            can_create_bucket,
            secret_access_key,
        }
    }

    // ============ Business Operations ============

    /// 重新命名 Access Key
    pub fn rename(&mut self, new_name: String) -> Result<(), DomainError> {
        Self::validate_name(&new_name)?;
        self.name = new_name;
        Ok(())
    }

    /// 更新過期時間
    pub fn update_expiration(&mut self, expiration: Option<DateTime<Utc>>) {
        self.expiration = expiration;
    }

    /// 更新建立 Bucket 權限
    pub fn update_create_bucket_permission(&mut self, can_create: bool) {
        self.can_create_bucket = can_create;
    }

    /// 應用更新（使用 UpdateField 語義）
    pub fn apply_update(
        &mut self,
        name: UpdateField<String>,
        expiration: UpdateField<DateTime<Utc>>,
        can_create_bucket: UpdateField<bool>,
    ) -> Result<(), DomainError> {
        // 處理名稱更新
        if let UpdateField::Set(new_name) = name {
            self.rename(new_name)?;
        }
        // Clear 對於 name 不合理，忽略

        // 處理過期時間更新
        match expiration {
            UpdateField::NoChange => {}
            UpdateField::Clear => self.expiration = None,
            UpdateField::Set(exp) => self.expiration = Some(exp),
        }

        // 處理權限更新
        match can_create_bucket {
            UpdateField::NoChange => {}
            UpdateField::Clear => self.can_create_bucket = false,
            UpdateField::Set(v) => self.can_create_bucket = v,
        }

        Ok(())
    }

    /// 檢查是否已過期
    pub fn is_expired(&self) -> bool {
        match self.expiration {
            Some(exp) => exp < Utc::now(),
            None => false, // 無過期時間 = 永不過期
        }
    }

    // ============ Validation ============

    /// 驗證名稱格式（公開供 Command 層使用）
    pub fn validate_name(name: &str) -> Result<(), DomainError> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(DomainError::ValidationError(
                "Access key name cannot be empty".to_string()
            ));
        }
        if trimmed.len() > 255 {
            return Err(DomainError::ValidationError(
                "Access key name cannot exceed 255 characters".to_string()
            ));
        }
        Ok(())
    }

    /// 驗證過期時間必須是未來（公開供 Command 層使用）
    pub fn validate_expiration_future(expiration: Option<DateTime<Utc>>) -> Result<(), DomainError> {
        if let Some(exp) = expiration {
            if exp < Utc::now() {
                return Err(DomainError::ValidationError(
                    "Expiration time must be in the future".to_string()
                ));
            }
        }
        Ok(())
    }

    /// 驗證 ID 非空
    pub fn validate_id(id: &str) -> Result<(), DomainError> {
        if id.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Access key ID cannot be empty".to_string()
            ));
        }
        Ok(())
    }

    // ============ Getters ============

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn can_create_bucket(&self) -> bool {
        self.can_create_bucket
    }

    pub fn secret_access_key(&self) -> &str {
        &self.secret_access_key
    }

    pub fn buckets(&self) -> &Vec<BucketVO> {
        &self.buckets
    }

    pub fn created(&self) -> DateTime<Utc> {
        self.created
    }

    pub fn expiration(&self) -> Option<DateTime<Utc>> {
        self.expiration
    }
}
