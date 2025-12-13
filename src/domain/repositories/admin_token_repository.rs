//! Admin Token Repository trait
//!
//! Domain 層的 Repository 抽象介面

use async_trait::async_trait;
use crate::domain::entities::{AdminTokenCreated, AdminTokenInfo, AdminTokenListItem};
use crate::domain::errors::DomainError;

/// 創建 Admin Token 的請求
#[derive(Debug, Clone, Default)]
pub struct CreateAdminTokenInput {
    pub name: Option<String>,
    pub expiration: Option<String>,
    pub scope: Option<Vec<String>>,
}

/// 更新 Admin Token 的請求
#[derive(Debug, Clone, Default)]
pub struct UpdateAdminTokenInput {
    pub name: Option<String>,
    pub expiration: Option<String>,
    pub scope: Option<Vec<String>>,
}

/// Admin Token Repository trait
///
/// 定義 Admin Token 資料存取的契約
/// 具體實現在 infrastructure 層
#[async_trait]
pub trait AdminTokenRepository: Send + Sync {
    /// 列出所有 Admin Tokens
    async fn list(&self) -> Result<Vec<AdminTokenListItem>, DomainError>;
    
    /// 獲取 Admin Token 詳細資訊
    async fn get(&self, id: &str) -> Result<AdminTokenInfo, DomainError>;
    
    /// 搜索 Admin Token
    async fn search(&self, query: &str) -> Result<AdminTokenInfo, DomainError>;
    
    /// 獲取當前 Admin Token 資訊
    async fn get_current(&self) -> Result<AdminTokenInfo, DomainError>;
    
    /// 創建 Admin Token
    async fn create(&self, input: CreateAdminTokenInput) -> Result<AdminTokenCreated, DomainError>;
    
    /// 更新 Admin Token
    async fn update(&self, id: &str, input: UpdateAdminTokenInput) -> Result<AdminTokenInfo, DomainError>;
    
    /// 刪除 Admin Token
    async fn delete(&self, id: &str) -> Result<(), DomainError>;
}
