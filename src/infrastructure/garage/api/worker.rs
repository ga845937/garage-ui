//! Worker API types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============ Request Types ============

/// 列出 Workers 請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWorkersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub busy_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_only: Option<bool>,
}

/// 獲取 Worker 資訊請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWorkerInfoRequest {
    pub id: i64,
}

/// 獲取 Worker 變數請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWorkerVariableRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable: Option<String>,
}

/// 設置 Worker 變數請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetWorkerVariableRequest {
    pub variable: String,
    pub value: String,
}

// ============ Response Types ============

/// 多節點 Worker 響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiNodeWorkerResponse<T> {
    #[serde(default)]
    pub success: HashMap<String, T>,
    #[serde(default)]
    pub error: HashMap<String, String>,
}

/// Worker 資訊響應
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerInfoResponse {
    pub id: i64,
    pub name: String,
    pub state: String,
    pub progress: Option<String>,
    pub errors: i64,
    pub consecutive_errors: i64,
    pub last_error: Option<WorkerErrorResponse>,
    pub tranquility: Option<i64>,
    pub freeform: Option<String>,
}

/// Worker 錯誤響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerErrorResponse {
    pub message: String,
    pub secs_ago: i64,
}

/// Worker 變數響應
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerVariablesResponse {
    #[serde(flatten)]
    pub variables: HashMap<String, String>,
}

/// 設置變數結果響應
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableResultResponse {
    pub variable: String,
    pub old_value: Option<String>,
    pub new_value: String,
}
