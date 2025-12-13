//! Access Key gRPC service implementation

use std::sync::Arc;
use serde::Serialize;
use tonic::{Request, Response, Status};

use crate::application::commands::access_key::{
    CreateKeyCommand, UpdateKeyCommand, DeleteKeyCommand
};
use crate::application::commands::access_key::handlers::{
    CreateKeyHandler, UpdateKeyHandler, DeleteKeyHandler,
};
use crate::application::queries::access_key::{
    ListKeysQuery, ReadKeyQuery,
};
use crate::application::queries::access_key::handlers::{
    ListKeysHandler, ReadKeyHandler,
};
use crate::grpc_log;
use crate::shared::get_trace_id;
use crate::infrastructure::grpc::conversions::{NullableStringExt, domain_error_to_status};

use crate::infrastructure::grpc::generated::access_key::{
    access_key_service_server::AccessKeyService,
    ListKeysRequest, ListKeysResponse, KeyListItem,
    KeyResponse, Key, ReadKeyRequest, DeleteKeyResponse,
    CreateKeyRequest, UpdateKeyRequest, DeleteKeyRequest,
    KeyPermissions, KeyBucket, KeyBucketPermissions,
};

/// gRPC service for access key operations
pub struct AccessKeyGrpcService {
    create_key_handler: Arc<CreateKeyHandler>,
    update_key_handler: Arc<UpdateKeyHandler>,
    delete_key_handler: Arc<DeleteKeyHandler>,
    list_keys_handler: Arc<ListKeysHandler>,
    read_key_handler: Arc<ReadKeyHandler>,
}

impl AccessKeyGrpcService {
    pub fn new(
        create_key_handler: Arc<CreateKeyHandler>,
        update_key_handler: Arc<UpdateKeyHandler>,
        delete_key_handler: Arc<DeleteKeyHandler>,
        list_keys_handler: Arc<ListKeysHandler>,
        read_key_handler: Arc<ReadKeyHandler>,
    ) -> Self {
        Self {
            create_key_handler,
            update_key_handler,
            delete_key_handler,
            list_keys_handler,
            read_key_handler,
        }
    }
}

#[tonic::async_trait]
impl AccessKeyService for AccessKeyGrpcService {
    async fn list_key(
        &self,
        request: Request<ListKeysRequest>,
    ) -> Result<Response<ListKeysResponse>, Status> {
        let req = request.into_inner();
        let pagination = req.pagination.clone().unwrap_or_default();
        
        let query = ListKeysQuery::from_grpc_request(
            pagination.page,
            pagination.page_size,
            req.name.clone(),
            req.created.as_ref().map(|c| c.start_date.clone()),
            req.created.as_ref().map(|c| c.end_date.clone()),
            req.expiration.as_ref().map(|e| e.start_date.clone()),
            req.expiration.as_ref().map(|e| e.end_date.clone()),
        );

        let log = grpc_log!("AccessKeyService", "ListKey", &ListRequest { 
            name: &req.name, 
            page: &query.page, 
            page_size: &query.page_size,
            created_start: &req.created.as_ref().map(|c| c.start_date.clone()),
            created_end: &req.created.as_ref().map(|c| c.end_date.clone()),
            expiration_start: &req.expiration.as_ref().map(|e| e.start_date.clone()),
            expiration_end: &req.expiration.as_ref().map(|e| e.end_date.clone()),
        });

        let trace_id = get_trace_id();
        
        let data = self
            .list_keys_handler
            .handle(query)
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let key_list: Vec<KeyListItem> = data.0.into_iter()
            .map(|k| KeyListItem {
                id: k.id.clone(),
                name: k.name,
                created: k.created.to_rfc3339(),
                expiration: k.expiration.map(|dt| dt.to_rfc3339()),
                secret_access_key: k.secret_access_key,
            })
            .collect();

        let response = ListKeysResponse {
            trace_id: trace_id.clone(),
            data: key_list.clone(),
            total: data.1 as i32
        };

        log.ok(&ApiResponseLog {
            trace_id: &trace_id,
            data: ListKeysResponseLog {
                data: key_list,
                total: data.1,
            },
        });
        Ok(Response::new(response))
    }

    async fn read_key(
        &self,
        request: Request<ReadKeyRequest>,
    ) -> Result<Response<KeyResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("AccessKeyService", "ReadKey", &SingleIdRequest { id: &req.id });
        let trace_id = get_trace_id();

