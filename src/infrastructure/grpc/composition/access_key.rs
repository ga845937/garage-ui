//! Access Key Service Composition
//!
//! 負責組合 AccessKeyGrpcService 及其所有 handlers

use std::sync::Arc;

use crate::infrastructure::garage::{
    GarageClient, GarageAccessKeyCommandRepository, GarageAccessKeyQueryRepository,
};
use crate::application::commands::access_key::handlers::{
    CreateKeyHandler, UpdateKeyHandler, DeleteKeyHandler,
};
use crate::application::queries::access_key::handlers::{
    ListKeysHandler, ReadKeyHandler,
};
use crate::infrastructure::grpc::services::AccessKeyGrpcService;

/// Access Key Service 的依賴建構器
pub struct AccessKeyServiceBuilder {
    client: GarageClient,
}

impl AccessKeyServiceBuilder {
    pub fn new(client: GarageClient) -> Self {
        Self { client }
    }

    pub fn build(self) -> AccessKeyGrpcService {
        let command_repository = Arc::new(GarageAccessKeyCommandRepository::new(self.client.clone()));
        let query_repository = Arc::new(GarageAccessKeyQueryRepository::new(self.client.clone()));

        // Command Handlers
        let create_key_handler = Arc::new(CreateKeyHandler::new(command_repository.clone()));
        let update_key_handler = Arc::new(UpdateKeyHandler::new(command_repository.clone()));
        let delete_key_handler = Arc::new(DeleteKeyHandler::new(command_repository));
        
        // Query Handlers
        let list_keys_handler = Arc::new(ListKeysHandler::new(query_repository.clone()));
        let read_key_handler = Arc::new(ReadKeyHandler::new(query_repository));

        AccessKeyGrpcService::new(
            create_key_handler,
            update_key_handler,
            delete_key_handler,
            list_keys_handler,
            read_key_handler,
        )
    }
}
