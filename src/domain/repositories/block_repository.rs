//! Block Repository trait
//!
//! Domain 層的 Repository 抽象介面

use async_trait::async_trait;
use crate::domain::entities::{
    BlockError, BlockInfo, MultiNodeResponse, PurgeBlocksResult, RetryResyncResult,
};
use crate::domain::errors::DomainError;

/// Block Repository trait
///
/// 定義 Block 相關資料存取的契約
/// 具體實現在 infrastructure 層
#[async_trait]
pub trait BlockRepository: Send + Sync {
    /// 獲取區塊資訊
    async fn get_info(&self, node: &str, block_hash: &str) -> Result<MultiNodeResponse<BlockInfo>, DomainError>;
    
    /// 列出區塊錯誤
    async fn list_errors(&self, node: &str) -> Result<MultiNodeResponse<Vec<BlockError>>, DomainError>;
    
    /// 清除區塊
    async fn purge(&self, node: &str, block_hashes: Vec<String>) -> Result<MultiNodeResponse<PurgeBlocksResult>, DomainError>;
    
    /// 重試區塊重同步（指定區塊）
    async fn retry_resync(&self, node: &str, block_hashes: Vec<String>) -> Result<MultiNodeResponse<RetryResyncResult>, DomainError>;
    
    /// 重試所有區塊重同步
    async fn retry_resync_all(&self, node: &str) -> Result<MultiNodeResponse<RetryResyncResult>, DomainError>;
}
