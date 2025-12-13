//! Worker Repository Implementation
//!
//! Infrastructure 層的 Repository 具體實現

use async_trait::async_trait;
use crate::domain::entities::{
    MultiNodeResponse, SetVariableResult, WorkerError, WorkerInfo, WorkerVariables,
};
use crate::domain::errors::DomainError;
use crate::domain::repositories::WorkerRepository;
use crate::infrastructure::garage::api::{
    GetWorkerInfoRequest, GetWorkerVariableRequest, ListWorkersRequest, 
    MultiNodeWorkerResponse, SetVariableResultResponse, SetWorkerVariableRequest, 
    WorkerInfoResponse, WorkerVariablesResponse,
};
use crate::infrastructure::garage::client::GarageClient;
use crate::infrastructure::garage::endpoints::GarageApiEndpoint;

/// Worker Repository 實現
pub struct GarageWorkerRepository {
    client: GarageClient,
}

impl GarageWorkerRepository {
    pub fn new(client: GarageClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl WorkerRepository for GarageWorkerRepository {
    async fn list(&self, node: &str, busy_only: bool, error_only: bool) -> Result<MultiNodeResponse<Vec<WorkerInfo>>, DomainError> {
        let path = format!("{}?node={}", GarageApiEndpoint::ListWorkers.path(), node);
        let request = ListWorkersRequest {
            busy_only: Some(busy_only),
            error_only: Some(error_only),
        };
        let response: MultiNodeWorkerResponse<Vec<WorkerInfoResponse>> = self.client.post(&path, &request).await?;
        Ok(MultiNodeResponse {
            success: response.success.into_iter().map(|(k, v)| {
                (k, v.into_iter().map(|w| convert_worker_info(w)).collect())
            }).collect(),
            error: response.error,
        })
    }
    
    async fn get_info(&self, node: &str, id: i64) -> Result<MultiNodeResponse<WorkerInfo>, DomainError> {
        let path = format!("{}?node={}", GarageApiEndpoint::GetWorkerInfo.path(), node);
        let request = GetWorkerInfoRequest { id };
        let response: MultiNodeWorkerResponse<WorkerInfoResponse> = self.client.post(&path, &request).await?;
        Ok(MultiNodeResponse {
            success: response.success.into_iter().map(|(k, v)| (k, convert_worker_info(v))).collect(),
            error: response.error,
        })
    }
    
    async fn get_variable(&self, node: &str, variable: Option<String>) -> Result<MultiNodeResponse<WorkerVariables>, DomainError> {
        let path = format!("{}?node={}", GarageApiEndpoint::GetWorkerVariable.path(), node);
        let request = GetWorkerVariableRequest { variable };
        let response: MultiNodeWorkerResponse<WorkerVariablesResponse> = self.client.post(&path, &request).await?;
        Ok(MultiNodeResponse {
            success: response.success.into_iter().map(|(k, v)| (k, WorkerVariables {
                variables: v.variables,
            })).collect(),
            error: response.error,
        })
    }
    
    async fn set_variable(&self, node: &str, variable: String, value: String) -> Result<MultiNodeResponse<SetVariableResult>, DomainError> {
        let path = format!("{}?node={}", GarageApiEndpoint::SetWorkerVariable.path(), node);
        let request = SetWorkerVariableRequest { variable, value };
        let response: MultiNodeWorkerResponse<SetVariableResultResponse> = self.client.post(&path, &request).await?;
        Ok(MultiNodeResponse {
            success: response.success.into_iter().map(|(k, v)| (k, SetVariableResult {
                variable: v.variable,
                old_value: v.old_value,
                new_value: v.new_value,
            })).collect(),
            error: response.error,
        })
    }
}

fn convert_worker_info(response: WorkerInfoResponse) -> WorkerInfo {
    WorkerInfo {
        id: response.id,
        name: response.name,
        state: response.state,
        progress: response.progress,
        errors: response.errors,
        consecutive_errors: response.consecutive_errors,
        last_error: response.last_error.map(|e| WorkerError {
            message: e.message,
            secs_ago: e.secs_ago,
        }),
        tranquility: response.tranquility,
        freeform: response.freeform,
    }
}
