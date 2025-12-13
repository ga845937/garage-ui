//! Bucket Service Composition
//!
//! 負責組合 BucketGrpcService 及其所有 handlers

use std::sync::Arc;

use crate::domain::events::EventBus;
use crate::infrastructure::garage::{GarageClient, GarageBucketRepository};
use crate::application::commands::bucket::handlers::{
    CreateBucketHandler, UpdateBucketHandler, DeleteBucketHandler,
    AddBucketAliasHandler, RemoveBucketAliasHandler,
    BatchAllowBucketKeyHandler, BatchDenyBucketKeyHandler,
};
use crate::application::queries::bucket::handlers::{
    ListBucketsHandler, GetBucketHandler,
};
use crate::infrastructure::grpc::services::BucketGrpcService;

/// Bucket Service 的依賴建構器
pub struct BucketServiceBuilder {
    client: GarageClient,
    event_bus: Arc<dyn EventBus>,
}

impl BucketServiceBuilder {
    pub fn new(
        client: GarageClient,
        event_bus: Arc<dyn EventBus>,
    ) -> Self {
        Self { client, event_bus }
    }

    pub fn build(self) -> BucketGrpcService {
        let repository = Arc::new(GarageBucketRepository::new(self.client));

        // Command Handlers
        let create_bucket_handler = Arc::new(CreateBucketHandler::new(
            repository.clone(),
            self.event_bus.clone(),
        ));
        let update_bucket_handler = Arc::new(UpdateBucketHandler::new(
            repository.clone(),
            self.event_bus.clone(),
        ));
        let delete_bucket_handler = Arc::new(DeleteBucketHandler::new(
            repository.clone(),
            self.event_bus.clone(),
        ));
        let add_bucket_alias_handler = Arc::new(AddBucketAliasHandler::new(
            repository.clone(),
            self.event_bus.clone(),
        ));
        let remove_bucket_alias_handler = Arc::new(RemoveBucketAliasHandler::new(
            repository.clone(),
            self.event_bus.clone(),
        ));
        let allow_bucket_key_handler = Arc::new(BatchAllowBucketKeyHandler::new(
            repository.clone(),
            self.event_bus.clone(),
        ));
        let deny_bucket_key_handler = Arc::new(BatchDenyBucketKeyHandler::new(
            repository.clone(),
            self.event_bus,
        ));

        // Query Handlers
        let list_buckets_handler = Arc::new(ListBucketsHandler::new(repository.clone()));
        let get_bucket_handler = Arc::new(GetBucketHandler::new(repository));

        BucketGrpcService::new(
            create_bucket_handler,
            update_bucket_handler,
            delete_bucket_handler,
            add_bucket_alias_handler,
            remove_bucket_alias_handler,
            allow_bucket_key_handler,
            deny_bucket_key_handler,
            list_buckets_handler,
            get_bucket_handler,
        )
    }
}
