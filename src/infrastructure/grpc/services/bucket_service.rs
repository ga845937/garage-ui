//! Bucket gRPC service implementation

use std::sync::Arc;
use serde::Serialize;
use tonic::{Request, Response, Status};

use crate::application::commands::bucket::{
    CreateBucketCommand, CreateLocalAliasCommand, DeleteBucketCommand,
    UpdateBucketCommand, AddBucketAliasCommand, RemoveBucketAliasCommand,
    BucketKeyPermissionInput,
    BatchAllowBucketKeyCommand, BatchDenyBucketKeyCommand, BucketKeyPermissionItem,
};
use crate::application::commands::bucket::handlers::{
    CreateBucketHandler, UpdateBucketHandler, DeleteBucketHandler,
    AddBucketAliasHandler, RemoveBucketAliasHandler,
    BatchAllowBucketKeyHandler, BatchDenyBucketKeyHandler,
};
use crate::application::queries::bucket::{
    GetBucketQuery, ListBucketsQuery,
};
use crate::application::queries::bucket::handlers::{
    ListBucketsHandler, GetBucketHandler,
};
use crate::domain::entities::WebsiteConfig;
use crate::domain::value_objects::Quotas;
use crate::grpc_log;
use crate::shared::get_trace_id;
use crate::infrastructure::grpc::conversions::{NullableBoolExt, domain_error_to_status};

use crate::infrastructure::grpc::generated::bucket::{
    bucket_service_server::BucketService,
    // Responses
    ListBucketsResponse, BucketResponse, DeleteBucketResponse,
    BucketKeyPermissionResponse, BucketKeyPermissionResult as GrpcPermissionResult,
    BucketAliasResponse, BucketAliasResult as GrpcAliasResult,
    // Messages
    Bucket, BucketListItem, BucketKey, BucketKeyPermissions, LocalAlias,
    // Requests
    ListBucketsRequest, ReadBucketRequest,
    CreateBucketRequest, UpdateBucketRequest, DeleteBucketRequest,
    AddBucketAliasRequest, RemoveBucketAliasRequest,
    BucketKeyPermissionRequest,
};

/// gRPC service for bucket operations
pub struct BucketGrpcService {
    create_bucket_handler: Arc<CreateBucketHandler>,
    update_bucket_handler: Arc<UpdateBucketHandler>,
    delete_bucket_handler: Arc<DeleteBucketHandler>,
    add_bucket_alias_handler: Arc<AddBucketAliasHandler>,
    remove_bucket_alias_handler: Arc<RemoveBucketAliasHandler>,
    allow_bucket_key_handler: Arc<BatchAllowBucketKeyHandler>,
    deny_bucket_key_handler: Arc<BatchDenyBucketKeyHandler>,
    list_buckets_handler: Arc<ListBucketsHandler>,
    get_bucket_handler: Arc<GetBucketHandler>,
}

impl BucketGrpcService {
    pub fn new(
        create_bucket_handler: Arc<CreateBucketHandler>,
        update_bucket_handler: Arc<UpdateBucketHandler>,
        delete_bucket_handler: Arc<DeleteBucketHandler>,
        add_bucket_alias_handler: Arc<AddBucketAliasHandler>,
        remove_bucket_alias_handler: Arc<RemoveBucketAliasHandler>,
        allow_bucket_key_handler: Arc<BatchAllowBucketKeyHandler>,
        deny_bucket_key_handler: Arc<BatchDenyBucketKeyHandler>,
        list_buckets_handler: Arc<ListBucketsHandler>,
        get_bucket_handler: Arc<GetBucketHandler>,
    ) -> Self {
        Self {
            create_bucket_handler,
            update_bucket_handler,
            delete_bucket_handler,
            add_bucket_alias_handler,
            remove_bucket_alias_handler,
            allow_bucket_key_handler,
            deny_bucket_key_handler,
            list_buckets_handler,
            get_bucket_handler,
        }
    }
}

