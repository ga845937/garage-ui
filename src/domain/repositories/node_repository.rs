//! Node Repository trait
//!
//! Domain 層的 Repository 抽象介面

use async_trait::async_trait;
use crate::domain::entities::{MultiNodeResponse, NodeInfo, NodeStatistics};
use crate::domain::errors::DomainError;

/// Node Repository trait
///
/// 定義 Node 相關資料存取的契約
/// 具體實現在 infrastructure 層
#[async_trait]
pub trait NodeRepository: Send + Sync {
    /// 獲取節點資訊
    async fn get_info(&self, node: &str) -> Result<MultiNodeResponse<NodeInfo>, DomainError>;
    
    /// 獲取節點統計
    async fn get_statistics(&self, node: &str) -> Result<MultiNodeResponse<NodeStatistics>, DomainError>;
    
    /// 創建元數據快照
    async fn create_metadata_snapshot(&self, node: &str) -> Result<MultiNodeResponse<()>, DomainError>;
    
    /// 啟動修復操作
    async fn launch_repair(&self, node: &str, repair_type: &str) -> Result<MultiNodeResponse<()>, DomainError>;
}
