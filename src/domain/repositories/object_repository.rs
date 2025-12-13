//! Object Repository trait
//!
//! Abstract interface for S3 object operations with streaming support

use async_trait::async_trait;
use aws_sdk_s3::primitives::ByteStream;
use tokio::sync::mpsc;

use crate::domain::entities::{
    CopyObjectResult, DeleteObjectsResult, DownloadMetadata, ListObjectsResult, ObjectMetadata, UploadResult,
};
use crate::domain::errors::DomainError;
use crate::infrastructure::s3::UploadProgress;

/// Download result containing metadata and body stream
pub struct DownloadResult {
    pub metadata: DownloadMetadata,
    pub body: ByteStream,
}

/// Object Repository interface for S3 operations
#[async_trait]
pub trait ObjectRepository: Send + Sync {
    // ============ Query Operations ============

    /// List objects in a bucket with optional prefix and pagination
    async fn list(
        &self,
        bucket: &str,
        prefix: Option<&str>,
        continuation_token: Option<&str>,
        max_keys: Option<i32>,
    ) -> Result<ListObjectsResult, DomainError>;

    /// Get object metadata (HEAD request)
    async fn get_metadata(&self, bucket: &str, key: &str) -> Result<ObjectMetadata, DomainError>;

    // ============ Streaming Operations ============

    /// Upload an object from a byte stream (simple upload, suitable for small files)
    async fn upload(
        &self,
        bucket: &str,
        key: &str,
        content_type: &str,
        content_length: Option<i64>,
        body: ByteStream,
    ) -> Result<UploadResult, DomainError>;

    /// Upload an object using multipart upload for streaming large files
    /// This avoids loading the entire file into memory
    /// progress_sender - Optional channel to send progress updates (upload_id, part uploads)
    async fn upload_multipart(
        &self,
        bucket: &str,
        key: &str,
        content_type: &str,
        content_length: Option<i64>,
        chunk_receiver: mpsc::Receiver<Result<bytes::Bytes, std::io::Error>>,
        progress_sender: Option<mpsc::Sender<UploadProgress>>,
    ) -> Result<UploadResult, DomainError>;

    /// Download an object as a byte stream
    async fn download(&self, bucket: &str, key: &str) -> Result<DownloadResult, DomainError>;

    // ============ Command Operations ============

    /// Delete a single object
    async fn delete(&self, bucket: &str, key: &str) -> Result<(), DomainError>;

    /// Delete multiple objects (batch)
    async fn delete_batch(
        &self,
        bucket: &str,
        keys: Vec<String>,
    ) -> Result<DeleteObjectsResult, DomainError>;

    /// Delete all objects under a prefix (recursive delete for folders)
    async fn delete_recursive(
        &self,
        bucket: &str,
        prefix: &str,
    ) -> Result<DeleteObjectsResult, DomainError>;

    /// Copy an object
    async fn copy(
        &self,
        source_bucket: &str,
        source_key: &str,
        dest_bucket: &str,
        dest_key: &str,
    ) -> Result<CopyObjectResult, DomainError>;

    /// Abort a multipart upload
    async fn abort_upload(
        &self,
        bucket: &str,
        key: &str,
        upload_id: &str,
    ) -> Result<(), DomainError>;

    // ============ PreSigned URL Operations ============

    /// Generate a presigned URL for uploading an object
    async fn generate_presigned_upload_url(
        &self,
        bucket: &str,
        key: &str,
        content_type: Option<&str>,
        expires_in_seconds: u64,
    ) -> Result<String, DomainError>;

    /// Generate a presigned URL for downloading an object
    async fn generate_presigned_download_url(
        &self,
        bucket: &str,
        key: &str,
        expires_in_seconds: u64,
    ) -> Result<String, DomainError>;
}