#[tonic::async_trait]
impl BucketService for BucketGrpcService {
    async fn list_bucket(
        &self,
        request: Request<ListBucketsRequest>,
    ) -> Result<Response<ListBucketsResponse>, Status> {
        let req = request.into_inner();
        let pagination = req.pagination.clone().unwrap_or_default();
        
        let query = ListBucketsQuery::from_grpc_request(
            pagination.page,
            pagination.page_size,
        );

        let log = grpc_log!("BucketService", "ListBucket", &ListRequest {
            page: &query.page,
            page_size: &query.page_size,
        });
        let trace_id = get_trace_id();

        let (buckets, total) = self
            .list_buckets_handler
            .handle(query)
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let bucket_list: Vec<BucketListItem> = buckets
            .iter()
            .map(|b| BucketListItem {
                id: b.id.clone(),
                global_aliases: b.global_aliases.clone(),
                local_aliases: b
                    .local_aliases
                    .iter()
                    .map(|la| LocalAlias {
                        access_key_id: la.access_key_id.clone(),
                        alias: la.alias.clone(),
                    })
                    .collect(),
                objects: b.objects as i64,
                bytes: b.bytes as i64,
                created: b.created.clone(),
            })
            .collect();

        let response = ListBucketsResponse {
            trace_id: trace_id.clone(),
            data: bucket_list.clone(),
            total: total as i32,
        };

        log.ok(&ApiResponseLog {
            trace_id: &trace_id,
            data: ListBucketsDataLog {
                buckets: bucket_list.iter().map(|b| BucketLog {
                    id: &b.id,
                    global_alias: &b.global_aliases,
                }).collect(),
            },
        });
        Ok(Response::new(response))
    }

    async fn read_bucket(
        &self,
        request: Request<ReadBucketRequest>,
    ) -> Result<Response<BucketResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("BucketService", "ReadBucket", &IdRequest { id: &req.id });
        let trace_id = get_trace_id();
        
        let bucket = self
            .get_bucket_handler
            .handle(GetBucketQuery { id: req.id })
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let bucket_data = convert_bucket(bucket);
        let response = BucketResponse {
            trace_id: trace_id.clone(),
            data: Some(bucket_data.clone()),
        };

