//! Cluster Aggregate Root
//!
//! 聚合根，負責管理 Cluster 的布局和節點操作

use crate::domain::errors::DomainError;
use crate::domain::events::{
    ClusterDeadNodesSkippedEvent, ClusterEvent, ClusterLayoutAppliedEvent,
    ClusterLayoutRevertedEvent, ClusterLayoutUpdatedEvent, ClusterNodesConnectedEvent,
};

/// 節點角色配置
#[derive(Debug, Clone)]
pub struct NodeRoleConfig {
    pub node_id: String,
    pub zone: Option<String>,
    pub capacity: Option<u64>,
    pub tags: Option<Vec<String>>,
}

/// Cluster Aggregate Root
///
/// 封裝所有 Cluster 相關的業務規則和不變條件
#[derive(Debug, Clone)]
pub struct ClusterAggregate {
    layout_version: i64,
    nodes: Vec<String>,
    staged_changes: Vec<NodeRoleConfig>,
}

impl ClusterAggregate {
    /// 創建新的 Cluster Aggregate
    pub fn new(layout_version: i64) -> Self {
        Self {
            layout_version,
            nodes: vec![],
            staged_changes: vec![],
        }
    }

    /// 從現有數據重建 Aggregate
    pub fn reconstitute(
        layout_version: i64,
        nodes: Vec<String>,
    ) -> Self {
        Self {
            layout_version,
            nodes,
            staged_changes: vec![],
        }
    }

    /// 連接節點
    ///
    /// # 業務規則
    /// - 節點地址必須是有效格式
    pub fn connect_nodes(&mut self, addresses: Vec<String>, successful_count: usize) -> Result<ClusterEvent, DomainError> {
        // 驗證地址格式
        for addr in &addresses {
            if addr.is_empty() {
                return Err(DomainError::ValidationError("Node address cannot be empty".to_string()));
            }
        }

        Ok(ClusterEvent::NodesConnected(ClusterNodesConnectedEvent::new(
            addresses,
            successful_count,
        )))
    }

    /// 暫存布局變更
    ///
    /// # 業務規則
    /// - 節點 ID 必須有效
    /// - Zone 設置時 capacity 也應該設置
    pub fn stage_role_change(&mut self, config: NodeRoleConfig) -> Result<(), DomainError> {
        if config.node_id.is_empty() {
            return Err(DomainError::ValidationError("Node ID cannot be empty".to_string()));
        }

        // 如果已有該節點的變更，替換它
        self.staged_changes.retain(|c| c.node_id != config.node_id);
        self.staged_changes.push(config);
        Ok(())
    }

    /// 更新布局
    pub fn update_layout(&mut self, new_version: i64) -> Result<ClusterEvent, DomainError> {
        self.layout_version = new_version;
        self.staged_changes.clear();
        Ok(ClusterEvent::LayoutUpdated(ClusterLayoutUpdatedEvent::new(new_version)))
    }

    /// 應用布局
    ///
    /// # 業務規則
    /// - 版本必須匹配當前暫存版本
    pub fn apply_layout(&mut self, version: i64, message: Vec<String>) -> Result<ClusterEvent, DomainError> {
        self.layout_version = version;
        self.staged_changes.clear();
        Ok(ClusterEvent::LayoutApplied(ClusterLayoutAppliedEvent::new(version, message)))
    }

    /// 還原布局
    pub fn revert_layout(&mut self, reverted_version: i64) -> Result<ClusterEvent, DomainError> {
        self.staged_changes.clear();
        self.layout_version = reverted_version;
        Ok(ClusterEvent::LayoutReverted(ClusterLayoutRevertedEvent::new(reverted_version)))
    }

    /// 跳過死節點
    ///
    /// # 業務規則
    /// - allow_missing_data 為 true 時可能導致數據丟失
    pub fn skip_dead_nodes(&mut self, version: i64, allow_missing_data: bool) -> Result<ClusterEvent, DomainError> {
        if allow_missing_data {
            // 這是危險操作，可以添加額外的確認邏輯
            tracing::warn!("Skipping dead nodes with allow_missing_data=true may cause data loss");
        }
        Ok(ClusterEvent::DeadNodesSkipped(ClusterDeadNodesSkippedEvent::new(
            version,
            allow_missing_data,
        )))
    }

    // ============ Getters ============

    pub fn layout_version(&self) -> i64 {
        self.layout_version
    }

    pub fn nodes(&self) -> &[String] {
        &self.nodes
    }

    pub fn staged_changes(&self) -> &[NodeRoleConfig] {
        &self.staged_changes
    }
}
