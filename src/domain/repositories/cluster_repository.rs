//! Cluster Repository trait
//!
//! Domain 層的 Repository 抽象介面

use async_trait::async_trait;
use crate::domain::entities::{
    ApplyLayoutResult, ClusterHealth, ClusterLayout, ClusterLayoutHistory,
    ClusterStatistics, ClusterStatus, ConnectNodeResult, SkipDeadNodesResult,
};
use crate::domain::errors::DomainError;

/// 更新集群布局的請求
#[derive(Debug, Clone)]
pub struct UpdateLayoutInput {
    pub node_id: String,
    pub zone: Option<String>,
    pub capacity: Option<u64>,
    pub tags: Option<Vec<String>>,
}

/// Cluster Repository trait
///
/// 定義 Cluster 相關資料存取的契約
/// 具體實現在 infrastructure 層
#[async_trait]
pub trait ClusterRepository: Send + Sync {
    // ============ Cluster ============
    
    /// 獲取集群狀態
    async fn get_status(&self) -> Result<ClusterStatus, DomainError>;
    
    /// 獲取集群健康狀態
    async fn get_health(&self) -> Result<ClusterHealth, DomainError>;
    
    /// 獲取集群統計資訊
    async fn get_statistics(&self) -> Result<ClusterStatistics, DomainError>;
    
    /// 連接集群節點
    async fn connect_nodes(&self, nodes: Vec<String>) -> Result<Vec<ConnectNodeResult>, DomainError>;
    
    // ============ Cluster Layout ============
    
    /// 獲取集群布局
    async fn get_layout(&self) -> Result<ClusterLayout, DomainError>;
    
    /// 更新集群布局
    async fn update_layout(&self, roles: Vec<UpdateLayoutInput>) -> Result<ClusterLayout, DomainError>;
    
    /// 應用集群布局
    async fn apply_layout(&self, version: i64) -> Result<ApplyLayoutResult, DomainError>;
    
    /// 還原集群布局
    async fn revert_layout(&self) -> Result<ClusterLayout, DomainError>;
    
    /// 預覽集群布局變更
    async fn preview_layout_changes(&self) -> Result<ApplyLayoutResult, DomainError>;
    
    /// 獲取集群布局歷史
    async fn get_layout_history(&self) -> Result<ClusterLayoutHistory, DomainError>;
    
    /// 跳過死節點
    async fn skip_dead_nodes(&self, version: i64, allow_missing_data: bool) -> Result<SkipDeadNodesResult, DomainError>;
}
