//! Worker Service Composition
//!
//! 負責組合 WorkerGrpcService 及其所有 handlers

use std::sync::Arc;

use crate::infrastructure::garage::{GarageClient, GarageWorkerRepository};
use crate::application::commands::worker::handlers::SetWorkerVariableHandler;
use crate::application::queries::worker::handlers::{
    ListWorkersHandler, GetWorkerInfoHandler, GetWorkerVariableHandler,
};
use crate::infrastructure::grpc::services::WorkerGrpcService;

/// Worker Service 的依賴建構器
pub struct WorkerServiceBuilder {
    client: GarageClient,
}

impl WorkerServiceBuilder {
    pub fn new(client: GarageClient) -> Self {
        Self { client }
    }

    pub fn build(self) -> WorkerGrpcService {
        let repository = Arc::new(GarageWorkerRepository::new(self.client));

        // Command Handler
        let set_worker_variable_handler = Arc::new(SetWorkerVariableHandler::new(repository.clone()));

        // Query Handlers
        let list_workers_handler = Arc::new(ListWorkersHandler::new(repository.clone()));
        let get_worker_info_handler = Arc::new(GetWorkerInfoHandler::new(repository.clone()));
        let get_worker_variable_handler = Arc::new(GetWorkerVariableHandler::new(repository));

        WorkerGrpcService::new(
            set_worker_variable_handler,
            list_workers_handler,
            get_worker_info_handler,
            get_worker_variable_handler,
        )
    }
}
