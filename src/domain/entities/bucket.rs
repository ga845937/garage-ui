//! Bucket entity - Core domain object for bucket management

use serde::{Deserialize, Serialize};
use crate::domain::value_objects::{LocalAlias, Quotas};

/// Bucket entity representing a storage bucket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bucket {
    pub id: String,
    pub global_aliases: Vec<String>,
    pub local_aliases: Vec<LocalAlias>,
    pub objects: u64,
    pub bytes: u64,
    pub created: String,
}

/// Detailed bucket information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketDetail {
    pub id: String,
    pub global_aliases: Vec<String>,
    pub local_aliases: Vec<LocalAlias>,
    pub website_access: bool,
    pub website_config: Option<WebsiteConfig>,
    pub keys: Vec<BucketKey>,
    pub quotas: Quotas,
    pub objects: u64,
    pub bytes: u64,
    pub created: String,
}

/// Website configuration for a bucket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteConfig {
    pub index_document: String,
    pub error_document: String,
}

/// Bucket key with permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketKey {
    pub access_key_id: String,
    pub name: String,
    pub permissions: BucketKeyPermissions,
    #[serde(default)]
    pub bucket_local_aliases: Vec<String>,
}

/// Permissions for a bucket key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketKeyPermissions {
    pub read: bool,
    pub write: bool,
    pub owner: bool,
}

impl Bucket {
    pub fn new(
        id: String,
        global_aliases: Vec<String>,
        local_aliases: Vec<LocalAlias>,
        objects: u64,
        bytes: u64,
        created: String,
    ) -> Self {
        Self {
            id,
            global_aliases,
            local_aliases,
            objects,
            bytes,
            created,
        }
    }
}

impl BucketDetail {
    pub fn new(
        id: String,
        global_aliases: Vec<String>,
        local_aliases: Vec<LocalAlias>,
        website_access: bool,
        website_config: Option<WebsiteConfig>,
        keys: Vec<BucketKey>,
        quotas: Quotas,
        objects: u64,
        bytes: u64,
        created: String,
    ) -> Self {
        Self {
            id,
            global_aliases,
            local_aliases,
            website_access,
            website_config,
            keys,
            quotas,
            objects,
            bytes,
            created,
        }
    }
}