        log.ok(&ApiResponseLogSimple {
            trace_id: &trace_id,
            data: BucketLogSimple {
                id: &bucket_data.id,
                global_alias: &bucket_data.global_aliases,
            },
        });
        Ok(Response::new(response))
    }

    async fn create_bucket(
        &self,
        request: Request<CreateBucketRequest>,
    ) -> Result<Response<BucketResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("BucketService", "CreateBucket", &CreateRequest { 
            global_alias: &req.global_alias 
        });
        let trace_id = get_trace_id();
        
        let command = CreateBucketCommand::new(
            req.global_alias,
            req.local_alias.map(|la| CreateLocalAliasCommand {
                access_key_id: la.access_key_id,
                alias: la.alias,
                allow_read: la.permissions.as_ref().map(|p| p.read).unwrap_or(false),
                allow_write: la.permissions.as_ref().map(|p| p.write).unwrap_or(false),
                allow_owner: la.permissions.as_ref().map(|p| p.owner).unwrap_or(false),
            }),
            req.quotas.and_then(|q| Quotas::new(q.max_size, q.max_objects).ok()),
            req.website_config.map(|wc| WebsiteConfig {
                index_document: wc.index_document.unwrap_or_default(),
                error_document: wc.error_document.unwrap_or_default(),
            }),
        );

        let (bucket_id, _event) = self
            .create_bucket_handler
            .handle(command)
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        // 取得新建的 bucket 詳細資訊
        let bucket = self
            .get_bucket_handler
            .handle(GetBucketQuery { id: bucket_id.clone() })
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let bucket_data = convert_bucket(bucket);
        let response = BucketResponse {
            trace_id: trace_id.clone(),
            data: Some(bucket_data.clone()),
        };

        log.ok(&ApiResponseLogSimple { 
            trace_id: &trace_id, 
            data: BucketLogSimple { 
                id: &bucket_data.id, 
                global_alias: &bucket_data.global_aliases, 
            } 
        });
        Ok(Response::new(response))
    }

    async fn update_bucket(
        &self,
        request: Request<UpdateBucketRequest>,
    ) -> Result<Response<BucketResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("BucketService", "UpdateBucket", &UpdateRequest { 
            id: &req.id,
        });
        let trace_id = get_trace_id();

        let command = UpdateBucketCommand::from_grpc_request(
            req.id,
            req.website_access.into_update_field(),
            req.website_config.map(|wc| WebsiteConfig {
                index_document: wc.index_document.unwrap_or_default(),
                error_document: wc.error_document.unwrap_or_default(),
            }),
            req.quotas.and_then(|q| Quotas::new(q.max_size, q.max_objects).ok()),
        );

        let (bucket, _event) = self
            .update_bucket_handler
            .handle(command)
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let bucket_data = convert_bucket(bucket);
        let response = BucketResponse {
            trace_id: trace_id.clone(),
            data: Some(bucket_data.clone()),
        };

        log.ok(&ApiResponseLogSimple {
            trace_id: &trace_id,
            data: BucketLogSimple {
                id: &bucket_data.id,
                global_alias: &bucket_data.global_aliases,
            },
        });
        Ok(Response::new(response))
    }

    async fn delete_bucket(
        &self,
        request: Request<DeleteBucketRequest>,
    ) -> Result<Response<DeleteBucketResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("BucketService", "DeleteBucket", &IdsRequest { ids: &req.id });
        let trace_id = get_trace_id();
        
        let command = DeleteBucketCommand::new(req.id);

        let deleted_ids = self.delete_bucket_handler
            .handle(command)
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let response = DeleteBucketResponse {
            trace_id: trace_id.clone(),
            id: deleted_ids.clone(),
        };

        log.ok(&ApiResponseLogIds { trace_id: &trace_id, data: IdsData { ids: &deleted_ids } });
        Ok(Response::new(response))
    }

    async fn add_bucket_alias(
        &self,
        request: Request<AddBucketAliasRequest>,
    ) -> Result<Response<BucketAliasResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("BucketService", "AddBucketAlias", &BatchRequest { 
            count: req.items.len(),
        });
        let trace_id = get_trace_id();

        use crate::infrastructure::grpc::generated::bucket::bucket_alias_item::AliasType;

        let mut results = Vec::new();

        for item in req.items {
            let command_result = match item.alias_type {
                Some(AliasType::GlobalAlias(alias)) => {
                    let command = AddBucketAliasCommand::new_global(item.bucket_id.clone(), alias.clone());
                    match self.add_bucket_alias_handler.handle(command).await {
                        Ok((bucket, _event)) => GrpcAliasResult {
                            bucket_id: item.bucket_id,
                            alias: alias,
                            success: true,
                            error: None,
                            bucket: Some(convert_bucket(bucket)),
                        },
                        Err(e) => GrpcAliasResult {
                            bucket_id: item.bucket_id,
                            alias: alias,
                            success: false,
                            error: Some(e.to_string()),
                            bucket: None,
                        },
                    }
                }
                Some(AliasType::LocalAlias(la)) => {
                    let alias = la.alias.clone();
                    let command = AddBucketAliasCommand::new_local(
                        item.bucket_id.clone(),
                        la.access_key_id,
                        la.alias,
                    );
                    match self.add_bucket_alias_handler.handle(command).await {
                        Ok((bucket, _event)) => GrpcAliasResult {
                            bucket_id: item.bucket_id,
                            alias: alias,
                            success: true,
                            error: None,
                            bucket: Some(convert_bucket(bucket)),
                        },
                        Err(e) => GrpcAliasResult {
                            bucket_id: item.bucket_id,
                            alias: alias,
                            success: false,
                            error: Some(e.to_string()),
                            bucket: None,
                        },
                    }
                }
                None => GrpcAliasResult {
                    bucket_id: item.bucket_id,
                    alias: String::new(),
                    success: false,
                    error: Some("Missing alias type".to_string()),
                    bucket: None,
                },
            };
            results.push(command_result);
        }

        let response = BucketAliasResponse {
            trace_id: trace_id.clone(),
            results,
        };

        log.ok(&BatchResponseLog { 
            trace_id: &trace_id, 
        });
        Ok(Response::new(response))
    }

    async fn remove_bucket_alias(
        &self,
        request: Request<RemoveBucketAliasRequest>,
    ) -> Result<Response<BucketAliasResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("BucketService", "RemoveBucketAlias", &BatchRequest { 
            count: req.items.len(),
        });
        let trace_id = get_trace_id();

        use crate::infrastructure::grpc::generated::bucket::bucket_alias_item::AliasType;

        let mut results = Vec::new();

        for item in req.items {
            let command_result = match item.alias_type {
                Some(AliasType::GlobalAlias(alias)) => {
                    let command = RemoveBucketAliasCommand::new_global(item.bucket_id.clone(), alias.clone());
                    match self.remove_bucket_alias_handler.handle(command).await {
                        Ok((bucket, _event)) => GrpcAliasResult {
                            bucket_id: item.bucket_id,
                            alias: alias,
                            success: true,
                            error: None,
                            bucket: Some(convert_bucket(bucket)),
                        },
                        Err(e) => GrpcAliasResult {
                            bucket_id: item.bucket_id,
                            alias: alias,
                            success: false,
                            error: Some(e.to_string()),
                            bucket: None,
                        },
                    }
                }
                Some(AliasType::LocalAlias(la)) => {
                    let alias = la.alias.clone();
                    let command = RemoveBucketAliasCommand::new_local(
                        item.bucket_id.clone(),
                        la.access_key_id,
                        la.alias,
                    );
                    match self.remove_bucket_alias_handler.handle(command).await {
                        Ok((bucket, _event)) => GrpcAliasResult {
                            bucket_id: item.bucket_id,
                            alias: alias,
                            success: true,
                            error: None,
                            bucket: Some(convert_bucket(bucket)),
                        },
                        Err(e) => GrpcAliasResult {
                            bucket_id: item.bucket_id,
                            alias: alias,
                            success: false,
                            error: Some(e.to_string()),
                            bucket: None,
                        },
                    }
                }
                None => GrpcAliasResult {
                    bucket_id: item.bucket_id,
                    alias: String::new(),
                    success: false,
                    error: Some("Missing alias type".to_string()),
                    bucket: None,
                },
            };
            results.push(command_result);
        }

        let response = BucketAliasResponse {
            trace_id: trace_id.clone(),
            results,
        };

        log.ok(&BatchResponseLog { 
            trace_id: &trace_id, 
        });
        Ok(Response::new(response))
    }

    async fn allow_bucket_key(
        &self,
        request: Request<BucketKeyPermissionRequest>,
    ) -> Result<Response<BucketKeyPermissionResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("BucketService", "AllowBucketKey", &BatchRequest { 
            count: req.items.len(),
        });
        let trace_id = get_trace_id();

        let items: Vec<BucketKeyPermissionItem> = req.items.into_iter().map(|item| {
            let permissions = item.permissions.unwrap_or_default();
            BucketKeyPermissionItem::new(
                item.bucket_id,
                item.access_key_id,
                BucketKeyPermissionInput {
                    read: permissions.read,
                    write: permissions.write,
                    owner: permissions.owner,
                },
            )
        }).collect();

        let command = BatchAllowBucketKeyCommand::new(items);

        let results = self
            .allow_bucket_key_handler
            .handle(command)
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let grpc_results: Vec<GrpcPermissionResult> = results.into_iter().map(|r| {
            GrpcPermissionResult {
                bucket_id: r.bucket_id,
                access_key_id: r.access_key_id,
                success: r.success,
                error: r.error,
                bucket: r.bucket.map(convert_bucket),
            }
        }).collect();

        let response = BucketKeyPermissionResponse {
            trace_id: trace_id.clone(),
            results: grpc_results,
        };

        log.ok(&BatchResponseLog { 
            trace_id: &trace_id, 
        });
        Ok(Response::new(response))
    }

    async fn deny_bucket_key(
        &self,
        request: Request<BucketKeyPermissionRequest>,
    ) -> Result<Response<BucketKeyPermissionResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("BucketService", "DenyBucketKey", &BatchRequest { 
            count: req.items.len(),
        });
        let trace_id = get_trace_id();

        let items: Vec<BucketKeyPermissionItem> = req.items.into_iter().map(|item| {
            let permissions = item.permissions.unwrap_or_default();
            BucketKeyPermissionItem::new(
                item.bucket_id,
                item.access_key_id,
                BucketKeyPermissionInput {
                    read: permissions.read,
                    write: permissions.write,
                    owner: permissions.owner,
                },
            )
        }).collect();

        let command = BatchDenyBucketKeyCommand::new(items);

        let results = self
            .deny_bucket_key_handler
            .handle(command)
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let grpc_results: Vec<GrpcPermissionResult> = results.into_iter().map(|r| {
            GrpcPermissionResult {
                bucket_id: r.bucket_id,
                access_key_id: r.access_key_id,
                success: r.success,
                error: r.error,
                bucket: r.bucket.map(convert_bucket),
            }
        }).collect();

        let response = BucketKeyPermissionResponse {
            trace_id: trace_id.clone(),
            results: grpc_results,
        };

        log.ok(&BatchResponseLog { 
            trace_id: &trace_id, 
        });
        Ok(Response::new(response))
    }
}