        let key = self
            .read_key_handler
            .handle(ReadKeyQuery {
                id: req.id,
            })
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let key_info = convert_key_info(key);
        let response = KeyResponse {
            trace_id: trace_id.clone(),
            data: Some(key_info.clone()),
        };

        log.ok(&ApiResponseLog {
            trace_id: &trace_id,
            data: KeyLogSimple {
                id: &key_info.id,
                name: &key_info.name,
            },
        });
        Ok(Response::new(response))
    }

    async fn create_key(
        &self,
        request: Request<CreateKeyRequest>,
    ) -> Result<Response<KeyResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("AccessKeyService", "CreateKey", &CreateKeyReq { name: &Some(req.name.clone()) });
        let trace_id = get_trace_id();
        
        let key = self
            .create_key_handler
            .handle(CreateKeyCommand::new(
                req.name,
                req.expiration,
                req.allow_create_bucket,
            ))
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let key_info = convert_key_info(key);
        let response = KeyResponse {
            trace_id: trace_id.clone(),
            data: Some(key_info.clone()),
        };

        log.ok(&ApiResponseLog {
            trace_id: &trace_id,
            data: KeyLogSimple {
                id: &key_info.id,
                name: &key_info.name,
            },
        });
        Ok(Response::new(response))
    }

    async fn update_key(
        &self,
        request: Request<UpdateKeyRequest>,
    ) -> Result<Response<KeyResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("AccessKeyService", "UpdateKey", &SingleIdRequest { id: &req.id });
        let trace_id = get_trace_id();

        let key = self
            .update_key_handler
            .handle(UpdateKeyCommand::from_grpc_request(
                req.id,
                req.name,
                req.expiration.into_update_field(),
                req.allow_create_bucket,
            ))
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let key_info = convert_key_info(key);
        let response = KeyResponse {
            trace_id: trace_id.clone(),
            data: Some(key_info.clone()),
        };

        log.ok(&ApiResponseLog {
            trace_id: &trace_id,
            data: KeyLogSimple {
                id: &key_info.id,
                name: &key_info.name,
            },
        });
        Ok(Response::new(response))
    }

    async fn delete_key(
        &self,
        request: Request<DeleteKeyRequest>,
    ) -> Result<Response<DeleteKeyResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("AccessKeyService", "DeleteKey", &IdsRequest { id: &req.id });
        let trace_id = get_trace_id();
        let req_id = req.id.clone();

        self.delete_key_handler
            .handle(DeleteKeyCommand::new(req.id))
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let response = DeleteKeyResponse {
            trace_id: trace_id.clone(),
            id: req_id.clone(),
        };

        log.ok(&ApiResponseLog {
            trace_id: &trace_id,
            data: req_id.clone(),
        });
        Ok(Response::new(response))
    }
}

// ============ Log Structs ============

#[derive(Serialize)]
struct ListRequest<'a> {
    name: &'a Option<String>,
    page: &'a i32,
    page_size: &'a i32,
    created_start: &'a Option<String>,
    created_end: &'a Option<String>,
    expiration_start: &'a Option<String>,
    expiration_end: &'a Option<String>,
}

#[derive(Serialize)]
struct SingleIdRequest<'a> {
    id: &'a str,
}

#[derive(Serialize)]
struct IdsRequest<'a> {
    id: &'a Vec<String>,
}

#[derive(Serialize)]
struct CreateKeyReq<'a> {
    name: &'a Option<String>,
}

#[derive(Serialize)]
struct ApiResponseLog<'a, T: Serialize> {
    trace_id: &'a str,
    data: T,
}

#[derive(Serialize)]
struct ListKeysResponseLog {
    data: Vec<KeyListItem>,
    total: usize,
}

#[derive(Serialize)]
struct KeyLogSimple<'a> {
    id: &'a str,
    name: &'a str,
}

// ============ Helpers ============

fn convert_key_info(key: crate::domain::entities::AccessKey) -> Key {
    Key {
        id: key.id,
        secret_access_key: key.secret_access_key,
        name: key.name,
        permissions: Some(KeyPermissions {
            create_bucket: key.permissions.create_bucket,
        }),
        expiration: key.expiration.map(|dt| dt.to_rfc3339()),
        created: key.created.to_rfc3339(),
        buckets: key
            .buckets
            .into_iter()
            .map(|b| KeyBucket {
                id: b.id,
                global_aliases: b.global_aliases,
                local_aliases: b.local_aliases,
                permissions: Some(KeyBucketPermissions {
                    read: b.permissions.read,
                    write: b.permissions.write,
                    owner: b.permissions.owner,
                }),
            })
            .collect(),
    }
}
