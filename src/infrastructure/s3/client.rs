//! S3 Client for Garage S3-compatible API
//!
//! Provides S3 operations using AWS SDK with streaming support

use aws_config::Region;
use aws_credential_types::Credentials;
use aws_sdk_s3::{
    config::Builder as S3ConfigBuilder,
    primitives::ByteStream,
    Client as S3Client,
    types::{Delete, ObjectIdentifier, CompletedMultipartUpload, CompletedPart},
};
use tracing::{error, info, debug};
use tokio::sync::mpsc;

use crate::domain::errors::DomainError;
use crate::infrastructure::config::S3Config;
use crate::shared::get_trace_id;

/// S3 client wrapper for Garage with streaming support
#[derive(Clone)]
pub struct GarageS3Client {
    client: S3Client,
}

impl GarageS3Client {
    /// Create a new S3 client from configuration
    pub async fn new(config: &S3Config) -> Self {
        let credentials = Credentials::new(
            &config.access_key_id,
            &config.secret_access_key,
            None,
            None,
            "garage-ui",
        );

        let s3_config = S3ConfigBuilder::new()
            .endpoint_url(&config.endpoint_url)
            .region(Region::new(config.region.clone()))
            .credentials_provider(credentials)
            .force_path_style(true) // Garage requires path-style access
            .behavior_version_latest()
            .build();

        let client = S3Client::from_conf(s3_config);

        Self { client }
    }

    /// Get the underlying S3 client
    pub fn inner(&self) -> &S3Client {
        &self.client
    }

    // ============ Streaming Upload ============

    /// Upload an object from a byte stream (simple upload for small files)
    pub async fn upload_object(
        &self,
        bucket: &str,
        key: &str,
        content_type: &str,
        content_length: Option<i64>,
        body: ByteStream,
    ) -> Result<UploadResult, DomainError> {
        let trace_id = get_trace_id();

        info!(
            trace_id = %trace_id,
            bucket = %bucket,
            key = %key,
            content_type = %content_type,
            content_length = ?content_length,
            "Uploading object via streaming"
        );

        let mut request = self
            .client
            .put_object()
            .bucket(bucket)
            .key(key)
            .content_type(content_type)
            .body(body);

        if let Some(len) = content_length {
            request = request.content_length(len);
        }

        let response = request.send().await.map_err(|e| {
            error!(trace_id = %trace_id, bucket = %bucket, key = %key, error = %e, "Failed to upload object");
            DomainError::InternalError(e.to_string())
        })?;

        let etag = response.e_tag().unwrap_or_default().trim_matches('"').to_string();

        info!(
            trace_id = %trace_id,
            bucket = %bucket,
            key = %key,
            etag = %etag,
            "Object uploaded successfully"
        );

        Ok(UploadResult {
            bucket: bucket.to_string(),
            key: key.to_string(),
            etag,
            size: content_length.unwrap_or(0),
        })
    }

