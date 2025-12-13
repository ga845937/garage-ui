//! Block Service Composition
//!
//! 負責組合 BlockGrpcService 及其所有 handlers

use std::sync::Arc;

use crate::infrastructure::garage::{GarageClient, GarageBlockRepository};
use crate::application::commands::block::handlers::{
    PurgeBlocksHandler, RetryBlockResyncHandler,
};
use crate::application::queries::block::handlers::{
    GetBlockInfoHandler, ListBlockErrorsHandler,
};
use crate::infrastructure::grpc::services::BlockGrpcService;

/// Block Service 的依賴建構器
pub struct BlockServiceBuilder {
    client: GarageClient,
}

impl BlockServiceBuilder {
    pub fn new(client: GarageClient) -> Self {
        Self { client }
    }

    pub fn build(self) -> BlockGrpcService {
        let repository = Arc::new(GarageBlockRepository::new(self.client));

        // Command Handlers
        let purge_blocks_handler = Arc::new(PurgeBlocksHandler::new(repository.clone()));
        let retry_block_resync_handler = Arc::new(RetryBlockResyncHandler::new(repository.clone()));

        // Query Handlers
        let get_block_info_handler = Arc::new(GetBlockInfoHandler::new(repository.clone()));
        let list_block_errors_handler = Arc::new(ListBlockErrorsHandler::new(repository));

        BlockGrpcService::new(
            purge_blocks_handler,
            retry_block_resync_handler,
            get_block_info_handler,
            list_block_errors_handler,
        )
    }
}
