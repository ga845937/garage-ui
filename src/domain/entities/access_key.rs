//! Access Key Read Models (DTOs)
//!
//! 用於查詢操作的唯讀資料結構
//! 這些不是 DDD Entity，而是 Read Model / DTO

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::domain::aggregates::AccessKeyAggregate;

// ============ List Item Read Model ============

/// Access Key 列表項目 (Read Model)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessKeyListItem {
    pub id: String,
    pub name: String,
    pub created: DateTime<Utc>,
    pub expiration: Option<DateTime<Utc>>,
    pub expired: bool,
    pub secret_access_key: String,
}

impl AccessKeyListItem {
    pub fn from_aggregate(aggregate: &AccessKeyAggregate) -> Self {
        AccessKeyListItem {
            id: aggregate.id().to_string(),
            name: aggregate.name().to_string(),
            created: aggregate.created(),
            expiration: aggregate.expiration(),
            expired: aggregate.is_expired(),
            secret_access_key: aggregate.secret_access_key().to_string(),
        }
    }

    /// 計算是否過期
    pub fn compute_expired(expiration: Option<DateTime<Utc>>) -> bool {
        expiration.map(|e| e < Utc::now()).unwrap_or(false)
    }
}

// ============ Detail Read Model ============

/// Access Key 詳細資訊 (Read Model)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessKey {
    pub id: String,
    pub name: String,
    pub secret_access_key: String,
    pub created: DateTime<Utc>,
    pub expiration: Option<DateTime<Utc>>,
    pub expired: bool,
    pub permissions: KeyPermissions,
    #[serde(default)]
    pub buckets: Vec<KeyBucket>,
}

impl AccessKey {
    pub fn from_aggregate(aggregate: AccessKeyAggregate) -> Self {
        AccessKey {
            id: aggregate.id().to_string(),
            name: aggregate.name().to_string(),
            secret_access_key: aggregate.secret_access_key().to_string(),
            created: aggregate.created(),
            expiration: aggregate.expiration(),
            expired: aggregate.is_expired(),
            permissions: KeyPermissions {
                create_bucket: aggregate.can_create_bucket(),
            },
            buckets: aggregate.buckets().iter().map(|b| {
                KeyBucket {
                    id: b.id().to_string(),
                    global_aliases: b.global_aliases().clone(),
                    local_aliases: b.local_aliases().clone(),
                    permissions: BucketPermissions {
                        owner: b.permissions().owner(),
                        read: b.permissions().read(),
                        write: b.permissions().write(),
                    },
                }
            }).collect(),
        }
    }

    /// 格式化日期為 RFC3339 字串（用於 gRPC 回應）
    pub fn created_string(&self) -> String {
        self.created.to_rfc3339()
    }

    /// 格式化日期為 RFC3339 字串（用於 gRPC 回應）
    pub fn expiration_string(&self) -> Option<String> {
        self.expiration.map(|dt| dt.to_rfc3339())
    }
}

// ============ Nested Types ============

/// Key 全局權限
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyPermissions {
    pub create_bucket: bool,
}

/// Key 關聯的 Bucket 資訊
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyBucket {
    pub id: String,
    #[serde(default)]
    pub global_aliases: Vec<String>,
    #[serde(default)]
    pub local_aliases: Vec<String>,
    pub permissions: BucketPermissions,
}

/// Bucket 權限
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BucketPermissions {
    pub read: bool,
    pub write: bool,
    pub owner: bool,
}
