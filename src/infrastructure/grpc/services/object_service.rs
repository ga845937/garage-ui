//! Object gRPC service implementation with streaming and presigned URL support

use std::pin::Pin;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};
use tracing::{error, info};
use aws_sdk_s3::primitives::ByteStream;

use crate::application::commands::object::{CopyObjectCommand, DeleteObjectsCommand};
use crate::application::commands::object::handlers::{CopyObjectHandler, DeleteObjectsHandler};
use crate::application::queries::object::{GetObjectMetadataQuery, ListObjectsQuery};
use crate::application::queries::object::handlers::{GetObjectMetadataHandler, ListObjectsHandler};
use crate::infrastructure::grpc::conversions::domain_error_to_status;
use crate::domain::repositories::ObjectRepository;
use crate::grpc_log;
use crate::shared::get_trace_id;

use crate::infrastructure::grpc::generated::object::{
    upload_chunk_request::Data as UploadData,
    upload_chunk_response::Data as UploadResponseData,
    download_chunk_response::Data as DownloadData,
    object_service_server::ObjectService,
    // Responses
    ListObjectsResponse, ObjectMetadataResponse,
    DeleteObjectResponse, CopyObjectResponse, PreSignedUrlResponse,
    AbortUploadResponse,
    // Requests
    ListObjectsRequest, GetObjectMetadataRequest,
    UploadChunkRequest, UploadChunkResponse, DownloadObjectRequest,
    GetUploadUrlRequest, GetDownloadUrlRequest,
    DeleteObjectRequest, CopyObjectRequest, AbortUploadRequest,
    // Messages
    ObjectInfo, ObjectMetadata, UploadResult, UploadInitiated, UploadProgress as ProtoUploadProgress,
    CopyResult, DeleteError, PreSignedUrl,
    DownloadMetadata, DownloadChunkResponse,
};
use crate::infrastructure::s3::UploadProgress;

/// Default presigned URL expiration (1 hour)
const DEFAULT_PRESIGNED_EXPIRATION: i32 = 3600;

/// gRPC service for object operations with streaming and presigned URL support
pub struct ObjectGrpcService {
    // Query handlers
    list_objects_handler: Arc<ListObjectsHandler>,
    get_object_metadata_handler: Arc<GetObjectMetadataHandler>,
    // Command handlers
    delete_objects_handler: Arc<DeleteObjectsHandler>,
    copy_object_handler: Arc<CopyObjectHandler>,
    // Object repository for streaming and presigned URL operations
    object_repository: Arc<dyn ObjectRepository>,
}

impl ObjectGrpcService {
    pub fn new(
        list_objects_handler: Arc<ListObjectsHandler>,
        get_object_metadata_handler: Arc<GetObjectMetadataHandler>,
        delete_objects_handler: Arc<DeleteObjectsHandler>,
        copy_object_handler: Arc<CopyObjectHandler>,
        object_repository: Arc<dyn ObjectRepository>,
    ) -> Self {
        Self {
            list_objects_handler,
            get_object_metadata_handler,
            delete_objects_handler,
            copy_object_handler,
            object_repository,
        }
    }
}

#[tonic::async_trait]
impl ObjectService for ObjectGrpcService {
    // ============ Query Operations ============

    async fn list_objects(
        &self,
        request: Request<ListObjectsRequest>,
    ) -> Result<Response<ListObjectsResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("ObjectService", "ListObjects", &req);
        let trace_id = get_trace_id();

        let query = ListObjectsQuery::new(
            req.bucket,
            req.prefix,
            req.continuation_token,
            req.max_keys,
        )
        .map_err(domain_error_to_status)?;

        let result = self
            .list_objects_handler
            .handle(query)
            .await
            .map_err(domain_error_to_status)?;

        let response = ListObjectsResponse {
            trace_id: trace_id.to_string(),
            data: result
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
            next_continuation_token: result.next_continuation_token,
            is_truncated: result.is_truncated,
        };

