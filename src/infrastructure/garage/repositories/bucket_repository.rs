//! Bucket repository implementation using Garage API

use async_trait::async_trait;
use crate::domain::aggregates::BucketAggregate;
use crate::domain::entities::{BucketDetail, BucketKey, BucketKeyPermissions, WebsiteConfig};
use crate::domain::errors::DomainError;
use crate::domain::repositories::{BucketRepository, CreateBucketInput};
use crate::domain::value_objects::{LocalAlias, Quotas};
use crate::infrastructure::garage::client::{ GarageClient, GarageApiEndpoint };
use crate::infrastructure::garage::api::{
    AddGlobalAliasRequest, AddLocalAliasRequest, RemoveGlobalAliasRequest, RemoveLocalAliasRequest,
};
use crate::domain::entities::garage::{      
    CreateBucketRequest, CreateBucketResponse,
    CreateLocalAliasAllow, CreateLocalAliasRequest,
    GarageBucketDetailResponse, GarageBucketInfo,
    UpdateBucketRequest, UpdateQuotas, UpdateWebsiteAccess,
 };

/// Implementation of BucketRepository using Garage API
pub struct GarageBucketRepository {
    client: GarageClient,
}

impl GarageBucketRepository {
    pub fn new(client: GarageClient) -> Self {
        Self { client }
    }

    /// 將 Garage API 回應轉換為 BucketDetail
    fn convert_to_bucket_detail(&self, detail: GarageBucketDetailResponse) -> BucketDetail {
        BucketDetail {
            created: detail.created,
            objects: detail.objects,
            bytes: detail.bytes,
            id: detail.id,
            global_aliases: detail.global_aliases,
            local_aliases: detail
                .local_aliases
                .into_iter()
                .filter_map(|la| LocalAlias::new(la.access_key_id, la.alias).ok())
                .collect(),
            website_access: detail.website_access,
            website_config: detail.website_config.map(|wc| WebsiteConfig {
                index_document: wc.index_document,
                error_document: wc.error_document,
            }),
            keys: detail
                .keys
                .into_iter()
                .map(|k| BucketKey {
                    access_key_id: k.access_key_id,
                    name: k.name,
                    permissions: BucketKeyPermissions {
                        read: k.permissions.read,
                        write: k.permissions.write,
                        owner: k.permissions.owner,
                    },
                    bucket_local_aliases: k.bucket_local_aliases,
                })
                .collect(),
            quotas: Quotas::new(detail.quotas.max_size, detail.quotas.max_objects)
                .unwrap_or_else(|_| Quotas::unlimited()),
        }
    }
}

#[async_trait]
impl BucketRepository for GarageBucketRepository {
    // ============ Aggregate 操作 ============
    
    async fn save(&self, aggregate: &BucketAggregate) -> Result<(), DomainError> {
        // 更新 Garage (使用 UpdateBucket API)
        let update_request = UpdateBucketRequest {
            website_access: if aggregate.website_access() {
                aggregate.website_config().map(|wc| UpdateWebsiteAccess {
                    enabled: true,
                    index_document: Some(wc.index_document.clone()),
                    error_document: Some(wc.error_document.clone()),
                })
            } else {
                None
            },
            quotas: Some(UpdateQuotas {
                max_size: aggregate.quotas().max_size(),
                max_objects: aggregate.quotas().max_objects(),
            }),
        };

        let path = format!("{}?id={}", GarageApiEndpoint::UpdateBucket.path(), aggregate.id());
        let _: GarageBucketDetailResponse = self.client.post(&path, &update_request).await?;
        
        Ok(())
    }
    
    async fn load(&self, id: &str) -> Result<BucketAggregate, DomainError> {
        let detail = self.get_detail(id).await?;
        
        // 從持久化資料重建 Aggregate
        Ok(BucketAggregate::reconstitute(
            detail.id,
            detail.global_aliases,
            detail.local_aliases,
            detail.quotas,
            detail.website_access,
            detail.website_config,
            detail.keys,
            detail.objects,
            detail.bytes,
        ))
    }
    
    // ============ 查詢操作（CQRS Read Side）============
    
    async fn list(&self) -> Result<Vec<GarageBucketInfo>, DomainError> {
        let buckets: Vec<GarageBucketInfo> = self.client.get(GarageApiEndpoint::ListBuckets.path()).await?;
        
        Ok(buckets)
    }
    
    async fn get_detail(&self, id: &str) -> Result<BucketDetail, DomainError> {
        let path = format!("{}?id={}", GarageApiEndpoint::GetBucketInfo.path(), id);
        let detail: GarageBucketDetailResponse = self.client.get(&path).await?;
        Ok(self.convert_to_bucket_detail(detail))
    }
    
    // ============ Infrastructure 操作 ============
    
