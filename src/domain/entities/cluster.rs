//! Cluster entities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 集群狀態
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterStatus {
    pub layout_version: i64,
    pub nodes: Vec<ClusterNode>,
}

/// 集群節點
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterNode {
    pub id: String,
    pub addr: Option<String>,
    pub is_up: bool,
    pub last_seen_secs_ago: Option<i64>,
    pub hostname: Option<String>,
    #[serde(default)]
    pub role: Option<NodeRole>,
    pub data_partition: Option<PartitionInfo>,
    pub metadata_partition: Option<PartitionInfo>,
}

/// 節點角色
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeRole {
    pub zone: String,
    pub capacity: Option<i64>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// 分區資訊
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartitionInfo {
    pub available: i64,
    pub total: i64,
}

/// 集群健康狀態
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterHealth {
    pub status: String,
    pub known_nodes: i32,
    pub connected_nodes: i32,
    pub storage_nodes: i32,
    pub storage_nodes_up: i32,
    pub partitions: i32,
    pub partitions_quorum: i32,
    pub partitions_all_ok: i32,
}

/// 集群統計資訊
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterStatistics {
    pub freeform: String,
}

/// 連接節點結果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectNodeResult {
    pub success: bool,
    pub error: Option<String>,
}

/// 集群布局
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterLayout {
    pub version: i64,
    pub partition_size: i64,
    pub roles: Vec<LayoutRole>,
    pub staged_role_changes: Vec<StagedRoleChange>,
    pub parameters: Option<LayoutParameters>,
    pub staged_parameters: Option<LayoutParameters>,
}

/// 布局角色
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutRole {
    pub id: String,
    pub zone: String,
    pub capacity: Option<i64>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// 暫存的角色變更
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StagedRoleChange {
    pub id: String,
    #[serde(flatten)]
    pub change: RoleChangeType,
}

/// 角色變更類型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RoleChangeType {
    Remove { remove: bool },
    Assign {
        zone: String,
        capacity: Option<i64>,
        #[serde(default)]
        tags: Vec<String>,
    },
}

/// 布局參數
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutParameters {
    pub zone_redundancy: Option<ZoneRedundancy>,
}

/// Zone 冗餘設定
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ZoneRedundancy {
    Value(i32),
    Maximum { maximum: bool },
}

/// 布局應用結果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyLayoutResult {
    pub layout: ClusterLayout,
    pub message: Vec<String>,
}

/// 布局歷史
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterLayoutHistory {
    pub current_version: i64,
    pub min_ack: i64,
    pub versions: Vec<LayoutVersion>,
    pub update_trackers: HashMap<String, UpdateTracker>,
}

/// 布局版本
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutVersion {
    pub version: i64,
    pub partition_size: i64,
    pub roles: Vec<LayoutRole>,
}

/// 更新追蹤器
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTracker {
    pub ack: i64,
    pub sync: i64,
}

/// 跳過死節點結果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkipDeadNodesResult {
    pub ack_updated: Vec<String>,
    pub sync_updated: Vec<String>,
}
