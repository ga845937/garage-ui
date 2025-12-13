//! Admin Token Aggregate Root
//!
//! 聚合根，負責管理 Admin Token 的完整生命週期和業務規則

use crate::domain::errors::DomainError;
use crate::domain::events::{
    AdminTokenCreatedEvent, AdminTokenDeletedEvent, AdminTokenEvent, AdminTokenUpdatedEvent,
};

/// Admin Token Aggregate Root
///
/// 封裝所有 Admin Token 相關的業務規則和不變條件
#[derive(Debug, Clone)]
pub struct AdminTokenAggregate {
    id: String,
    name: Option<String>,
    scope: Vec<String>,
    expired: bool,
    expiration: Option<String>,
}

/// 預定義的 Admin Token 權限範圍
pub struct AdminTokenScope;

impl AdminTokenScope {
    pub const READ_CLUSTER_STATUS: &'static str = "read:cluster:status";
    pub const WRITE_CLUSTER_LAYOUT: &'static str = "write:cluster:layout";
    pub const READ_BUCKETS: &'static str = "read:buckets";
    pub const WRITE_BUCKETS: &'static str = "write:buckets";
    pub const READ_KEYS: &'static str = "read:keys";
    pub const WRITE_KEYS: &'static str = "write:keys";
    pub const ADMIN: &'static str = "admin:*";
}

impl AdminTokenAggregate {
    /// 創建新 Admin Token（工廠方法）
    pub fn create(
        id: String,
        name: Option<String>,
        scope: Vec<String>,
    ) -> Result<(Self, AdminTokenEvent), DomainError> {
        // 驗證 scope
        Self::validate_scope(&scope)?;

        let token = Self {
            id: id.clone(),
            name: name.clone(),
            scope: scope.clone(),
            expired: false,
            expiration: None,
        };

        let event = AdminTokenEvent::Created(AdminTokenCreatedEvent::new(id, name, scope));
        Ok((token, event))
    }

    /// 從現有數據重建 Aggregate（用於讀取）
    pub fn reconstitute(
        id: String,
        name: Option<String>,
        scope: Vec<String>,
        expired: bool,
        expiration: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            scope,
            expired,
            expiration,
        }
    }

    /// 驗證 scope 格式
    fn validate_scope(scope: &[String]) -> Result<(), DomainError> {
        for s in scope {
            if s.is_empty() {
                return Err(DomainError::ValidationError("Scope cannot contain empty strings".to_string()));
            }
            // 基本格式驗證：應該是 action:resource 或 action:resource:subresource
            if !s.contains(':') && s != "*" {
                return Err(DomainError::ValidationError(
                    format!("Invalid scope format: {}. Expected format: action:resource", s)
                ));
            }
        }
        Ok(())
    }

    /// 更新名稱
    pub fn update_name(&mut self, name: Option<String>) -> Result<AdminTokenEvent, DomainError> {
        self.name = name.clone();
        Ok(AdminTokenEvent::Updated(AdminTokenUpdatedEvent::new(self.id.clone(), name)))
    }

    /// 更新 scope
    pub fn update_scope(&mut self, scope: Vec<String>) -> Result<AdminTokenEvent, DomainError> {
        Self::validate_scope(&scope)?;
        self.scope = scope;
        Ok(AdminTokenEvent::Updated(AdminTokenUpdatedEvent::new(self.id.clone(), self.name.clone())))
    }

    /// 刪除 Admin Token
    pub fn delete(self) -> Result<AdminTokenEvent, DomainError> {
        Ok(AdminTokenEvent::Deleted(AdminTokenDeletedEvent::new(self.id)))
    }

    /// 檢查是否有特定權限
    pub fn has_permission(&self, required_scope: &str) -> bool {
        // admin:* 擁有所有權限
        if self.scope.iter().any(|s| s == AdminTokenScope::ADMIN || s == "*") {
            return true;
        }
        self.scope.iter().any(|s| s == required_scope)
    }

    // ============ Getters ============

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn scope(&self) -> &[String] {
        &self.scope
    }

    pub fn is_expired(&self) -> bool {
        self.expired
    }
}
