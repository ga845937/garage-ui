//! Block entities

use serde::{Deserialize, Serialize};

/// 區塊資訊
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockInfo {
    pub block_hash: String,
    pub size: i64,
    pub refcount: i64,
    #[serde(default)]
    pub versions: Vec<BlockVersionRef>,
    #[serde(default)]
    pub uploads: Vec<BlockUploadRef>,
}

/// 區塊版本引用
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockVersionRef {
    pub bucket_id: String,
    pub key: String,
    pub version_uuid: String,
    pub deleted: bool,
    pub block_offset: i64,
}

/// 區塊上傳引用
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockUploadRef {
    pub bucket_id: String,
    pub key: String,
    pub upload_id: String,
    pub part_number: i32,
    pub block_offset: i64,
}

/// 區塊錯誤
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockError {
    pub block_hash: String,
    pub error: String,
}

/// 清除區塊結果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurgeBlocksResult {
    pub blocks_purged: i64,
    pub objects_deleted: i64,
    pub uploads_deleted: i64,
}

/// 重試重同步結果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetryResyncResult {
    pub blocks_retried: i64,
}