    /// Upload an object using multipart upload for streaming large files
    /// This avoids loading the entire file into memory
    /// For very small files or empty files (like folders), uses simple upload instead
    /// 
    /// # Arguments
    /// * `bucket` - Target bucket name
    /// * `key` - Object key
    /// * `content_type` - Content type
    /// * `content_length` - Total content length (optional, used to decide upload strategy)
    /// * `chunk_receiver` - Channel receiver for streaming chunks as bytes::Bytes
    /// * `part_size` - Size of each part (default 5MB, minimum 5MB required by S3)
    /// * `progress_sender` - Optional channel to send progress updates (upload_id, part_number, bytes_uploaded)
    pub async fn upload_object_multipart(
        &self,
        bucket: &str,
        key: &str,
        content_type: &str,
        content_length: Option<i64>,
        mut chunk_receiver: mpsc::Receiver<Result<bytes::Bytes, std::io::Error>>,
        part_size: Option<usize>,
        progress_sender: Option<mpsc::Sender<UploadProgress>>,
    ) -> Result<UploadResult, DomainError> {
        let trace_id = get_trace_id();
        let part_size = part_size.unwrap_or(5 * 1024 * 1024); // Default 5MB

        info!(
            trace_id = %trace_id,
            bucket = %bucket,
            key = %key,
            content_type = %content_type,
            content_length = ?content_length,
            part_size = %part_size,
            "Starting multipart upload with streaming"
        );

        // For larger files, use multipart upload
        info!(
            trace_id = %trace_id,
            bucket = %bucket,
            key = %key,
            "Using multipart upload for streaming"
        );

        // Step 1: Initiate multipart upload
        let create_output = self
            .client
            .create_multipart_upload()
            .bucket(bucket)
            .key(key)
            .content_type(content_type)
            .send()
            .await
            .map_err(|e| {
                error!(trace_id = %trace_id, bucket = %bucket, key = %key, error = %e, "Failed to create multipart upload");
                DomainError::InternalError(e.to_string())
            })?;

        let upload_id = create_output.upload_id().ok_or_else(|| {
            DomainError::InternalError("No upload ID returned".to_string())
        })?;

        info!(
            trace_id = %trace_id,
            bucket = %bucket,
            key = %key,
            upload_id = %upload_id,
            "Multipart upload initiated"
        );

        // Send initiated message with upload_id
        if let Some(ref sender) = progress_sender {
            let _ = sender.send(UploadProgress::Initiated {
                upload_id: upload_id.to_string(),
                bucket: bucket.to_string(),
                key: key.to_string(),
            }).await;
        }

        // Step 2: Upload parts
        let mut completed_parts = Vec::new();
        let mut part_number = 1;
        let mut buffer = Vec::new();
        let mut total_size = 0i64;

        // Collect chunks into parts and upload
        while let Some(chunk_result) = chunk_receiver.recv().await {
            let chunk = match chunk_result {
                Ok(bytes) => bytes,
                Err(e) => {
                    error!(trace_id = %trace_id, error = %e, "Error receiving chunk, aborting upload");
                    // Abort multipart upload on error
                    let _ = self.client
                        .abort_multipart_upload()
                        .bucket(bucket)
                        .key(key)
                        .upload_id(upload_id)
                        .send()
                        .await;
                    return Err(DomainError::InternalError(format!("Chunk error: {}", e)));
                }
            };

            buffer.extend_from_slice(&chunk);
            total_size += chunk.len() as i64;

            // Upload when buffer reaches part size
            while buffer.len() >= part_size {
                let part_data: Vec<u8> = buffer.drain(..part_size).collect();
                
                debug!(
                    trace_id = %trace_id,
                    part_number = %part_number,
                    part_size = %part_data.len(),
                    total_size = %total_size,
                    "Uploading part"
                );

                let upload_part_output = self
                    .client
                    .upload_part()
                    .bucket(bucket)
                    .key(key)
                    .upload_id(upload_id)
                    .part_number(part_number)
                    .body(ByteStream::from(part_data))
                    .send()
                    .await
                    .map_err(|e| {
                        error!(trace_id = %trace_id, part_number = %part_number, error = %e, "Failed to upload part");
                        DomainError::InternalError(e.to_string())
                    })?;

                let etag = upload_part_output.e_tag().unwrap_or_default().to_string();
                
                completed_parts.push(
                    CompletedPart::builder()
                        .part_number(part_number)
                        .e_tag(etag)
                        .build()
                );

                debug!(
                    trace_id = %trace_id,
                    part_number = %part_number,
                    "Part uploaded successfully"
                );

                // Send progress update
                if let Some(ref sender) = progress_sender {
                    let _ = sender.send(UploadProgress::PartUploaded {
                        part_number,
                        bytes_uploaded: total_size,
                        total_bytes: content_length.unwrap_or(0),
                    }).await;
                }

                part_number += 1;
            }
        }

        // Upload remaining data as final part
        if !buffer.is_empty() {
            debug!(
                trace_id = %trace_id,
                part_number = %part_number,
                part_size = %buffer.len(),
                "Uploading final part"
            );

            let upload_part_output = self
                .client
                .upload_part()
                .bucket(bucket)
                .key(key)
                .upload_id(upload_id)
                .part_number(part_number)
                .body(ByteStream::from(buffer))
                .send()
                .await
                .map_err(|e| {
                    error!(trace_id = %trace_id, part_number = %part_number, error = %e, "Failed to upload final part");
                    DomainError::InternalError(e.to_string())
                })?;

            let etag = upload_part_output.e_tag().unwrap_or_default().to_string();
            
            completed_parts.push(
                CompletedPart::builder()
                    .part_number(part_number)
                    .e_tag(etag)
                    .build()
            );

            // Send final progress update
            if let Some(ref sender) = progress_sender {
                let _ = sender.send(UploadProgress::PartUploaded {
                    part_number,
                    bytes_uploaded: total_size,
                    total_bytes: content_length.unwrap_or(0),
                }).await;
            }
        }

        // Step 3: Complete multipart upload
        let completed_upload = CompletedMultipartUpload::builder()
            .set_parts(Some(completed_parts))
            .build();

        let complete_output = self
            .client
            .complete_multipart_upload()
            .bucket(bucket)
            .key(key)
            .upload_id(upload_id)
            .multipart_upload(completed_upload)
            .send()
            .await
            .map_err(|e| {
                error!(trace_id = %trace_id, bucket = %bucket, key = %key, error = %e, "Failed to complete multipart upload");
                DomainError::InternalError(e.to_string())
            })?;

        let etag = complete_output.e_tag().unwrap_or_default().trim_matches('"').to_string();

        info!(
            trace_id = %trace_id,
            bucket = %bucket,
            key = %key,
            etag = %etag,
            total_size = %total_size,
            parts = %(part_number - 1),
            "Multipart upload completed successfully"
        );

        Ok(UploadResult {
            bucket: bucket.to_string(),
            key: key.to_string(),
            etag,
            size: total_size,
        })
    }

