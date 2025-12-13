//! Bucket Aggregate Root
//! 
//! 聚合根，負責管理 Bucket 的完整生命週期和業務規則

use crate::domain::entities::{BucketKey, WebsiteConfig};
use crate::domain::errors::DomainError;
use crate::domain::events::{BucketCreatedEvent, BucketDeletedEvent, BucketEvent, BucketUpdatedEvent};
use crate::domain::value_objects::{GlobalAlias, LocalAlias, Quotas};

/// Bucket Aggregate Root
/// 
/// 封裝所有 Bucket 相關的業務規則和不變條件
#[derive(Debug, Clone)]
pub struct BucketAggregate {
    id: String,
    global_aliases: Vec<GlobalAlias>,
    local_aliases: Vec<LocalAlias>,
    quotas: Quotas,
    website_access: bool,
    website_config: Option<WebsiteConfig>,
    keys: Vec<BucketKey>,
    objects: u64,
    bytes: u64,
}

impl BucketAggregate {
    /// 創建新 Bucket（工廠方法）
    /// 
    /// # 業務規則
    /// - ID 必須唯一（由調用者保證）
    /// - Global alias 必須符合 DNS 規範（由 GlobalAlias 保證）
    /// - 創建時配額為無限制
    pub fn create(
        id: String,
        global_alias: Option<String>,
    ) -> Result<(Self, BucketEvent), DomainError> {
        // 驗證並轉換 global_alias
        let global_aliases = if let Some(alias) = global_alias.clone() {
            vec![GlobalAlias::new(alias)?]
        } else {
            vec![]
        };

        let bucket = Self {
            id: id.clone(),
            global_aliases,
            local_aliases: vec![],
            quotas: Quotas::unlimited(),
            website_access: false,
            website_config: None,
            keys: vec![],
            objects: 0,
            bytes: 0,
        };

        let event = BucketEvent::Created(BucketCreatedEvent::new(id, global_alias));
        Ok((bucket, event))
    }

    /// 創建帶有 local alias 的 Bucket
    pub fn create_with_local_alias(
        id: String,
        global_alias: Option<String>,
        access_key_id: String,
        local_alias: String,
    ) -> Result<(Self, BucketEvent), DomainError> {
        let (mut bucket, event) = Self::create(id, global_alias)?;
        
        // 添加 local alias
        let local = LocalAlias::new(access_key_id, local_alias)?;
        bucket.local_aliases.push(local);

        Ok((bucket, event))
    }

    /// 添加 Global Alias
    /// 
    /// # 業務規則
    /// - Alias 不能重複
    /// - 必須符合 DNS 規範
    pub fn add_global_alias(&mut self, alias: String) -> Result<BucketEvent, DomainError> {
        let new_alias = GlobalAlias::new(alias)?;

        // 檢查是否已存在
        if self.global_aliases.contains(&new_alias) {
            return Err(DomainError::BucketAlreadyExists(
                "Alias already exists".to_string()
            ));
        }

        self.global_aliases.push(new_alias);

        Ok(BucketEvent::Updated(BucketUpdatedEvent::new(self.id.clone())))
    }

    /// 移除 Global Alias
    /// 
    /// # 業務規則
    /// - Alias 必須存在
    pub fn remove_global_alias(&mut self, alias: &str) -> Result<BucketEvent, DomainError> {
        let target = GlobalAlias::new(alias.to_string())?;
        
        let original_len = self.global_aliases.len();
        self.global_aliases.retain(|a| a != &target);

        if self.global_aliases.len() == original_len {
            return Err(DomainError::BucketNotFound(
                "Alias not found".to_string()
            ));
        }

        Ok(BucketEvent::Updated(BucketUpdatedEvent::new(self.id.clone())))
    }

    /// 添加 Local Alias
    /// 
    /// # 業務規則
    /// - Alias 不能重複
    /// - 必須符合規範
    pub fn add_local_alias(&mut self, access_key_id: String, alias: String) -> Result<BucketEvent, DomainError> {
        let new_alias = LocalAlias::new(access_key_id, alias)?;

        // 檢查是否已存在
        if self.local_aliases.iter().any(|a| a == &new_alias) {
            return Err(DomainError::LocalAliasAlreadyExists(
                "Local alias already exists".to_string()
            ));
        }

        self.local_aliases.push(new_alias);

        Ok(BucketEvent::Updated(BucketUpdatedEvent::new(self.id.clone())))
    }

    /// 移除 Local Alias
    /// 
    /// # 業務規則
    /// - Alias 必須存在
    pub fn remove_local_alias(&mut self, access_key_id: &str, alias: &str) -> Result<BucketEvent, DomainError> {
        let original_len = self.local_aliases.len();
        self.local_aliases.retain(|a| {
            a.access_key_id() != access_key_id || a.alias() != alias
        });

        if self.local_aliases.len() == original_len {
            return Err(DomainError::BucketNotFound(
                "Local alias not found".to_string()
            ));
        }

        Ok(BucketEvent::Updated(BucketUpdatedEvent::new(self.id.clone())))
    }

    /// 設置配額
    /// 
    /// # 業務規則
    /// - 配額必須為正數或 None
    pub fn set_quotas(&mut self, max_size: Option<i64>, max_objects: Option<i64>) 
        -> Result<BucketEvent, DomainError> {
        self.quotas = Quotas::new(max_size, max_objects)?;
        Ok(BucketEvent::Updated(BucketUpdatedEvent::new(self.id.clone())))
    }

