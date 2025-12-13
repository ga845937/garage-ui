//! Worker Repository trait
//!
//! Domain 層的 Repository 抽象介面

use async_trait::async_trait;
use crate::domain::entities::{
    MultiNodeResponse, SetVariableResult, WorkerInfo, WorkerVariables,
};
use crate::domain::errors::DomainError;

/// Worker Repository trait
///
/// 定義 Worker 相關資料存取的契約
/// 具體實現在 infrastructure 層
#[async_trait]
pub trait WorkerRepository: Send + Sync {
    /// 列出 Workers
    async fn list(&self, node: &str, busy_only: bool, error_only: bool) -> Result<MultiNodeResponse<Vec<WorkerInfo>>, DomainError>;
    
    /// 獲取 Worker 資訊
    async fn get_info(&self, node: &str, id: i64) -> Result<MultiNodeResponse<WorkerInfo>, DomainError>;
    
    /// 獲取 Worker 變數
    async fn get_variable(&self, node: &str, variable: Option<String>) -> Result<MultiNodeResponse<WorkerVariables>, DomainError>;
    
    /// 設置 Worker 變數
    async fn set_variable(&self, node: &str, variable: String, value: String) -> Result<MultiNodeResponse<SetVariableResult>, DomainError>;
}