    /// Abort a multipart upload
    pub async fn abort_multipart_upload(
        &self,
        bucket: &str,
        key: &str,
        upload_id: &str,
    ) -> Result<(), DomainError> {
        let trace_id = get_trace_id();

        info!(
            trace_id = %trace_id,
            bucket = %bucket,
            key = %key,
            upload_id = %upload_id,
            "Aborting multipart upload"
        );

        self.client
            .abort_multipart_upload()
            .bucket(bucket)
            .key(key)
            .upload_id(upload_id)
            .send()
            .await
            .map_err(|e| {
                error!(
                    trace_id = %trace_id,
                    bucket = %bucket,
                    key = %key,
                    upload_id = %upload_id,
                    error = %e,
                    "Failed to abort multipart upload"
                );
                DomainError::InternalError(e.to_string())
            })?;

        info!(
            trace_id = %trace_id,
            bucket = %bucket,
            key = %key,
            upload_id = %upload_id,
            "Multipart upload aborted successfully"
        );

        Ok(())
    }

    // ============ Streaming Download ============

    /// Download an object and return metadata + byte stream
    pub async fn download_object(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<DownloadResult, DomainError> {
        let trace_id = get_trace_id();

        info!(
            trace_id = %trace_id,
            bucket = %bucket,
            key = %key,
            "Downloading object via streaming"
        );

        let response = self
            .client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                error!(trace_id = %trace_id, bucket = %bucket, key = %key, error = %e, "Failed to download object");
                DomainError::ObjectNotFound(format!("{}/{}", bucket, key))
            })?;

        let metadata = DownloadMetadata {
            bucket: bucket.to_string(),
            key: key.to_string(),
            content_type: response.content_type().unwrap_or("application/octet-stream").to_string(),
            content_length: response.content_length().unwrap_or(0),
            etag: response.e_tag().unwrap_or_default().trim_matches('"').to_string(),
            last_modified: response
                .last_modified()
                .map(|dt| dt.to_string())
                .unwrap_or_default(),
        };

        info!(
            trace_id = %trace_id,
            bucket = %bucket,
            key = %key,
            content_length = metadata.content_length,
            "Object download started"
        );

