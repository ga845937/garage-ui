//! Block gRPC service implementation

use std::sync::Arc;
use std::collections::HashMap;
use serde::Serialize;
use tonic::{Request, Response, Status};

use crate::application::commands::block::{
    PurgeBlocksCommand, RetryBlockResyncCommand,
};
use crate::application::commands::block::handlers::{
    PurgeBlocksHandler, RetryBlockResyncHandler,
};
use crate::application::queries::block::{
    GetBlockInfoQuery, ListBlockErrorsQuery,
};
use crate::application::queries::block::handlers::{
    GetBlockInfoHandler, ListBlockErrorsHandler,
};
use crate::infrastructure::grpc::conversions::domain_error_to_status;
use crate::grpc_log;
use crate::shared::get_trace_id;

use crate::infrastructure::grpc::generated::block::{
    block_service_server::BlockService,
    ApiResponse, api_response::Data,
    GetBlockInfoRequest, MultiNodeBlockInfoData, BlockInfoResult, block_info_result,
    ListBlockErrorsRequest, MultiNodeBlockErrorsData, BlockErrorsResult, block_errors_result, BlockErrors,
    PurgeBlocksRequest, MultiNodePurgeResultData, PurgeResult, purge_result,
    RetryBlockResyncRequest, MultiNodeResyncResultData, ResyncResult, resync_result,
    BlockInfo, BlockVersionRef, BlockUploadRef, BlockError, 
    PurgeBlocksResult, RetryResyncResult,
};

/// gRPC service for block operations
pub struct BlockGrpcService {
    // Command handlers
    purge_blocks_handler: Arc<PurgeBlocksHandler>,
    retry_block_resync_handler: Arc<RetryBlockResyncHandler>,
    // Query handlers
    get_block_info_handler: Arc<GetBlockInfoHandler>,
    list_block_errors_handler: Arc<ListBlockErrorsHandler>,
}

impl BlockGrpcService {
    pub fn new(
        purge_blocks_handler: Arc<PurgeBlocksHandler>,
        retry_block_resync_handler: Arc<RetryBlockResyncHandler>,
        get_block_info_handler: Arc<GetBlockInfoHandler>,
        list_block_errors_handler: Arc<ListBlockErrorsHandler>,
    ) -> Self {
        Self {
            purge_blocks_handler,
            retry_block_resync_handler,
            get_block_info_handler,
            list_block_errors_handler,
        }
    }
}

