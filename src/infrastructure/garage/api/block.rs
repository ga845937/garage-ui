//! Block API types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============ Request Types ============

/// 獲取區塊資訊請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockInfoRequest {
    pub block_hash: String,
}

/// 清除區塊請求
pub type PurgeBlocksRequest = Vec<String>;

/// 重試區塊重同步請求
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum RetryBlockResyncRequest {
    All { all: bool },
    Specific { block_hashes: Vec<String> },
}

// ============ Response Types ============

/// 多節點區塊響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiNodeBlockResponse<T> {
    #[serde(default)]
    pub success: HashMap<String, T>,
    #[serde(default)]
    pub error: HashMap<String, String>,
}

/// 區塊資訊響應
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockInfoResponse {
    pub block_hash: String,
    pub size: i64,
    pub refcount: i64,
    #[serde(default)]
    pub versions: Vec<BlockVersionRefResponse>,
    #[serde(default)]
    pub uploads: Vec<BlockUploadRefResponse>,
}

/// 區塊版本引用響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockVersionRefResponse {
    pub bucket_id: String,
    pub key: String,
    pub version_uuid: String,
    #[serde(default)]
    pub deleted: bool,
    pub block_offset: i64,
}

/// 區塊上傳引用響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockUploadRefResponse {
    pub bucket_id: String,
    pub key: String,
    pub upload_id: String,
    pub part_number: i32,
    pub block_offset: i64,
}

/// 區塊錯誤響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockErrorResponse {
    pub block_hash: String,
    pub error: String,
}

/// 清除區塊結果響應
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurgeBlocksResultResponse {
    pub blocks_purged: i64,
    pub objects_deleted: i64,
    pub uploads_deleted: i64,
}

/// 重試重同步結果響應
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetryResyncResultResponse {
    pub blocks_retried: i64,
}