        Ok(DownloadResult {
            metadata,
            body: response.body,
        })
    }

    // ============ Object Operations ============

    /// List objects in a bucket with pagination
    pub async fn list_objects(
        &self,
        bucket: &str,
        prefix: Option<&str>,
        continuation_token: Option<&str>,
        max_keys: Option<i32>,
    ) -> Result<ListObjectsOutput, DomainError> {
        let trace_id = get_trace_id();

        let mut request = self.client.list_objects_v2().bucket(bucket);

        if let Some(p) = prefix {
            request = request.prefix(p);
        }
        if let Some(token) = continuation_token {
            request = request.continuation_token(token);
        }
        if let Some(max) = max_keys {
            request = request.max_keys(max);
        }

        let response = request.send().await.map_err(|e| {
            error!(trace_id = %trace_id, bucket = %bucket, error = %e, "Failed to list objects");
            DomainError::InternalError(e.to_string())
        })?;

        let objects: Vec<ObjectInfo> = response
            .contents()
            .iter()
            .map(|obj| ObjectInfo {
                key: obj.key().unwrap_or_default().to_string(),
                size: obj.size().unwrap_or(0),
                last_modified: obj
                    .last_modified()
                    .map(|dt| dt.to_string())
                    .unwrap_or_default(),
                etag: obj.e_tag().unwrap_or_default().trim_matches('"').to_string(),
                storage_class: obj
                    .storage_class()
                    .map(|sc| sc.as_str().to_string())
                    .unwrap_or_default(),
            })
            .collect();

        let next_token = response.next_continuation_token().map(|s| s.to_string());
        let is_truncated = response.is_truncated().unwrap_or(false);
        let prefix = response.prefix().map(|s| s.to_string());

        info!(
            trace_id = %trace_id,
            bucket = %bucket,
            object_count = objects.len(),
            is_truncated = %is_truncated,
            "Listed objects"
        );

        Ok(ListObjectsOutput {
            objects,
            next_continuation_token: next_token,
            is_truncated,
            prefix,
        })
    }

    /// Get object metadata (HEAD request)
    pub async fn head_object(&self, bucket: &str, key: &str) -> Result<ObjectMetadata, DomainError> {
        let trace_id = get_trace_id();

        let response = self
            .client
            .head_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                error!(trace_id = %trace_id, bucket = %bucket, key = %key, error = %e, "Failed to get object metadata");
                DomainError::ObjectNotFound(format!("{}/{}", bucket, key))
            })?;

        info!(trace_id = %trace_id, bucket = %bucket, key = %key, "Got object metadata");

        Ok(ObjectMetadata {
            content_length: response.content_length().unwrap_or(0),
            content_type: response.content_type().unwrap_or_default().to_string(),
            etag: response.e_tag().unwrap_or_default().trim_matches('"').to_string(),
            last_modified: response
                .last_modified()
                .map(|dt| dt.to_string())
                .unwrap_or_default(),
        })
    }

    /// Delete a single object
    pub async fn delete_object(&self, bucket: &str, key: &str) -> Result<(), DomainError> {
        let trace_id = get_trace_id();

        self.client
            .delete_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                error!(trace_id = %trace_id, bucket = %bucket, key = %key, error = %e, "Failed to delete object");
                DomainError::InternalError(e.to_string())
            })?;

        info!(trace_id = %trace_id, bucket = %bucket, key = %key, "Deleted object");

        Ok(())
    }

    /// Delete multiple objects (batch delete)
    pub async fn delete_objects(
        &self,
        bucket: &str,
        keys: Vec<String>,
    ) -> Result<DeleteObjectsOutput, DomainError> {
        let trace_id = get_trace_id();

        if keys.is_empty() {
            return Ok(DeleteObjectsOutput {
                deleted: vec![],
                errors: vec![],
            });
        }

        let objects: Vec<ObjectIdentifier> = keys
            .iter()
            .map(|key| ObjectIdentifier::builder().key(key).build())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        let delete = Delete::builder()
            .set_objects(Some(objects))
            .build()
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        let response = self
            .client
            .delete_objects()
            .bucket(bucket)
            .delete(delete)
            .send()
            .await
            .map_err(|e| {
                error!(trace_id = %trace_id, bucket = %bucket, count = keys.len(), error = %e, "Failed to delete objects");
                DomainError::InternalError(e.to_string())
            })?;

        let deleted: Vec<String> = response
            .deleted()
            .iter()
            .filter_map(|d| d.key().map(|k| k.to_string()))
            .collect();

        let errors: Vec<DeleteError> = response
            .errors()
            .iter()
            .map(|e| DeleteError {
                key: e.key().unwrap_or_default().to_string(),
                code: e.code().unwrap_or_default().to_string(),
                message: e.message().unwrap_or_default().to_string(),
            })
            .collect();

        info!(
            trace_id = %trace_id,
            bucket = %bucket,
            deleted_count = deleted.len(),
            error_count = errors.len(),
            "Batch deleted objects"
        );

        Ok(DeleteObjectsOutput { deleted, errors })
    }

    /// Delete all objects under a prefix (recursive delete for folders)
    /// This will list all objects with the given prefix and delete them in batches
    pub async fn delete_objects_recursive(
        &self,
        bucket: &str,
        prefix: &str,
    ) -> Result<DeleteObjectsOutput, DomainError> {
        let trace_id = get_trace_id();
        
        info!(
            trace_id = %trace_id,
            bucket = %bucket,
            prefix = %prefix,
            "Starting recursive delete"
        );

        let mut all_deleted: Vec<String> = Vec::new();
        let mut all_errors: Vec<DeleteError> = Vec::new();
        let mut continuation_token: Option<String> = None;

        // S3 DeleteObjects API 一次最多刪除 1000 個物件
        const MAX_KEYS_PER_REQUEST: i32 = 1000;

        loop {
            // 列出所有以 prefix 開頭的物件
            let list_result = self
                .list_objects(bucket, Some(prefix), continuation_token.as_deref(), Some(MAX_KEYS_PER_REQUEST))
                .await?;

            if list_result.objects.is_empty() {
                break;
            }

            // 收集要刪除的 keys
            let keys: Vec<String> = list_result.objects.iter().map(|o| o.key.clone()).collect();

            info!(
                trace_id = %trace_id,
                bucket = %bucket,
                prefix = %prefix,
                batch_count = keys.len(),
                "Deleting batch of objects"
            );

            // 批次刪除
            let delete_result = self.delete_objects(bucket, keys).await?;
            all_deleted.extend(delete_result.deleted);
            all_errors.extend(delete_result.errors);

            // 檢查是否還有更多物件
            if !list_result.is_truncated {
                break;
            }
            continuation_token = list_result.next_continuation_token;
        }

        info!(
            trace_id = %trace_id,
            bucket = %bucket,
            prefix = %prefix,
            total_deleted = all_deleted.len(),
            total_errors = all_errors.len(),
            "Recursive delete completed"
        );

        Ok(DeleteObjectsOutput {
            deleted: all_deleted,
            errors: all_errors,
        })
    }

    /// Copy an object within the same bucket or to another bucket
    pub async fn copy_object(
        &self,
        source_bucket: &str,
        source_key: &str,
        dest_bucket: &str,
        dest_key: &str,
    ) -> Result<crate::domain::entities::CopyObjectResult, DomainError> {
        let trace_id = get_trace_id();
        let copy_source = format!("{}/{}", source_bucket, source_key);

        let output = self.client
            .copy_object()
            .copy_source(&copy_source)
            .bucket(dest_bucket)
            .key(dest_key)
            .send()
            .await
            .map_err(|e| {
                error!(
                    trace_id = %trace_id,
                    source = %copy_source,
                    dest_bucket = %dest_bucket,
                    dest_key = %dest_key,
                    error = %e,
                    "Failed to copy object"
                );
                DomainError::InternalError(e.to_string())
            })?;

        let copy_result = output.copy_object_result();
        let etag = copy_result
            .and_then(|r| r.e_tag())
            .unwrap_or("")
            .trim_matches('"')
            .to_string();
        let last_modified = copy_result
            .and_then(|r| r.last_modified())
            .map(|dt| dt.to_string())
            .unwrap_or_default();

        info!(
            trace_id = %trace_id,
            source = %copy_source,
            dest_bucket = %dest_bucket,
            dest_key = %dest_key,
            etag = %etag,
            "Copied object"
        );

        Ok(crate::domain::entities::CopyObjectResult {
            etag,
            last_modified,
        })
    }

    // ============ PreSigned URL Operations ============

    /// Generate a presigned URL for uploading an object
    pub async fn generate_presigned_upload_url(
        &self,
        bucket: &str,
        key: &str,
        content_type: Option<&str>,
        expires_in_seconds: u64,
    ) -> Result<String, DomainError> {
        use aws_sdk_s3::presigning::PresigningConfig;
        use std::time::Duration;

        let trace_id = get_trace_id();

        let presigning_config = PresigningConfig::builder()
            .expires_in(Duration::from_secs(expires_in_seconds))
            .build()
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        let mut request = self.client
            .put_object()
            .bucket(bucket)
            .key(key);

        if let Some(ct) = content_type {
            request = request.content_type(ct);
        }

        let presigned = request
            .presigned(presigning_config)
            .await
            .map_err(|e| {
                error!(
                    trace_id = %trace_id,
                    bucket = %bucket,
                    key = %key,
                    error = %e,
                    "Failed to generate presigned upload URL"
                );
                DomainError::InternalError(e.to_string())
            })?;

        info!(
            trace_id = %trace_id,
            bucket = %bucket,
            key = %key,
            expires_in = expires_in_seconds,
            "Generated presigned upload URL"
        );

        Ok(presigned.uri().to_string())
    }

    /// Generate a presigned URL for downloading an object
    pub async fn generate_presigned_download_url(
        &self,
        bucket: &str,
        key: &str,
        expires_in_seconds: u64,
    ) -> Result<String, DomainError> {
        use aws_sdk_s3::presigning::PresigningConfig;
        use std::time::Duration;

        let trace_id = get_trace_id();

        let presigning_config = PresigningConfig::builder()
            .expires_in(Duration::from_secs(expires_in_seconds))
            .build()
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        let presigned = self.client
            .get_object()
            .bucket(bucket)
            .key(key)
            .presigned(presigning_config)
            .await
            .map_err(|e| {
                error!(
                    trace_id = %trace_id,
                    bucket = %bucket,
                    key = %key,
                    error = %e,
                    "Failed to generate presigned download URL"
                );
                DomainError::InternalError(e.to_string())
            })?;

        info!(
            trace_id = %trace_id,
            bucket = %bucket,
            key = %key,
            expires_in = expires_in_seconds,
            "Generated presigned download URL"
        );

        Ok(presigned.uri().to_string())
    }
}