    async fn create_bucket(&self, input: CreateBucketInput) -> Result<String, DomainError> {
        let request = CreateBucketRequest {
            global_alias: input.global_alias,
            local_alias: input.local_alias.map(|la| CreateLocalAliasRequest {
                access_key_id: la.access_key_id,
                alias: la.alias,
                allow: CreateLocalAliasAllow {
                    read: la.allow_read,
                    write: la.allow_write,
                    owner: la.allow_owner,
                },
            }),
        };

        let response: CreateBucketResponse = self.client.post(GarageApiEndpoint::CreateBucket.path(), &request).await?;
        Ok(response.id)
    }

    async fn delete_bucket(&self, id: &str) -> Result<(), DomainError> {
        let path = format!("{}?id={}", GarageApiEndpoint::DeleteBucket.path(), id);
        self.client.post_empty(&path).await
    }

    // ============ Alias 操作 ============

    async fn add_global_alias(&self, bucket_id: &str, alias: &str) -> Result<BucketDetail, DomainError> {
        let request = AddGlobalAliasRequest {
            bucket_id: bucket_id.to_string(),
            global_alias: alias.to_string(),
        };
        let response: GarageBucketDetailResponse = self.client.post(
            GarageApiEndpoint::AddBucketAlias.path(),
            &request
        ).await?;
        Ok(self.convert_to_bucket_detail(response))
    }

    async fn add_local_alias(&self, bucket_id: &str, access_key_id: &str, alias: &str) -> Result<BucketDetail, DomainError> {
        let request = AddLocalAliasRequest {
            bucket_id: bucket_id.to_string(),
            access_key_id: access_key_id.to_string(),
            local_alias: alias.to_string(),
        };
        let response: GarageBucketDetailResponse = self.client.post(
            GarageApiEndpoint::AddBucketAlias.path(),
            &request
        ).await?;
        Ok(self.convert_to_bucket_detail(response))
    }

    async fn remove_global_alias(&self, bucket_id: &str, alias: &str) -> Result<BucketDetail, DomainError> {
        let request = RemoveGlobalAliasRequest {
            bucket_id: bucket_id.to_string(),
            global_alias: alias.to_string(),
        };
        let response: GarageBucketDetailResponse = self.client.post(
            GarageApiEndpoint::RemoveBucketAlias.path(),
            &request
        ).await?;
        Ok(self.convert_to_bucket_detail(response))
    }

    async fn remove_local_alias(&self, bucket_id: &str, access_key_id: &str, alias: &str) -> Result<BucketDetail, DomainError> {
        let request = RemoveLocalAliasRequest {
            bucket_id: bucket_id.to_string(),
            access_key_id: access_key_id.to_string(),
            local_alias: alias.to_string(),
        };
        let response: GarageBucketDetailResponse = self.client.post(
            GarageApiEndpoint::RemoveBucketAlias.path(),
            &request
        ).await?;
        Ok(self.convert_to_bucket_detail(response))
    }

    // ============ Permission 操作 ============

    async fn allow_bucket_key(
        &self,
        bucket_id: &str,
        access_key_id: &str,
        read: bool,
        write: bool,
        owner: bool,
    ) -> Result<BucketDetail, DomainError> {
        use crate::infrastructure::garage::api::{AllowBucketKeyRequest, BucketKeyPermRequest};

        let request = AllowBucketKeyRequest {
            bucket_id: bucket_id.to_string(),
            access_key_id: access_key_id.to_string(),
            permissions: BucketKeyPermRequest {
                read: if read { Some(true) } else { None },
                write: if write { Some(true) } else { None },
                owner: if owner { Some(true) } else { None },
            },
        };
        let response: GarageBucketDetailResponse = self.client.post(
            GarageApiEndpoint::AllowBucketKey.path(),
            &request
        ).await?;
        Ok(self.convert_to_bucket_detail(response))
    }

    async fn deny_bucket_key(
        &self,
        bucket_id: &str,
        access_key_id: &str,
        read: bool,
        write: bool,
        owner: bool,
    ) -> Result<BucketDetail, DomainError> {
        use crate::infrastructure::garage::api::{DenyBucketKeyRequest, BucketKeyPermRequest};

        let request = DenyBucketKeyRequest {
            bucket_id: bucket_id.to_string(),
            access_key_id: access_key_id.to_string(),
            permissions: BucketKeyPermRequest {
                read: if read { Some(true) } else { None },
                write: if write { Some(true) } else { None },
                owner: if owner { Some(true) } else { None },
            },
        };
        let response: GarageBucketDetailResponse = self.client.post(
            GarageApiEndpoint::DenyBucketKey.path(),
            &request
        ).await?;
        Ok(self.convert_to_bucket_detail(response))
    }
}
