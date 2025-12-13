//! Access Key Repository traits
//!
//! Domain 層的 Repository 抽象介面
//! CQRS: 分離 Command 和 Query Repository

use async_trait::async_trait;
use crate::domain::errors::DomainError;
use crate::domain::aggregates::AccessKeyAggregate;
use crate::domain::entities::{AccessKey, garage::KeyListItemResponse};

/// Command Repository - 用於寫入操作
/// 
/// 處理所有會改變狀態的操作，返回 Aggregate 以維護業務規則
#[async_trait]
pub trait AccessKeyCommandRepository: Send + Sync {
    /// 獲取 Aggregate 用於業務邏輯處理
    async fn get(&self, id: &str) -> Result<AccessKeyAggregate, DomainError>;
    
    /// 創建 Access Key
    async fn create(&self, aggregate: &AccessKeyAggregate) -> Result<AccessKeyAggregate, DomainError>;
    
    /// 保存 Access Key（用於更新，接收 Aggregate 而非 Command）
    async fn save(&self, aggregate: &AccessKeyAggregate) -> Result<AccessKeyAggregate, DomainError>;
    
    /// 刪除 Access Key
    async fn delete(&self, aggregate: &AccessKeyAggregate) -> Result<(), DomainError>;
}

/// Query Repository - 用於讀取操作
/// 
/// 直接返回 Read Model (DTO)，不經過 Aggregate
/// 專為查詢優化，可以做快取、投影等
#[async_trait]
pub trait AccessKeyQueryRepository: Send + Sync {
    /// 列出所有 Access Keys（返回 Read Model）
    async fn list(&self) -> Result<Vec<KeyListItemResponse>, DomainError>;
    
    /// 獲取 Access Key 詳細資訊（返回 Read Model）
    async fn find_by_id(&self, id: &str) -> Result<AccessKey, DomainError>;
}

