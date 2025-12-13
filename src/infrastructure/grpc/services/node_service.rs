//! Node gRPC service implementation

use std::sync::Arc;
use std::collections::HashMap;
use serde::Serialize;
use tonic::{Request, Response, Status};

use crate::application::commands::node::{
    CreateMetadataSnapshotCommand, LaunchRepairCommand,
};
use crate::application::commands::node::handlers::{
    CreateMetadataSnapshotHandler, LaunchRepairHandler,
};
use crate::application::queries::node::{
    GetNodeInfoQuery, GetNodeStatisticsQuery,
};
use crate::application::queries::node::handlers::{
    GetNodeInfoHandler, GetNodeStatisticsHandler,
};
use crate::infrastructure::grpc::conversions::domain_error_to_status;
use crate::grpc_log;
use crate::shared::get_trace_id;

use crate::infrastructure::grpc::generated::node::{
    node_service_server::NodeService,
    ApiResponse, api_response::Data,
    GetNodeInfoRequest, MultiNodeInfoData, NodeInfoResult, node_info_result,
    GetNodeStatisticsRequest, MultiNodeStatisticsData, NodeStatisticsResult, node_statistics_result,
    CreateMetadataSnapshotRequest, LaunchRepairRequest, 
    MultiNodeEmptyData, EmptyResult, empty_result,
    NodeInfo, NodeStatistics,
};

/// gRPC service for node operations
pub struct NodeGrpcService {
    // Command handlers
    create_metadata_snapshot_handler: Arc<CreateMetadataSnapshotHandler>,
    launch_repair_handler: Arc<LaunchRepairHandler>,
    // Query handlers
    get_node_info_handler: Arc<GetNodeInfoHandler>,
    get_node_statistics_handler: Arc<GetNodeStatisticsHandler>,
}

impl NodeGrpcService {
    pub fn new(
        create_metadata_snapshot_handler: Arc<CreateMetadataSnapshotHandler>,
        launch_repair_handler: Arc<LaunchRepairHandler>,
        get_node_info_handler: Arc<GetNodeInfoHandler>,
        get_node_statistics_handler: Arc<GetNodeStatisticsHandler>,
    ) -> Self {
        Self {
            create_metadata_snapshot_handler,
            launch_repair_handler,
            get_node_info_handler,
            get_node_statistics_handler,
        }
    }
}

#[tonic::async_trait]
impl NodeService for NodeGrpcService {
    async fn get_node_info(
        &self,
        request: Request<GetNodeInfoRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("NodeService", "GetNodeInfo", &NodeRequest { node: &req.node });
        let trace_id = get_trace_id();
        
        let response = self
            .get_node_info_handler
            .handle(GetNodeInfoQuery::new(req.node))
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let mut results: HashMap<String, NodeInfoResult> = HashMap::new();
        
        for (node_id, info) in response.success.iter() {
            results.insert(node_id.clone(), NodeInfoResult {
                result: Some(node_info_result::Result::Info(NodeInfo {
                    node_id: info.node_id.clone(),
                    node_addr: info.node_addr.clone(),
                    zone: info.zone.clone(),
                    capacity: info.capacity,
                    tags: info.tags.clone(),
                    garage_version: info.garage_version.clone(),
                    garage_features: info.garage_features.clone().unwrap_or_default(),
                    rust_version: info.rust_version.clone(),
                    db_engine: info.db_engine.clone(),
                })),
            });
        }
        
        for (node_id, error) in response.error.iter() {
            results.insert(node_id.clone(), NodeInfoResult {
                result: Some(node_info_result::Result::Error(error.clone())),
            });
        }

        let api_response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::NodeInfo(MultiNodeInfoData { results: results.clone() })),
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

    async fn get_node_statistics(
        &self,
        request: Request<GetNodeStatisticsRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("NodeService", "GetNodeStatistics", &NodeRequest { node: &req.node });
        let trace_id = get_trace_id();
        
        let response = self
            .get_node_statistics_handler
            .handle(GetNodeStatisticsQuery::new(req.node))
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let mut results: HashMap<String, NodeStatisticsResult> = HashMap::new();
        
        for (node_id, stats) in response.success.iter() {
            results.insert(node_id.clone(), NodeStatisticsResult {
                result: Some(node_statistics_result::Result::Statistics(NodeStatistics {
                    freeform: stats.freeform.clone(),
                })),
            });
        }
        
        for (node_id, error) in response.error.iter() {
            results.insert(node_id.clone(), NodeStatisticsResult {
                result: Some(node_statistics_result::Result::Error(error.clone())),
            });
        }

        let api_response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::NodeStatistics(MultiNodeStatisticsData { results: results.clone() })),
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

    async fn create_metadata_snapshot(
        &self,
        request: Request<CreateMetadataSnapshotRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("NodeService", "CreateMetadataSnapshot", &NodeRequest { node: &req.node });
        let trace_id = get_trace_id();
        
        let response = self
            .create_metadata_snapshot_handler
            .handle(CreateMetadataSnapshotCommand::new(req.node))
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let mut results: HashMap<String, EmptyResult> = HashMap::new();
        
        for (node_id, _) in response.success.iter() {
            results.insert(node_id.clone(), EmptyResult {
                result: Some(empty_result::Result::Success(true)),
            });
        }
        
        for (node_id, error) in response.error.iter() {
            results.insert(node_id.clone(), EmptyResult {
                result: Some(empty_result::Result::Error(error.clone())),
            });
        }

        let api_response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::MetadataSnapshot(MultiNodeEmptyData { results: results.clone() })),
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

    async fn launch_repair(
        &self,
        request: Request<LaunchRepairRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("NodeService", "LaunchRepair", &RepairRequest { node: &req.node, repair_type: &req.repair_type });
        let trace_id = get_trace_id();
        
        let response = self
            .launch_repair_handler
            .handle(LaunchRepairCommand::new(req.node, req.repair_type))
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let mut results: HashMap<String, EmptyResult> = HashMap::new();
        
        for (node_id, _) in response.success.iter() {
            results.insert(node_id.clone(), EmptyResult {
                result: Some(empty_result::Result::Success(true)),
            });
        }
        
        for (node_id, error) in response.error.iter() {
            results.insert(node_id.clone(), EmptyResult {
                result: Some(empty_result::Result::Error(error.clone())),
            });
        }

        let api_response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::LaunchRepair(MultiNodeEmptyData { results: results.clone() })),
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
struct RepairRequest<'a> {
    node: &'a str,
    repair_type: &'a str,
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
