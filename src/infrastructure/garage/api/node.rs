//! Node API types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============ Request Types ============

/// 修復操作請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchRepairRequest {
    pub repair_type: RepairTypeRequest,
}

/// 修復類型請求
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum RepairTypeRequest {
    Simple(String),
    Scrub {
        scrub: ScrubCommandRequest,
    },
}

/// Scrub 命令請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScrubCommandRequest {
    pub command: String,
}

// ============ Response Types ============

/// 多節點響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiNodeResponse<T> {
    #[serde(default)]
    pub success: HashMap<String, T>,
    #[serde(default)]
    pub error: HashMap<String, String>,
}

/// 節點資訊響應
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfoResponse {
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

/// 節點統計響應
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeStatisticsResponse {
    pub freeform: String,
}
