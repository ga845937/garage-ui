//! Block Repository Implementation
//!
//! Infrastructure 層的 Repository 具體實現

use async_trait::async_trait;
use crate::domain::entities::{
    BlockError, BlockInfo, BlockUploadRef, BlockVersionRef, MultiNodeResponse, 
    PurgeBlocksResult, RetryResyncResult,
};
use crate::domain::errors::DomainError;
use crate::domain::repositories::BlockRepository;
use crate::infrastructure::garage::api::{
    BlockErrorResponse, BlockInfoResponse, GetBlockInfoRequest, 
    MultiNodeBlockResponse, PurgeBlocksRequest, PurgeBlocksResultResponse, 
    RetryBlockResyncRequest, RetryResyncResultResponse,
};
use crate::infrastructure::garage::client::GarageClient;
use crate::infrastructure::garage::endpoints::GarageApiEndpoint;

/// Block Repository 實現
pub struct GarageBlockRepository {
    client: GarageClient,
}

impl GarageBlockRepository {
    pub fn new(client: GarageClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl BlockRepository for GarageBlockRepository {
    async fn get_info(&self, node: &str, block_hash: &str) -> Result<MultiNodeResponse<BlockInfo>, DomainError> {
        let path = format!("{}?node={}", GarageApiEndpoint::GetBlockInfo.path(), node);
        let request = GetBlockInfoRequest { block_hash: block_hash.to_string() };
        let response: MultiNodeBlockResponse<BlockInfoResponse> = self.client.post(&path, &request).await?;
        Ok(MultiNodeResponse {
            success: response.success.into_iter().map(|(k, v)| (k, convert_block_info(v))).collect(),
            error: response.error,
        })
    }
    
    async fn list_errors(&self, node: &str) -> Result<MultiNodeResponse<Vec<BlockError>>, DomainError> {
        let path = format!("{}?node={}", GarageApiEndpoint::ListBlockErrors.path(), node);
        let response: MultiNodeBlockResponse<Vec<BlockErrorResponse>> = self.client.get(&path).await?;
        Ok(MultiNodeResponse {
            success: response.success.into_iter().map(|(k, v)| {
                (k, v.into_iter().map(|e| BlockError {
                    block_hash: e.block_hash,
                    error: e.error,
                }).collect())
            }).collect(),
            error: response.error,
        })
    }
    
    async fn purge(&self, node: &str, block_hashes: Vec<String>) -> Result<MultiNodeResponse<PurgeBlocksResult>, DomainError> {
        let path = format!("{}?node={}", GarageApiEndpoint::PurgeBlocks.path(), node);
        let request: PurgeBlocksRequest = block_hashes;
        let response: MultiNodeBlockResponse<PurgeBlocksResultResponse> = self.client.post(&path, &request).await?;
        Ok(MultiNodeResponse {
            success: response.success.into_iter().map(|(k, v)| (k, PurgeBlocksResult {
                blocks_purged: v.blocks_purged,
                objects_deleted: v.objects_deleted,
                uploads_deleted: v.uploads_deleted,
            })).collect(),
            error: response.error,
        })
    }
    
    async fn retry_resync(&self, node: &str, block_hashes: Vec<String>) -> Result<MultiNodeResponse<RetryResyncResult>, DomainError> {
        let path = format!("{}?node={}", GarageApiEndpoint::RetryBlockResync.path(), node);
        let request = RetryBlockResyncRequest::Specific { block_hashes };
        let response: MultiNodeBlockResponse<RetryResyncResultResponse> = self.client.post(&path, &request).await?;
        Ok(MultiNodeResponse {
            success: response.success.into_iter().map(|(k, v)| (k, RetryResyncResult {
                blocks_retried: v.blocks_retried,
            })).collect(),
            error: response.error,
        })
    }
    
    async fn retry_resync_all(&self, node: &str) -> Result<MultiNodeResponse<RetryResyncResult>, DomainError> {
        let path = format!("{}?node={}", GarageApiEndpoint::RetryBlockResync.path(), node);
        let request = RetryBlockResyncRequest::All { all: true };
        let response: MultiNodeBlockResponse<RetryResyncResultResponse> = self.client.post(&path, &request).await?;
        Ok(MultiNodeResponse {
            success: response.success.into_iter().map(|(k, v)| (k, RetryResyncResult {
                blocks_retried: v.blocks_retried,
            })).collect(),
            error: response.error,
        })
    }
}

fn convert_block_info(response: BlockInfoResponse) -> BlockInfo {
    BlockInfo {
        block_hash: response.block_hash,
        size: response.size,
        refcount: response.refcount,
        versions: response.versions.into_iter().map(|v| BlockVersionRef {
            bucket_id: v.bucket_id,
            key: v.key,
            version_uuid: v.version_uuid,
            deleted: v.deleted,
            block_offset: v.block_offset,
        }).collect(),
        uploads: response.uploads.into_iter().map(|u| BlockUploadRef {
            bucket_id: u.bucket_id,
            key: u.key,
            upload_id: u.upload_id,
            part_number: u.part_number,
            block_offset: u.block_offset,
        }).collect(),
    }
}
