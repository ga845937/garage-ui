//! Worker entities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Worker 資訊
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerInfo {
    pub id: i64,
    pub name: String,
    pub state: String,
    pub progress: Option<String>,
    pub errors: i64,
    pub consecutive_errors: i64,
    pub last_error: Option<WorkerError>,
    pub tranquility: Option<i64>,
    pub freeform: Option<String>,
}

/// Worker 錯誤
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerError {
    pub message: String,
    pub secs_ago: i64,
}

/// Worker 變數
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerVariable {
    pub variable: String,
    pub value: String,
}

/// Worker 變數列表
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerVariables {
    #[serde(flatten)]
    pub variables: HashMap<String, String>,
}

/// 設置變數結果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableResult {
    pub variable: String,
    pub old_value: Option<String>,
    pub new_value: String,
}
