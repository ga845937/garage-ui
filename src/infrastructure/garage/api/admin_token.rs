//! Admin Token API types

use serde::{Deserialize, Serialize};

// ============ Request Types ============

/// 創建 Admin Token 請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAdminTokenRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub never_expires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
}

/// 更新 Admin Token 請求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAdminTokenRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub never_expires: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
}

// ============ Response Types ============

/// Admin Token 列表項目響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminTokenListItemResponse {
    pub id: String,
    pub name: Option<String>,
    pub created: Option<String>,
    pub expiration: Option<String>,
    #[serde(default)]
    pub expired: bool,
    #[serde(default)]
    pub scope: Vec<String>,
}

/// Admin Token 資訊響應
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminTokenInfoResponse {
    pub id: String,
    pub name: Option<String>,
    pub created: Option<String>,
    pub expiration: Option<String>,
    #[serde(default)]
    pub expired: bool,
    #[serde(default)]
    pub scope: Vec<String>,
}

/// 創建 Admin Token 響應（包含 secret）
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminTokenCreatedResponse {
    pub id: String,
    pub name: Option<String>,
    pub created: Option<String>,
    pub expiration: Option<String>,
    #[serde(default)]
    pub expired: bool,
    #[serde(default)]
    pub scope: Vec<String>,
    pub secret_token: String,
}
