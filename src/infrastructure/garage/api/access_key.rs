//! Access Key API types and operations

use serde::{Deserialize, Serialize};

// ============ Request Types ============

/// 創建 Key 請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateKeyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub never_expires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<KeyPermRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny: Option<KeyPermRequest>,
}

/// Key 權限請求
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyPermRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_bucket: Option<bool>,
}

/// 更新 Key 請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateKeyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub never_expires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<KeyPermRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny: Option<KeyPermRequest>,
}

/// 導入 Key 請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportKeyRequest {
    pub access_key_id: String,
    pub secret_access_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

// ============ Response Types ============

/// Key 列表項目響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyListItemResponse {
    pub id: String,
    pub name: String,
    pub created: String,
    pub expiration: Option<String>,
    #[serde(default)]
    pub expired: bool,
}

/// Key 詳細資訊響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyInfoResponse {
    pub access_key_id: String,
    pub name: String,
    pub secret_access_key: String,
    pub created: String,
    pub expiration: Option<String>,
    #[serde(default)]
    pub expired: bool,
    pub permissions: KeyPermissionsResponse,
    #[serde(default)]
    pub buckets: Vec<KeyBucketResponse>,
}

/// Key 更新後的資訊響應（不含 secretAccessKey）
/// 
/// Garage API 的 Update 操作不會返回 secretAccessKey，
/// 因為 secret key 只在創建時返回一次，之後無法再次取得。
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyUpdateResponse {
    pub access_key_id: String,
    pub name: String,
    pub created: String,
    pub expiration: Option<String>,
    #[serde(default)]
    pub expired: bool,
    pub permissions: KeyPermissionsResponse,
    #[serde(default)]
    pub buckets: Vec<KeyBucketResponse>,
}

/// Key 權限響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyPermissionsResponse {
    #[serde(default)]
    pub create_bucket: bool,
}

/// Key 關聯的 Bucket 響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyBucketResponse {
    pub id: String,
    #[serde(default)]
    pub global_aliases: Vec<String>,
    #[serde(default)]
    pub local_aliases: Vec<String>,
    pub permissions: BucketPermissionsResponse,
}

/// Bucket 權限響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BucketPermissionsResponse {
    #[serde(default)]
    pub read: bool,
    #[serde(default)]
    pub write: bool,
    #[serde(default)]
    pub owner: bool,
}
