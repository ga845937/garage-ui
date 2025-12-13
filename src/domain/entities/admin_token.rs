//! Admin Token entities

use serde::{Deserialize, Serialize};

/// Admin Token 列表項目
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminTokenListItem {
    pub id: String,
    pub name: Option<String>,
    pub created: Option<String>,
    pub expiration: Option<String>,
    pub expired: bool,
    #[serde(default)]
    pub scope: Vec<String>,
}

/// Admin Token 詳細資訊
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminTokenInfo {
    pub id: String,
    pub name: Option<String>,
    pub created: Option<String>,
    pub expiration: Option<String>,
    pub expired: bool,
    #[serde(default)]
    pub scope: Vec<String>,
}

/// 創建 Admin Token 響應（包含 secret）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminTokenCreated {
    pub id: String,
    pub name: Option<String>,
    pub created: Option<String>,
    pub expiration: Option<String>,
    pub expired: bool,
    #[serde(default)]
    pub scope: Vec<String>,
    pub secret_token: String,
}
