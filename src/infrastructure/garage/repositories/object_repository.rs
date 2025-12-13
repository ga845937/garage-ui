//! Object repository implementation using S3 client
//!
//! Implements ObjectRepository using GarageS3Client with streaming support

use async_trait::async_trait;
use aws_sdk_s3::primitives::ByteStream;
use tokio::sync::mpsc;

use crate::domain::entities::{
    CopyObjectResult, DeleteObjectError, DeleteObjectsResult, DownloadMetadata, ListObjectsResult,
    ObjectInfo, ObjectMetadata, UploadResult,
};
use crate::domain::errors::DomainError;
use crate::domain::repositories::{DownloadResult, ObjectRepository};
use crate::infrastructure::config::S3Config;
use crate::infrastructure::s3::{GarageS3Client, UploadProgress};

/// Implementation of ObjectRepository using S3 client
pub struct GarageObjectRepository {
    client: GarageS3Client,
}

impl GarageObjectRepository {
    /// Create a new ObjectRepository with the given S3 configuration
    pub async fn new(config: &S3Config) -> Self {
        let client = GarageS3Client::new(config).await;
        Self { client }
    }

    /// Create from an existing S3 client
    pub fn from_client(client: GarageS3Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl ObjectRepository for GarageObjectRepository {
    // ============ Query Operations ============

    async fn list(
        &self,
        bucket: &str,
        prefix: Option<&str>,
        continuation_token: Option<&str>,
        max_keys: Option<i32>,
    ) -> Result<ListObjectsResult, DomainError> {
        let output = self
            .client
            .list_objects(bucket, prefix, continuation_token, max_keys)
            .await?;

        Ok(ListObjectsResult {
            objects: output
                .objects
                .into_iter()
                .map(|o| ObjectInfo {
                    key: o.key,
                    size: o.size,
                    last_modified: o.last_modified,
                    etag: o.etag,
                    storage_class: o.storage_class,
                })
                .collect(),
            next_continuation_token: output.next_continuation_token,
            is_truncated: output.is_truncated,
        })
    }

    async fn get_metadata(&self, bucket: &str, key: &str) -> Result<ObjectMetadata, DomainError> {
        let metadata = self.client.head_object(bucket, key).await?;

        Ok(ObjectMetadata {
            content_length: metadata.content_length,
            content_type: metadata.content_type,
            etag: metadata.etag,
            last_modified: metadata.last_modified,
        })
    }

    // ============ Streaming Operations ============

    async fn upload(
        &self,
        bucket: &str,
        key: &str,
        content_type: &str,
        content_length: Option<i64>,
        body: ByteStream,
    ) -> Result<UploadResult, DomainError> {
        let result = self
            .client
            .upload_object(bucket, key, content_type, content_length, body)
            .await?;

        Ok(UploadResult {
            bucket: result.bucket,
            key: result.key,
            etag: result.etag,
            size: result.size,
        })
    }

    async fn upload_multipart(
        &self,
        bucket: &str,
        key: &str,
        content_type: &str,
        content_length: Option<i64>,
        chunk_receiver: mpsc::Receiver<Result<bytes::Bytes, std::io::Error>>,
        progress_sender: Option<mpsc::Sender<UploadProgress>>,
    ) -> Result<UploadResult, DomainError> {
        let result = self
            .client
            .upload_object_multipart(bucket, key, content_type, content_length, chunk_receiver, None, progress_sender)
            .await?;

        Ok(UploadResult {
            bucket: result.bucket,
            key: result.key,
            etag: result.etag,
            size: result.size,
        })
    }

    async fn download(&self, bucket: &str, key: &str) -> Result<DownloadResult, DomainError> {
        let result = self.client.download_object(bucket, key).await?;

        Ok(DownloadResult {
            metadata: DownloadMetadata {
                bucket: result.metadata.bucket,
                key: result.metadata.key,
                content_type: result.metadata.content_type,
                content_length: result.metadata.content_length,
                etag: result.metadata.etag,
                last_modified: result.metadata.last_modified,
            },
            body: result.body,
        })
    }

    // ============ Command Operations ============

    async fn delete(&self, bucket: &str, key: &str) -> Result<(), DomainError> {
        self.client.delete_object(bucket, key).await
    }

    async fn delete_batch(
        &self,
        bucket: &str,
        keys: Vec<String>,
    ) -> Result<DeleteObjectsResult, DomainError> {
        let output = self.client.delete_objects(bucket, keys).await?;

        Ok(DeleteObjectsResult {
            deleted: output.deleted,
            errors: output
                .errors
                .into_iter()
                .map(|e| DeleteObjectError {
                    key: e.key,
                    code: e.code,
                    message: e.message,
                })
                .collect(),
        })
    }

    async fn delete_recursive(
        &self,
        bucket: &str,
        prefix: &str,
    ) -> Result<DeleteObjectsResult, DomainError> {
        let output = self.client.delete_objects_recursive(bucket, prefix).await?;

        Ok(DeleteObjectsResult {
            deleted: output.deleted,
            errors: output
                .errors
                .into_iter()
                .map(|e| DeleteObjectError {
                    key: e.key,
                    code: e.code,
                    message: e.message,
                })
                .collect(),
        })
    }

    async fn copy(
        &self,
        source_bucket: &str,
        source_key: &str,
        dest_bucket: &str,
        dest_key: &str,
    ) -> Result<CopyObjectResult, DomainError> {
        let result = self.client
            .copy_object(source_bucket, source_key, dest_bucket, dest_key)
            .await?;
        
        Ok(result)
    }

    async fn abort_upload(
        &self,
        bucket: &str,
        key: &str,
        upload_id: &str,
    ) -> Result<(), DomainError> {
        self.client
            .abort_multipart_upload(bucket, key, upload_id)
            .await
    }

    // ============ PreSigned URL Operations ============

    async fn generate_presigned_upload_url(
        &self,
        bucket: &str,
        key: &str,
        content_type: Option<&str>,
        expires_in_seconds: u64,
    ) -> Result<String, DomainError> {
        self.client
            .generate_presigned_upload_url(bucket, key, content_type, expires_in_seconds)
            .await
    }

    async fn generate_presigned_download_url(
        &self,
        bucket: &str,
        key: &str,
        expires_in_seconds: u64,
    ) -> Result<String, DomainError> {
        self.client
            .generate_presigned_download_url(bucket, key, expires_in_seconds)
            .await
    }
}