        log.ok(&response);
        Ok(Response::new(response))
    }

    async fn get_object_metadata(
        &self,
        request: Request<GetObjectMetadataRequest>,
    ) -> Result<Response<ObjectMetadataResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("ObjectService", "GetObjectMetadata", &req);
        let trace_id = get_trace_id();

        let query = GetObjectMetadataQuery::new(req.bucket, req.key)
            .map_err(domain_error_to_status)?;

        let metadata = self
            .get_object_metadata_handler
            .handle(query)
            .await
            .map_err(domain_error_to_status)?;

        let response = ObjectMetadataResponse {
            trace_id: trace_id.to_string(),
            data: Some(ObjectMetadata {
                content_length: metadata.content_length,
                content_type: metadata.content_type,
                etag: metadata.etag,
                last_modified: metadata.last_modified,
            }),
        };

        log.ok(&response);
        Ok(Response::new(response))
    }

    // ============ Streaming Operations ============

    type UploadObjectStream = Pin<Box<dyn Stream<Item = Result<UploadChunkResponse, Status>> + Send>>;

    /// Upload object using bidirectional streaming
    /// Client sends metadata first, server responds with upload_id, then client sends chunks
    /// Server sends progress updates and final result
    async fn upload_object(
        &self,
        request: Request<Streaming<UploadChunkRequest>>,
    ) -> Result<Response<Self::UploadObjectStream>, Status> {
        let trace_id = get_trace_id();
        let mut stream = request.into_inner();

        // First message must be metadata
        let first_msg: UploadChunkRequest = stream
            .next()
            .await
            .ok_or_else(|| Status::invalid_argument("Empty stream"))?
            .map_err(|e| Status::internal(format!("Stream error: {}", e)))?;

        let metadata = match first_msg.data {
            Some(UploadData::Metadata(m)) => m,
            _ => return Err(Status::invalid_argument("First message must be metadata")),
        };

        info!(
            trace_id = %trace_id,
            bucket = %metadata.bucket,
            key = %metadata.key,
            content_type = %metadata.content_type,
            content_length = metadata.content_length,
            "Starting bidirectional streaming upload"
        );

        if metadata.content_length == 0 {
            self.object_repository
                .upload(
                    &metadata.bucket,
                    &metadata.key,
                    &metadata.content_type,
                    Some(0),
                    ByteStream::from(vec![]),
                )
                .await
                .map_err(domain_error_to_status)?;

            let response = UploadChunkResponse {
                trace_id: trace_id.clone(),
                data: Some(UploadResponseData::Initiated(UploadInitiated {
                    upload_id: "folder_upload".to_string(),
                    bucket: metadata.bucket,
                    key: metadata.key,
                })),
            };
            let output_stream = futures::stream::once(async move { Ok(response) });
            return Ok(Response::new(Box::pin(output_stream)));
        }

        // Create channel for streaming chunks to S3
        let (chunk_tx, chunk_rx) = mpsc::channel::<Result<bytes::Bytes, std::io::Error>>(32);
        
        // Create channel for progress updates from S3 client
        let (progress_tx, mut progress_rx) = mpsc::channel::<UploadProgress>(16);
        
        // Create channel for responses to client
        let (response_tx, response_rx) = mpsc::channel::<Result<UploadChunkResponse, Status>>(16);

        let trace_id_clone = trace_id.to_string();
        let bucket = metadata.bucket.clone();
        let key = metadata.key.clone();
        let content_type = metadata.content_type.clone();
        let content_length = Some(metadata.content_length.clone());
        let object_repository = self.object_repository.clone();

        // Spawn task to forward chunks from gRPC stream to S3
        tokio::spawn(async move {
            while let Some(msg_result) = stream.next().await {
                match msg_result {
                    Ok(msg) => match msg.data {
                        Some(UploadData::Chunk(chunk)) => {
                            let chunk_bytes = bytes::Bytes::from(chunk);
                            if chunk_tx.send(Ok(chunk_bytes)).await.is_err() {
                                error!("Channel closed during upload");
                                break;
                            }
                        }
                        Some(UploadData::Metadata(_)) => {
                            let _ = chunk_tx
                                .send(Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidInput,
                                    "Unexpected metadata",
                                )))
                                .await;
                            break;
                        }
                        None => continue,
                    },
                    Err(e) => {
                        let _ = chunk_tx
                            .send(Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!("Stream error: {}", e),
                            )))
                            .await;
                        break;
                    }
                }
            }
        });

        // Spawn task to forward progress updates to response stream
        let response_tx_progress = response_tx.clone();
        let trace_id_progress = trace_id.to_string();
        tokio::spawn(async move {
            while let Some(progress) = progress_rx.recv().await {
                let response = match progress {
                    UploadProgress::Initiated { upload_id, bucket, key } => {
                        info!(
                            trace_id = %trace_id_progress,
                            upload_id = %upload_id,
                            bucket = %bucket,
                            key = %key,
                            "Upload initiated, sending upload_id to client"
                        );
                        UploadChunkResponse {
                            trace_id: trace_id_progress.clone(),
                            data: Some(UploadResponseData::Initiated(UploadInitiated {
                                upload_id,
                                bucket,
                                key,
                            })),
                        }
                    }
                    UploadProgress::PartUploaded { part_number, bytes_uploaded, total_bytes } => {
                        UploadChunkResponse {
                            trace_id: trace_id_progress.clone(),
                            data: Some(UploadResponseData::Progress(ProtoUploadProgress {
                                part_number,
                                bytes_uploaded,
                                total_bytes,
                            })),
                        }
                    }
                };
                if response_tx_progress.send(Ok(response)).await.is_err() {
                    break;
                }
            }
        });

        // Spawn task to perform the upload and send final result
        let response_tx_final = response_tx;
        tokio::spawn(async move {
            let result = object_repository
                .upload_multipart(
                    &bucket,
                    &key,
                    &content_type,
                    content_length,
                    chunk_rx,
                    Some(progress_tx),
                )
                .await;

            let response = match result {
                Ok(upload_result) => {
                    info!(
                        trace_id = %trace_id_clone,
                        bucket = %upload_result.bucket,
                        key = %upload_result.key,
                        etag = %upload_result.etag,
                        size = %upload_result.size,
                        "Streaming upload completed successfully"
                    );
                    Ok(UploadChunkResponse {
                        trace_id: trace_id_clone,
                        data: Some(UploadResponseData::Result(UploadResult {
                            bucket: upload_result.bucket,
                            key: upload_result.key,
                            etag: upload_result.etag,
                            size: upload_result.size,
                        })),
                    })
                }
                Err(e) => {
                    error!(trace_id = %trace_id_clone, error = %e, "Upload failed");
                    Err(Status::internal(format!("Upload failed: {}", e)))
                }
            };
            let _ = response_tx_final.send(response).await;
        });

        let stream = ReceiverStream::new(response_rx);
        Ok(Response::new(Box::pin(stream)))
    }

    type DownloadObjectStream = Pin<Box<dyn Stream<Item = Result<DownloadChunkResponse, Status>> + Send>>;

    /// Download object using server streaming
    /// Server sends metadata first, then file chunks
    async fn download_object(
        &self,
        request: Request<DownloadObjectRequest>,
    ) -> Result<Response<Self::DownloadObjectStream>, Status> {
        let req = request.into_inner();
        let trace_id = get_trace_id();

        info!(
            trace_id = %trace_id,
            bucket = %req.bucket,
            key = %req.key,
            "Starting streaming download"
        );

        // Get the object from S3
        let download_result = self
            .object_repository
            .download(&req.bucket, &req.key)
            .await
            .map_err(domain_error_to_status)?;

        let metadata = download_result.metadata;
        let body = download_result.body;

        info!(
            trace_id = %trace_id,
            bucket = %metadata.bucket,
            key = %metadata.key,
            content_length = metadata.content_length,
            content_type = %metadata.content_type,
            "Object found, starting stream"
        );

        // Create a channel for streaming responses
        let (tx, rx) = mpsc::channel::<Result<DownloadChunkResponse, Status>>(16);
        let trace_id_clone = trace_id.to_string();

        // Spawn a task to stream the data
        tokio::spawn(async move {
            // First, send metadata
            let metadata_response = DownloadChunkResponse {
                trace_id: trace_id_clone.clone(),
                data: Some(DownloadData::Metadata(DownloadMetadata {
                    bucket: metadata.bucket.clone(),
                    key: metadata.key.clone(),
                    content_type: metadata.content_type.clone(),
                    content_length: metadata.content_length,
                    etag: metadata.etag.clone(),
                    last_modified: metadata.last_modified.clone(),
                })),
            };

            if tx.send(Ok(metadata_response)).await.is_err() {
                error!("Failed to send metadata response");
                return;
            }

            // Stream the body in chunks using ByteStream's native streaming
            let mut byte_stream = body;
            let mut total_bytes = 0i64;

            while let Some(chunk_result) = byte_stream.next().await {
                match chunk_result {
                    Ok(bytes) => {
                        // ByteStream yields Bytes chunks directly
                        let chunk_data = bytes.to_vec();
                        total_bytes += chunk_data.len() as i64;

                        let chunk_response = DownloadChunkResponse {
                            trace_id: trace_id_clone.clone(),
                            data: Some(DownloadData::Chunk(chunk_data)),
                        };

                        if tx.send(Ok(chunk_response)).await.is_err() {
                            error!(trace_id = %trace_id_clone, "Client disconnected during download");
                            break;
                        }
                    }
                    Err(e) => {
                        error!(trace_id = %trace_id_clone, error = %e, "Error reading from S3");
                        let _ = tx.send(Err(Status::internal(format!("Read error: {}", e)))).await;
                        break;
                    }
                }
            }

            info!(trace_id = %trace_id_clone, total_bytes = total_bytes, "Download stream completed");
        });

        let stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream)))
    }

    // ============ PreSigned URL Operations ============

    async fn get_upload_url(
        &self,
        request: Request<GetUploadUrlRequest>,
    ) -> Result<Response<PreSignedUrlResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("ObjectService", "GetUploadUrl", &req);
        let trace_id = get_trace_id();

        let expires_in = req.expires_in_seconds.unwrap_or(DEFAULT_PRESIGNED_EXPIRATION) as u64;
        let content_type = req.content_type.as_deref();

        let url = self
            .object_repository
            .generate_presigned_upload_url(
                &req.bucket,
                &req.key,
                content_type,
                expires_in,
            )
            .await
            .map_err(domain_error_to_status)?;

        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64 + expires_in as i64;

        let response = PreSignedUrlResponse {
            trace_id: trace_id.to_string(),
            data: Some(PreSignedUrl {
                url,
                method: "PUT".to_string(),
                expires_at,
                expires_in_seconds: expires_in as i32,
            }),
        };

        log.ok(&response);
        Ok(Response::new(response))
    }

    async fn get_download_url(
        &self,
        request: Request<GetDownloadUrlRequest>,
    ) -> Result<Response<PreSignedUrlResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("ObjectService", "GetDownloadUrl", &req);
        let trace_id = get_trace_id();

        let expires_in = req.expires_in_seconds.unwrap_or(DEFAULT_PRESIGNED_EXPIRATION) as u64;

        let url = self
            .object_repository
            .generate_presigned_download_url(
                &req.bucket,
                &req.key,
                expires_in,
            )
            .await
            .map_err(domain_error_to_status)?;

        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64 + expires_in as i64;

        let response = PreSignedUrlResponse {
            trace_id: trace_id.to_string(),
            data: Some(PreSignedUrl {
                url,
                method: "GET".to_string(),
                expires_at,
                expires_in_seconds: expires_in as i32,
            }),
        };

        log.ok(&response);
        Ok(Response::new(response))
    }

    // ============ Command Operations ============

    async fn delete_object(
        &self,
        request: Request<DeleteObjectRequest>,
    ) -> Result<Response<DeleteObjectResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("ObjectService", "DeleteObject", &req);
        let trace_id = get_trace_id();

        let command = DeleteObjectsCommand::new(req.bucket, req.keys)
            .map_err(domain_error_to_status)?;

        let result = self
            .delete_objects_handler
            .handle(command)
            .await
            .map_err(domain_error_to_status)?;

        let response = DeleteObjectResponse {
            trace_id: trace_id.to_string(),
            deleted: result.deleted,
            errors: result
                .errors
                .into_iter()
                .map(|e| DeleteError {
                    key: e.key,
                    code: e.code,
                    message: e.message,
                })
                .collect(),
        };

        log.ok(&response);
        Ok(Response::new(response))
    }

    async fn copy_object(
        &self,
        request: Request<CopyObjectRequest>,
    ) -> Result<Response<CopyObjectResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("ObjectService", "CopyObject", &req);
        let trace_id = get_trace_id();

        let command = CopyObjectCommand::new(
            req.source_bucket,
            req.source_key,
            req.dest_bucket,
            req.dest_key,
        )
        .map_err(domain_error_to_status)?;

        let result = self
            .copy_object_handler
            .handle(command)
            .await
            .map_err(domain_error_to_status)?;

        let response = CopyObjectResponse {
            trace_id: trace_id.to_string(),
            data: Some(CopyResult {
                etag: result.etag,
                last_modified: result.last_modified,
            }),
        };

        log.ok(&response);
        Ok(Response::new(response))
    }

    async fn abort_upload(
        &self,
        request: Request<AbortUploadRequest>,
    ) -> Result<Response<AbortUploadResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("ObjectService", "AbortUpload", &req);
        let trace_id = get_trace_id();

        info!(
            trace_id = %trace_id,
            bucket = %req.bucket,
            key = %req.key,
            upload_id = %req.upload_id,
            "Aborting multipart upload"
        );

        self.object_repository
            .abort_upload(&req.bucket, &req.key, &req.upload_id)
            .await
            .map_err(domain_error_to_status)?;

        let response = AbortUploadResponse {
            trace_id: trace_id.to_string(),
            success: true,
            bucket: req.bucket,
            key: req.key,
            upload_id: req.upload_id,
        };

        log.ok(&response);
        Ok(Response::new(response))
    }
}
