//! Object domain entity
//!
//! Represents S3 objects within buckets

/// Object information from list operation
#[derive(Debug, Clone)]
pub struct ObjectInfo {
    pub key: String,
    pub size: i64,
    pub last_modified: String,
    pub etag: String,
    pub storage_class: String,
}

/// Object metadata from HEAD operation
#[derive(Debug, Clone)]
pub struct ObjectMetadata {
    pub content_length: i64,
    pub content_type: String,
    pub etag: String,
    pub last_modified: String,
}

/// Upload result after successful object upload
#[derive(Debug, Clone)]
pub struct UploadResult {
    pub bucket: String,
    pub key: String,
    pub etag: String,
    pub size: i64,
}

/// Download metadata for streaming download
#[derive(Debug, Clone)]
pub struct DownloadMetadata {
    pub bucket: String,
    pub key: String,
    pub content_type: String,
    pub content_length: i64,
    pub etag: String,
    pub last_modified: String,
}

/// List objects result with pagination
#[derive(Debug, Clone)]
pub struct ListObjectsResult {
    pub objects: Vec<ObjectInfo>,
    pub common_prefixes: Vec<String>,
    pub next_continuation_token: Option<String>,
    pub is_truncated: bool,
}

/// Delete objects result
#[derive(Debug, Clone)]
pub struct DeleteObjectsResult {
    pub deleted: Vec<String>,
    pub errors: Vec<DeleteObjectError>,
}

/// Delete object error information
#[derive(Debug, Clone)]
pub struct DeleteObjectError {
    pub key: String,
    pub code: String,
    pub message: String,
}

/// Copy object result
#[derive(Debug, Clone)]
pub struct CopyObjectResult {
    pub etag: String,
    pub last_modified: String,
}
