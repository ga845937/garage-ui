//! Object Service Composition
//!
//! 負責組合 ObjectGrpcService 及其所有 handlers

use std::sync::Arc;

use crate::application::commands::object::handlers::{
    CopyObjectHandler, DeleteObjectsHandler,
};
use crate::application::queries::object::handlers::{
    GetObjectMetadataHandler, ListObjectsHandler,
};
use crate::domain::repositories::ObjectRepository;
use crate::infrastructure::config::S3Config;
use crate::infrastructure::garage::repositories::GarageObjectRepository;
use crate::infrastructure::grpc::services::ObjectGrpcService;

/// Object Service 的依賴建構器
pub struct ObjectServiceBuilder {
    s3_config: S3Config,
}

impl ObjectServiceBuilder {
    pub fn new(s3_config: S3Config) -> Self {
        Self { s3_config }
    }

    pub async fn build(self) -> ObjectGrpcService {
        let repository: Arc<dyn ObjectRepository> =
            Arc::new(GarageObjectRepository::new(&self.s3_config).await);

        // Query Handlers
        let list_objects_handler = Arc::new(ListObjectsHandler::new(repository.clone()));
        let get_object_metadata_handler =
            Arc::new(GetObjectMetadataHandler::new(repository.clone()));

        // Command Handlers
        let delete_objects_handler = Arc::new(DeleteObjectsHandler::new(repository.clone()));
        let copy_object_handler = Arc::new(CopyObjectHandler::new(repository.clone()));

        ObjectGrpcService::new(
            list_objects_handler,
            get_object_metadata_handler,
            delete_objects_handler,
            copy_object_handler,
            repository,
        )
    }
}
