//! Bucket repository interface

use async_trait::async_trait;
use crate::domain::aggregates::BucketAggregate;
use crate::domain::entities::{ garage::GarageBucketInfo, BucketDetail };
use crate::domain::errors::DomainError;

/// Input for creating a bucket with local alias
#[derive(Debug, Clone)]
pub struct LocalAliasInput {
    pub access_key_id: String,
    pub alias: String,
    pub allow_read: bool,
    pub allow_write: bool,
    pub allow_owner: bool,
}

/// Input for creating a bucket
#[derive(Debug, Clone, Default)]
pub struct CreateBucketInput {
    pub global_alias: Option<String>,
    pub local_alias: Option<LocalAliasInput>,
}

/// Bucket repository interface
/// 
/// This trait defines the contract for bucket data access.
/// The implementation is in the infrastructure layer.
#[async_trait]
pub trait BucketRepository: Send + Sync {
    // ============ Aggregate 操作 ============
    
    /// 保存 Aggregate（創建或更新）
    async fn save(&self, aggregate: &BucketAggregate) -> Result<(), DomainError>;
    
    /// 載入 Aggregate
    async fn load(&self, id: &str) -> Result<BucketAggregate, DomainError>;
    
    // ============ 查詢操作（CQRS Read Side）============
    
    /// List all buckets (simplified view)
    async fn list(&self) -> Result<Vec<GarageBucketInfo>, DomainError>;
    
    /// Get bucket detail (for queries)
    async fn get_detail(&self, id: &str) -> Result<BucketDetail, DomainError>;
    
    // ============ Infrastructure 操作 ============
    
    /// Create a new bucket and return its ID (Garage generates the ID)
    async fn create_bucket(&self, input: CreateBucketInput) -> Result<String, DomainError>;
    
    /// Delete a bucket by ID
    async fn delete_bucket(&self, id: &str) -> Result<(), DomainError>;
    
    // ============ Alias 操作 ============
    
    /// Add a global alias to a bucket
    async fn add_global_alias(&self, bucket_id: &str, alias: &str) -> Result<BucketDetail, DomainError>;
    
    /// Add a local alias to a bucket
    async fn add_local_alias(&self, bucket_id: &str, access_key_id: &str, alias: &str) -> Result<BucketDetail, DomainError>;
    
    /// Remove a global alias from a bucket
    async fn remove_global_alias(&self, bucket_id: &str, alias: &str) -> Result<BucketDetail, DomainError>;
    
    /// Remove a local alias from a bucket
    async fn remove_local_alias(&self, bucket_id: &str, access_key_id: &str, alias: &str) -> Result<BucketDetail, DomainError>;

    // ============ Permission 操作 ============

    /// Allow a key to access a bucket with specified permissions
    async fn allow_bucket_key(
        &self,
        bucket_id: &str,
        access_key_id: &str,
        read: bool,
        write: bool,
        owner: bool,
    ) -> Result<BucketDetail, DomainError>;

    /// Deny a key from accessing a bucket with specified permissions
    async fn deny_bucket_key(
        &self,
        bucket_id: &str,
        access_key_id: &str,
        read: bool,
        write: bool,
        owner: bool,
    ) -> Result<BucketDetail, DomainError>;
}

