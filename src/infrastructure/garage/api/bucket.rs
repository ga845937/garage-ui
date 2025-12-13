//! Bucket API types (擴展)

use serde::{Deserialize, Serialize};

// ============ Request Types ============

/// 添加 Bucket Alias 請求（全局）
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddGlobalAliasRequest {
    pub bucket_id: String,
    pub global_alias: String,
}

/// 添加 Bucket Alias 請求（本地）
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddLocalAliasRequest {
    pub bucket_id: String,
    pub access_key_id: String,
    pub local_alias: String,
}

/// 移除 Bucket Alias 請求（全局）
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveGlobalAliasRequest {
    pub bucket_id: String,
    pub global_alias: String,
}

/// 移除 Bucket Alias 請求（本地）
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveLocalAliasRequest {
    pub bucket_id: String,
    pub access_key_id: String,
    pub local_alias: String,
}

/// 允許 Bucket Key 請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowBucketKeyRequest {
    pub bucket_id: String,
    pub access_key_id: String,
    pub permissions: BucketKeyPermRequest,
}

/// 拒絕 Bucket Key 請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DenyBucketKeyRequest {
    pub bucket_id: String,
    pub access_key_id: String,
    pub permissions: BucketKeyPermRequest,
}

/// Bucket Key 權限請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BucketKeyPermRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
}

/// 清理未完成上傳請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanupIncompleteUploadsRequest {
    pub bucket_id: String,
    pub older_than_secs: i64,
}

// ============ Response Types ============

/// 清理未完成上傳響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanupIncompleteUploadsResponse {
    pub uploads_deleted: i64,
}

/// 檢查對象響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InspectObjectResponse {
    pub bucket_id: String,
    pub key: String,
    pub versions: Vec<ObjectVersionResponse>,
}

/// 對象版本響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectVersionResponse {
    pub uuid: String,
    pub timestamp: i64,
    pub state: ObjectStateResponse,
}

/// 對象狀態響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectStateResponse {
    #[serde(rename = "type")]
    pub state_type: String,
    pub size: Option<i64>,
    pub content_type: Option<String>,
    pub etag: Option<String>,
    pub blocks: Option<Vec<ObjectBlockResponse>>,
}

/// 對象區塊響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectBlockResponse {
    pub hash: String,
    pub size: i64,
    pub offset: i64,
}

/// 擴展的 Bucket 詳情響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BucketDetailExtResponse {
    pub id: String,
    #[serde(default)]
    pub global_aliases: Vec<String>,
    #[serde(default)]
    pub keys: Vec<BucketKeyInfoResponse>,
    pub bytes: i64,
    pub objects: i64,
    pub created: Option<String>,
    pub unfinished_uploads: i64,
    pub unfinished_multipart_uploads: i64,
    pub unfinished_multipart_upload_parts: i64,
    pub unfinished_multipart_upload_bytes: i64,
    #[serde(default)]
    pub website_access: bool,
    pub website_config: Option<WebsiteConfigResponse>,
    pub quotas: Option<BucketQuotasResponse>,
}

/// Bucket Key 資訊響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BucketKeyInfoResponse {
    pub access_key_id: String,
    pub name: Option<String>,
    pub permissions: BucketKeyPermResponse,
    #[serde(default)]
    pub bucket_local_aliases: Vec<String>,
}

/// Bucket Key 權限響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BucketKeyPermResponse {
    #[serde(default)]
    pub read: bool,
    #[serde(default)]
    pub write: bool,
    #[serde(default)]
    pub owner: bool,
}

/// 網站配置響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebsiteConfigResponse {
    pub index_document: String,
    pub error_document: String,
}

/// Bucket 配額響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BucketQuotasResponse {
    pub max_size: Option<i64>,
    pub max_objects: Option<i64>,
}