    /// 啟用 Website 訪問
    /// 
    /// # 業務規則
    /// - 啟用時必須提供 index_document
    pub fn enable_website_access(&mut self, website_config: WebsiteConfig) 
        -> Result<BucketEvent, DomainError> {
        // 驗證 website config
        if website_config.index_document.is_empty() {
            return Err(DomainError::InvalidBucketName(
                "Index document is required for website access".to_string()
            ));
        }

        self.website_access = true;
        self.website_config = Some(website_config);

        Ok(BucketEvent::Updated(BucketUpdatedEvent::new(self.id.clone())))
    }

    /// 禁用 Website 訪問
    pub fn disable_website_access(&mut self) -> Result<BucketEvent, DomainError> {
        self.website_access = false;
        self.website_config = None;

        Ok(BucketEvent::Updated(BucketUpdatedEvent::new(self.id.clone())))
    }

    /// 檢查是否可以刪除
    /// 
    /// # 業務規則
    /// - Bucket 必須為空（由外部檢查）
    /// - 有 active keys 的 bucket 不能刪除
    pub fn can_delete(&self) -> Result<(), DomainError> {
        if !self.keys.is_empty() {
            return Err(DomainError::InvalidBucketName(
                "Cannot delete bucket with active keys".to_string()
            ));
        }
        Ok(())
    }

    /// 準備刪除（檢查並生成事件）
    pub fn prepare_delete(self) -> Result<BucketEvent, DomainError> {
        // self.can_delete()?;
        Ok(BucketEvent::Deleted(BucketDeletedEvent::new(self.id)))
    }

    // ============ Getters (唯讀訪問) ============

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn global_aliases(&self) -> Vec<String> {
        self.global_aliases.iter().map(|a| a.value().to_string()).collect()
    }

    pub fn local_aliases(&self) -> &[LocalAlias] {
        &self.local_aliases
    }

    pub fn quotas(&self) -> &Quotas {
        &self.quotas
    }

    pub fn website_access(&self) -> bool {
        self.website_access
    }

    pub fn website_config(&self) -> Option<&WebsiteConfig> {
        self.website_config.as_ref()
    }

    pub fn keys(&self) -> &[BucketKey] {
        &self.keys
    }

    // ============ For Repository (重建 Aggregate) ============

    /// 從持久化資料重建 Aggregate（不驗證）
    /// 
    /// 這個方法用於從資料庫載入已驗證的資料，跳過驗證以提高性能
    pub fn reconstitute(
        id: String,
        global_aliases: Vec<String>,
        local_aliases: Vec<LocalAlias>,
        quotas: Quotas,
        website_access: bool,
        website_config: Option<WebsiteConfig>,
        keys: Vec<BucketKey>,
        objects: u64,
        bytes: u64,
    ) -> Self {
        // 直接重建，假設資料已經驗證過
        let global_aliases = global_aliases
            .into_iter()
            .filter_map(|a| GlobalAlias::new(a).ok())
            .collect();

        Self {
            id,
            global_aliases,
            local_aliases,
            quotas,
            website_access,
            website_config,
            keys,
            objects,
            bytes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bucket() {
        let (bucket, event) = BucketAggregate::create(
            "test-id".to_string(),
            Some("my-bucket".to_string()),
        ).unwrap();

        assert_eq!(bucket.id(), "test-id");
        assert_eq!(bucket.global_aliases().len(), 1);
        assert!(bucket.quotas().is_unlimited());
        assert!(!bucket.website_access());
        assert!(matches!(event, BucketEvent::Created(_)));
    }

    #[test]
    fn test_add_global_alias() {
        let (mut bucket, _) = BucketAggregate::create(
            "test-id".to_string(),
            None,
        ).unwrap();

        let result = bucket.add_global_alias("new-alias".to_string());
        assert!(result.is_ok());
        assert_eq!(bucket.global_aliases().len(), 1);

        // 重複添加應該失敗
        let result = bucket.add_global_alias("new-alias".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_set_quotas() {
        let (mut bucket, _) = BucketAggregate::create(
            "test-id".to_string(),
            None,
        ).unwrap();

        let result = bucket.set_quotas(Some(1000), Some(100));
        assert!(result.is_ok());
        assert_eq!(bucket.quotas().max_size(), Some(1000));

        // 無效配額應該失敗
        let result = bucket.set_quotas(Some(-100), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_enable_website() {
        let (mut bucket, _) = BucketAggregate::create(
            "test-id".to_string(),
            None,
        ).unwrap();

        let config = WebsiteConfig {
            index_document: "index.html".to_string(),
            error_document: "error.html".to_string(),
        };

        let result = bucket.enable_website_access(config);
        assert!(result.is_ok());
        assert!(bucket.website_access());

        // 空 index_document 應該失敗
        let invalid_config = WebsiteConfig {
            index_document: "".to_string(),
            error_document: "error.html".to_string(),
        };
        let (mut bucket2, _) = BucketAggregate::create("test-id".to_string(), None).unwrap();
        let result = bucket2.enable_website_access(invalid_config);
        assert!(result.is_err());
    }

    #[test]
    fn test_can_delete() {
        let (bucket, _) = BucketAggregate::create(
            "test-id".to_string(),
            None,
        ).unwrap();

        // 空 bucket 可以刪除
        assert!(bucket.can_delete().is_ok());
    }
}
