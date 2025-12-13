//! Node entities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 多節點響應
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiNodeResponse<T> {
    #[serde(default)]
    pub success: HashMap<String, T>,
    #[serde(default)]
    pub error: HashMap<String, String>,
}

/// 節點資訊
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfo {
    pub node_id: String,
    pub node_addr: String,
    pub zone: Option<String>,
    pub capacity: Option<i64>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub garage_version: String,
    pub garage_features: Option<Vec<String>>,
    pub rust_version: String,
    pub db_engine: String,
}

/// 節點統計資訊
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeStatistics {
    pub freeform: String,
}

/// 修復操作類型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RepairType {
    Tables,
    Blocks,
    Versions,
    MultipartUploads,
    BlockRefs,
    BlockRc,
    RebalancePartitions,
    Scrub(ScrubCommand),
}

/// Scrub 命令
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScrubCommand {
    pub command: String,
}