// ============ Log DTOs ============
#[derive(Serialize)]
struct ListRequest<'a> { 
    page: &'a i32,
    page_size: &'a i32,
}

#[derive(Serialize)]
struct IdRequest<'a> { id: &'a str }

#[derive(Serialize)]
struct IdsRequest<'a> { ids: &'a [String] }

#[derive(Serialize)]
struct CreateRequest<'a> { global_alias: &'a Option<String> }

#[derive(Serialize)]
struct UpdateRequest<'a> { 
    id: &'a str,
}

#[derive(Serialize)]
struct BatchRequest { 
    count: usize,
}

#[derive(Serialize)]
struct BatchResponseLog<'a> { 
    trace_id: &'a str,
}

#[derive(Serialize)]
struct ApiResponseLog<'a, T: Serialize> {
    trace_id: &'a str,
    data: T,
}

#[derive(Serialize)]
struct ListBucketsDataLog<'a> {
    buckets: Vec<BucketLog<'a>>,
}

#[derive(Serialize)]
struct BucketLog<'a> {
    id: &'a str,
    global_alias: &'a Vec<String>,
}

#[derive(Serialize)]
struct ApiResponseLogSimple<'a> {
    trace_id: &'a str,
    data: BucketLogSimple<'a>,
}

#[derive(Serialize)]
struct BucketLogSimple<'a> {
    id: &'a str,
    global_alias: &'a Vec<String>,
}

