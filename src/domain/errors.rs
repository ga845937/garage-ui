//! Domain errors

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    // ============ Validation Errors ============
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    // ============ Bucket Errors ============
    
    #[error("Bucket not found: {0}")]
    BucketNotFound(String),
    
    #[error("Bucket already exists: {0}")]
    BucketAlreadyExists(String),
    
    #[error("Local alias already exists: {0}")]
    LocalAliasAlreadyExists(String),
    
    #[error("Invalid bucket name: {0}")]
    InvalidBucketName(String),
    
    // ============ Access Key Errors ============
    
    #[error("Access key not found: {0}")]
    AccessKeyNotFound(String),
    
    #[error("Access key already exists: {0}")]
    AccessKeyAlreadyExists(String),
    
    // ============ Admin Token Errors ============
    
    #[error("Admin token not found: {0}")]
    AdminTokenNotFound(String),
    
    // ============ Cluster Errors ============
    
    #[error("Cluster operation failed: {0}")]
    ClusterOperationFailed(String),
    
    #[error("Layout version mismatch: expected {expected}, got {actual}")]
    LayoutVersionMismatch { expected: i64, actual: i64 },
    
    // ============ Node Errors ============
    
    #[error("Node not found: {0}")]
    NodeNotFound(String),

    // ============ Object Errors ============
    
    #[error("Object not found: {0}")]
    ObjectNotFound(String),
    
    // ============ Infrastructure Errors ============
    
    #[error("Garage API error: {0}")]
    GarageApiError(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}
