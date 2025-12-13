//! Cluster API types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============ Request Types ============

/// 連接節點請求
pub type ConnectNodesRequest = Vec<String>;

/// 更新集群布局請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLayoutRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RoleChangeRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<LayoutParametersRequest>,
}

/// 角色變更請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleChangeRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// 布局參數請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutParametersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_redundancy: Option<ZoneRedundancyRequest>,
}

/// Zone 冗餘設定請求
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ZoneRedundancyRequest {
    Value(i32),
    Maximum { maximum: bool },
}

/// 應用布局請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyLayoutRequest {
    pub version: i64,
}

/// 跳過死節點請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SkipDeadNodesRequest {
    pub version: i64,
    pub allow_missing_data: bool,
}

// ============ Response Types ============

/// 集群狀態響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterStatusResponse {
    pub layout_version: i64,
    pub nodes: Vec<ClusterNodeResponse>,
}

/// 集群節點響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterNodeResponse {
    pub id: String,
    pub addr: Option<String>,
    #[serde(default)]
    pub is_up: bool,
    pub last_seen_secs_ago: Option<i64>,
    pub hostname: Option<String>,
    pub role: Option<NodeRoleResponse>,
    pub data_partition: Option<PartitionInfoResponse>,
    pub metadata_partition: Option<PartitionInfoResponse>,
}

/// 節點角色響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeRoleResponse {
    pub zone: String,
    pub capacity: Option<i64>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// 分區資訊響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartitionInfoResponse {
    pub available: i64,
    pub total: i64,
}

/// 集群健康響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterHealthResponse {
    pub status: String,
    pub known_nodes: i32,
    pub connected_nodes: i32,
    pub storage_nodes: i32,
    pub storage_nodes_up: i32,
    pub partitions: i32,
    pub partitions_quorum: i32,
    pub partitions_all_ok: i32,
}

/// 集群統計響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterStatisticsResponse {
    pub freeform: String,
}

/// 連接節點結果響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectNodeResultResponse {
    pub success: bool,
    pub error: Option<String>,
}

/// 集群布局響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterLayoutResponse {
    pub version: i64,
    pub partition_size: i64,
    pub roles: Vec<LayoutRoleResponse>,
    #[serde(default)]
    pub staged_role_changes: Vec<StagedRoleChangeResponse>,
    pub parameters: Option<LayoutParametersResponse>,
    pub staged_parameters: Option<LayoutParametersResponse>,
}

/// 布局角色響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutRoleResponse {
    pub id: String,
    pub zone: String,
    pub capacity: Option<i64>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// 暫存角色變更響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StagedRoleChangeResponse {
    pub id: String,
    pub remove: Option<bool>,
    pub zone: Option<String>,
    pub capacity: Option<i64>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// 布局參數響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutParametersResponse {
    pub zone_redundancy: Option<ZoneRedundancyResponse>,
}

/// Zone 冗餘響應
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ZoneRedundancyResponse {
    Value(i32),
    Maximum { maximum: bool },
}

/// 應用布局結果響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyLayoutResultResponse {
    pub layout: ClusterLayoutResponse,
    pub message: Vec<String>,
}

/// 預覽布局變更響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewLayoutChangesResponse {
    pub new_layout: ClusterLayoutResponse,
    pub message: Vec<String>,
}

/// 布局歷史響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterLayoutHistoryResponse {
    pub current_version: i64,
    pub min_ack: i64,
    pub versions: Vec<LayoutVersionResponse>,
    pub update_trackers: HashMap<String, UpdateTrackerResponse>,
}

/// 布局版本響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutVersionResponse {
    pub version: i64,
    pub partition_size: i64,
    pub roles: Vec<LayoutRoleResponse>,
}

/// 更新追蹤器響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTrackerResponse {
    pub ack: i64,
    pub sync: i64,
}

/// 跳過死節點結果響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkipDeadNodesResultResponse {
    #[serde(default)]
    pub ack_updated: Vec<String>,
    #[serde(default)]
    pub sync_updated: Vec<String>,
}
