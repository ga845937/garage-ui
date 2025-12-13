//! Worker gRPC service implementation

use std::sync::Arc;
use std::collections::HashMap;
use serde::Serialize;
use tonic::{Request, Response, Status};

use crate::application::commands::worker::SetWorkerVariableCommand;
use crate::application::commands::worker::handlers::SetWorkerVariableHandler;
use crate::application::queries::worker::{
    ListWorkersQuery, GetWorkerInfoQuery, GetWorkerVariableQuery,
};
use crate::application::queries::worker::handlers::{
    ListWorkersHandler, GetWorkerInfoHandler, GetWorkerVariableHandler,
};
use crate::infrastructure::grpc::conversions::domain_error_to_status;
use crate::grpc_log;
use crate::shared::get_trace_id;

use crate::infrastructure::grpc::generated::worker::{
    worker_service_server::WorkerService,
    ApiResponse, api_response::Data,
    ListWorkersRequest, MultiNodeWorkersData, WorkersResult, workers_result, WorkersList,
    GetWorkerInfoRequest, MultiNodeWorkerInfoData, WorkerInfoResult, worker_info_result,
    GetWorkerVariableRequest, MultiNodeVariablesData, VariablesResult, variables_result,
    SetWorkerVariableRequest, MultiNodeSetVariableData, SetVariableResult, set_variable_result,
    WorkerInfo, WorkerError, WorkerVariables, VariableChangeInfo,
};

/// gRPC service for worker operations
pub struct WorkerGrpcService {
    // Command handlers
    set_worker_variable_handler: Arc<SetWorkerVariableHandler>,
    // Query handlers
    list_workers_handler: Arc<ListWorkersHandler>,
    get_worker_info_handler: Arc<GetWorkerInfoHandler>,
    get_worker_variable_handler: Arc<GetWorkerVariableHandler>,
}

impl WorkerGrpcService {
    pub fn new(
        set_worker_variable_handler: Arc<SetWorkerVariableHandler>,
        list_workers_handler: Arc<ListWorkersHandler>,
        get_worker_info_handler: Arc<GetWorkerInfoHandler>,
        get_worker_variable_handler: Arc<GetWorkerVariableHandler>,
    ) -> Self {
        Self {
            set_worker_variable_handler,
            list_workers_handler,
            get_worker_info_handler,
            get_worker_variable_handler,
        }
    }
}

#[tonic::async_trait]
impl WorkerService for WorkerGrpcService {
    async fn list_workers(
        &self,
        request: Request<ListWorkersRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("WorkerService", "ListWorkers", &ListWorkersReq { node: &req.node, busy_only: req.busy_only, error_only: req.error_only });
        let trace_id = get_trace_id();
        
        let mut query = ListWorkersQuery::new(req.node);
        if req.busy_only {
            query = query.busy_only();
        }
        if req.error_only {
            query = query.error_only();
        }
        
        let response = self
            .list_workers_handler
            .handle(query)
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let mut results: HashMap<String, WorkersResult> = HashMap::new();
        
        for (node_id, workers) in response.success.iter() {
            results.insert(node_id.clone(), WorkersResult {
                result: Some(workers_result::Result::Workers(WorkersList {
                    workers: workers.iter().map(|w| convert_worker_info(w.clone())).collect(),
                })),
            });
        }
        
        for (node_id, error) in response.error.iter() {
            results.insert(node_id.clone(), WorkersResult {
                result: Some(workers_result::Result::Error(error.clone())),
            });
        }

        let api_response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::ListWorkers(MultiNodeWorkersData { results: results.clone() })),
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

    async fn get_worker_info(
        &self,
        request: Request<GetWorkerInfoRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("WorkerService", "GetWorkerInfo", &WorkerInfoReq { node: &req.node, worker_id: req.worker_id });
        let trace_id = get_trace_id();
        
        let response = self
            .get_worker_info_handler
            .handle(GetWorkerInfoQuery::new(req.node, req.worker_id))
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let mut results: HashMap<String, WorkerInfoResult> = HashMap::new();
        
        for (node_id, worker) in response.success.iter() {
            results.insert(node_id.clone(), WorkerInfoResult {
                result: Some(worker_info_result::Result::Info(convert_worker_info(worker.clone()))),
            });
        }
        
        for (node_id, error) in response.error.iter() {
            results.insert(node_id.clone(), WorkerInfoResult {
                result: Some(worker_info_result::Result::Error(error.clone())),
            });
        }

        let api_response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::WorkerInfo(MultiNodeWorkerInfoData { results: results.clone() })),
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

    async fn get_worker_variable(
        &self,
        request: Request<GetWorkerVariableRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("WorkerService", "GetWorkerVariable", &VariableReq { node: &req.node, variable: &req.variable });
        let trace_id = get_trace_id();
        
        let response = self
            .get_worker_variable_handler
            .handle(GetWorkerVariableQuery::new(req.node, req.variable))
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let mut results: HashMap<String, VariablesResult> = HashMap::new();
        
        for (node_id, vars) in response.success.iter() {
            results.insert(node_id.clone(), VariablesResult {
                result: Some(variables_result::Result::Variables(WorkerVariables {
                    variables: vars.variables.clone(),
                })),
            });
        }
        
        for (node_id, error) in response.error.iter() {
            results.insert(node_id.clone(), VariablesResult {
                result: Some(variables_result::Result::Error(error.clone())),
            });
        }

        let api_response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::WorkerVariables(MultiNodeVariablesData { results: results.clone() })),
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

    async fn set_worker_variable(
        &self,
        request: Request<SetWorkerVariableRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("WorkerService", "SetWorkerVariable", &SetVariableReq { node: &req.node, variable: &req.variable, value: &req.value });
        let trace_id = get_trace_id();
        
        let response = self
            .set_worker_variable_handler
            .handle(SetWorkerVariableCommand::new(req.node, req.variable, req.value))
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let mut results: HashMap<String, SetVariableResult> = HashMap::new();
        
        for (node_id, result) in response.success.iter() {
            results.insert(node_id.clone(), SetVariableResult {
                result: Some(set_variable_result::Result::Change(VariableChangeInfo {
                    variable: result.variable.clone(),
                    old_value: result.old_value.clone().unwrap_or_default(),
                    new_value: result.new_value.clone(),
                })),
            });
        }
        
        for (node_id, error) in response.error.iter() {
            results.insert(node_id.clone(), SetVariableResult {
                result: Some(set_variable_result::Result::Error(error.clone())),
            });
        }

        let api_response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::SetVariable(MultiNodeSetVariableData { results: results.clone() })),
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
struct ListWorkersReq<'a> {
    node: &'a str,
    busy_only: bool,
    error_only: bool,
}

#[derive(Serialize)]
struct WorkerInfoReq<'a> {
    node: &'a str,
    worker_id: i64,
}

#[derive(Serialize)]
struct VariableReq<'a> {
    node: &'a str,
    variable: &'a Option<String>,
}

#[derive(Serialize)]
struct SetVariableReq<'a> {
    node: &'a str,
    variable: &'a str,
    value: &'a str,
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

// ============ Helpers ============

fn convert_worker_info(info: crate::domain::entities::WorkerInfo) -> WorkerInfo {
    WorkerInfo {
        id: info.id,
        name: info.name,
        state: info.state,
        progress: info.progress,
        errors: info.errors,
        consecutive_errors: info.consecutive_errors,
        last_error: info.last_error.map(|e| WorkerError {
            message: e.message,
            secs_ago: e.secs_ago,
        }),
        tranquility: info.tranquility,
        freeform: info.freeform,
    }
}