#[tonic::async_trait]
impl BlockService for BlockGrpcService {
    async fn get_block_info(
        &self,
        request: Request<GetBlockInfoRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("BlockService", "GetBlockInfo", &BlockRequest { node: &req.node, block_hash: &req.block_hash });
        let trace_id = get_trace_id();
        
        let response = self
            .get_block_info_handler
            .handle(GetBlockInfoQuery::new(req.node, req.block_hash))
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let mut results: HashMap<String, BlockInfoResult> = HashMap::new();
        
        for (node_id, info) in response.success.iter() {
            results.insert(node_id.clone(), BlockInfoResult {
                result: Some(block_info_result::Result::Info(BlockInfo {
                    block_hash: info.block_hash.clone(),
                    size: info.size,
                    refcount: info.refcount,
                    versions: info.versions.iter().map(|v| BlockVersionRef {
                        bucket_id: v.bucket_id.clone(),
                        key: v.key.clone(),
                        version_uuid: v.version_uuid.clone(),
                        deleted: v.deleted,
                        block_offset: v.block_offset,
                    }).collect(),
                    uploads: info.uploads.iter().map(|u| BlockUploadRef {
                        bucket_id: u.bucket_id.clone(),
                        key: u.key.clone(),
                        upload_id: u.upload_id.clone(),
                        part_number: u.part_number,
                        block_offset: u.block_offset,
                    }).collect(),
                })),
            });
        }
        
        for (node_id, error) in response.error.iter() {
            results.insert(node_id.clone(), BlockInfoResult {
                result: Some(block_info_result::Result::Error(error.clone())),
            });
        }

        let api_response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::BlockInfo(MultiNodeBlockInfoData { results: results.clone() })),
        };

        log.ok(&ApiResponseLog {
            trace_id: &trace_id,
            data: MultiNodeResultLog {
                success_count: response.success.len(),
                error_count: response.error.len(),
            },
        });
        Ok(Response::new(api_response))
    }

    async fn list_block_errors(
        &self,
        request: Request<ListBlockErrorsRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("BlockService", "ListBlockErrors", &NodeRequest { node: &req.node });
        let trace_id = get_trace_id();
        
        let response = self
            .list_block_errors_handler
            .handle(ListBlockErrorsQuery::new(req.node))
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let mut results: HashMap<String, BlockErrorsResult> = HashMap::new();
        
        for (node_id, errors) in response.success.iter() {
            results.insert(node_id.clone(), BlockErrorsResult {
                result: Some(block_errors_result::Result::Errors(BlockErrors {
                    errors: errors.iter().map(|e| BlockError {
                        block_hash: e.block_hash.clone(),
                        error: e.error.clone(),
                    }).collect(),
                })),
            });
        }
        
        for (node_id, error) in response.error.iter() {
            results.insert(node_id.clone(), BlockErrorsResult {
                result: Some(block_errors_result::Result::Error(error.clone())),
            });
        }

        let api_response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::BlockErrors(MultiNodeBlockErrorsData { results: results.clone() })),
        };

        log.ok(&ApiResponseLog {
            trace_id: &trace_id,
            data: MultiNodeResultLog {
                success_count: response.success.len(),
                error_count: response.error.len(),
            },
        });
        Ok(Response::new(api_response))
    }

    async fn purge_blocks(
        &self,
        request: Request<PurgeBlocksRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("BlockService", "PurgeBlocks", &PurgeRequest { node: &req.node, block_count: req.block_hashes.len() });
        let trace_id = get_trace_id();
        
        let response = self
            .purge_blocks_handler
            .handle(PurgeBlocksCommand::new(req.node, req.block_hashes))
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let mut results: HashMap<String, PurgeResult> = HashMap::new();
        
        for (node_id, result) in response.success.iter() {
            results.insert(node_id.clone(), PurgeResult {
                result: Some(purge_result::Result::PurgeResult(PurgeBlocksResult {
                    blocks_purged: result.blocks_purged,
                    objects_deleted: result.objects_deleted,
                    uploads_deleted: result.uploads_deleted,
                })),
            });
        }
        
        for (node_id, error) in response.error.iter() {
            results.insert(node_id.clone(), PurgeResult {
                result: Some(purge_result::Result::Error(error.clone())),
            });
        }

        let api_response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::PurgeResult(MultiNodePurgeResultData { results: results.clone() })),
        };

        log.ok(&ApiResponseLog {
            trace_id: &trace_id,
            data: MultiNodeResultLog {
                success_count: response.success.len(),
                error_count: response.error.len(),
            },
        });
        Ok(Response::new(api_response))
    }

    async fn retry_block_resync(
        &self,
        request: Request<RetryBlockResyncRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("BlockService", "RetryBlockResync", &ResyncRequest { node: &req.node, all: req.all });
        let trace_id = get_trace_id();
        
        let command = if req.all || req.block_hashes.is_empty() {
            RetryBlockResyncCommand::all(req.node)
        } else {
            RetryBlockResyncCommand::new(req.node, req.block_hashes)
        };
        
        let response = self
            .retry_block_resync_handler
            .handle(command)
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let mut results: HashMap<String, ResyncResult> = HashMap::new();
        
        for (node_id, result) in response.success.iter() {
            results.insert(node_id.clone(), ResyncResult {
                result: Some(resync_result::Result::ResyncResult(RetryResyncResult {
                    blocks_retried: result.blocks_retried,
                })),
            });
        }
        
        for (node_id, error) in response.error.iter() {
            results.insert(node_id.clone(), ResyncResult {
                result: Some(resync_result::Result::Error(error.clone())),
            });
        }

        let api_response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::ResyncResult(MultiNodeResyncResultData { results: results.clone() })),
        };

        log.ok(&ApiResponseLog {
            trace_id: &trace_id,
            data: MultiNodeResultLog {
                success_count: response.success.len(),
                error_count: response.error.len(),
            },
        });
        Ok(Response::new(api_response))
    }
}

// ============ Log Structs ============

#[derive(Serialize)]
struct NodeRequest<'a> {
    node: &'a str,
}

#[derive(Serialize)]
struct BlockRequest<'a> {
    node: &'a str,
    block_hash: &'a str,
}

#[derive(Serialize)]
struct PurgeRequest<'a> {
    node: &'a str,
    block_count: usize,
}

#[derive(Serialize)]
struct ResyncRequest<'a> {
    node: &'a str,
    all: bool,
}

#[derive(Serialize)]
struct ApiResponseLog<'a, T: Serialize> {
    trace_id: &'a str,
    data: T,
}

#[derive(Serialize)]
struct MultiNodeResultLog {
    success_count: usize,
    error_count: usize,
}