// ============ Output Types ============

/// Upload progress messages for streaming feedback
#[derive(Debug, Clone)]
pub enum UploadProgress {
    /// Upload initiated with upload_id
    Initiated {
        upload_id: String,
        bucket: String,
        key: String,
    },
    /// A part was uploaded
    PartUploaded {
        part_number: i32,
        bytes_uploaded: i64,
        total_bytes: i64,
    },
}

/// Upload result
#[derive(Debug, Clone)]
pub struct UploadResult {
    pub bucket: String,
    pub key: String,
    pub etag: String,
    pub size: i64,
}

/// Download metadata
#[derive(Debug, Clone)]
pub struct DownloadMetadata {
    pub bucket: String,
    pub key: String,
    pub content_type: String,
    pub content_length: i64,
    pub etag: String,
    pub last_modified: String,
}

/// Download result with body stream
pub struct DownloadResult {
    pub metadata: DownloadMetadata,
    pub body: ByteStream,
}

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

/// List objects output with pagination
#[derive(Debug, Clone)]
pub struct ListObjectsOutput {
    pub objects: Vec<ObjectInfo>,
    pub next_continuation_token: Option<String>,
    pub is_truncated: bool,
    pub prefix: Option<String>,
}

/// Delete objects output
#[derive(Debug, Clone)]
pub struct DeleteObjectsOutput {
    pub deleted: Vec<String>,
    pub errors: Vec<DeleteError>,
}

/// Delete error information
#[derive(Debug, Clone)]
pub struct DeleteError {
    pub key: String,
    pub code: String,
    pub message: String,
}