#[derive(Serialize)]
struct ApiResponseLogIds<'a> {
    trace_id: &'a str,
    data: IdsData<'a>,
}

#[derive(Serialize)]
struct IdsData<'a> { ids: &'a [String] }

// ============ Helpers ============

fn convert_bucket(bucket: crate::domain::entities::BucketDetail) -> Bucket {
    Bucket {
        created: bucket.created.clone(),
        objects: bucket.objects as i64,
        bytes: bucket.bytes as i64,
        id: bucket.id,
        global_aliases: bucket.global_aliases,
        local_aliases: bucket
            .local_aliases
            .into_iter()
            .map(|la| LocalAlias {
                access_key_id: la.access_key_id,
                alias: la.alias,
            })
            .collect(),
        website_access: bucket.website_access,
        website_config: bucket
            .website_config
            .map(|wc| format!("index: {}, error: {}", wc.index_document, wc.error_document)),
        keys: bucket
            .keys
            .into_iter()
            .map(|k| BucketKey {
                access_key_id: k.access_key_id,
                name: k.name,
                permissions: Some(BucketKeyPermissions {
                    read: k.permissions.read,
                    write: k.permissions.write,
                    owner: k.permissions.owner,
                }),
                bucket_local_aliases: !k.bucket_local_aliases.is_empty(),
            })
            .collect(),
        quotas: Some(crate::infrastructure::grpc::generated::bucket::Quotas {
            max_size: bucket.quotas.max_size(),
            max_objects: bucket.quotas.max_objects(),
        }),
    }
}
