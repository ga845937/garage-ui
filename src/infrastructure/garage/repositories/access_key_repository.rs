//! Access Key Repository Implementation
//!
//! Infrastructure 層的 Repository 具體實現
//! CQRS: 分離 Command 和 Query Repository

use async_trait::async_trait;
use crate::domain::errors::DomainError;
use crate::domain::repositories::{AccessKeyCommandRepository, AccessKeyQueryRepository};
use crate::domain::entities::{AccessKey, AccessKeyListItem, KeyPermissions, KeyBucket, BucketPermissions};
use crate::domain::aggregates::{AccessKeyAggregate, BucketVO, BucketPermissionVO};
use crate::domain::entities::garage::{
    CreateKeyRequest, KeyInfoResponse, KeyUpdateResponse, KeyListItemResponse, UpdateKeyRequest, KeyPermRequest,
};
use crate::infrastructure::garage::client::GarageClient;
use crate::infrastructure::garage::endpoints::GarageApiEndpoint;
use crate::shared::parse_datetime;

// ============ Command Repository ============

/// Access Key Command Repository 實現
/// 用於寫入操作，返回 Aggregate
pub struct GarageAccessKeyCommandRepository {
    client: GarageClient,
}

impl GarageAccessKeyCommandRepository {
    pub fn new(client: GarageClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl AccessKeyCommandRepository for GarageAccessKeyCommandRepository {
    async fn get(&self, id: &str) -> Result<AccessKeyAggregate, DomainError> {
        let path = format!("{}?id={}&showSecretKey=true", 
            GarageApiEndpoint::GetKeyInfo.path(), id);

        let response: KeyInfoResponse = self.client.get(&path).await?;
        Ok(map_response_to_aggregate(response))
    }
    
    async fn create(&self, aggregate: &AccessKeyAggregate) -> Result<AccessKeyAggregate, DomainError> {
        let request = CreateKeyRequest {
            name: Some(aggregate.name().to_string()),
            expiration: aggregate.expiration().map(|e| e.to_rfc3339()),
            never_expires: if aggregate.expiration().is_some() { Some(false) } else { Some(true) },
            allow: Some(KeyPermRequest {
                create_bucket: Some(aggregate.can_create_bucket()),
            }),
            deny: Some(KeyPermRequest {
                create_bucket: Some(!aggregate.can_create_bucket()),
            }),
        };

        let response: KeyInfoResponse = self.client.post(GarageApiEndpoint::CreateKey.path(), &request).await?;
        Ok(map_response_to_aggregate(response))
    }
    
    async fn save(&self, aggregate: &AccessKeyAggregate) -> Result<AccessKeyAggregate, DomainError> {
        let path = format!("{}?id={}", GarageApiEndpoint::UpdateKey.path(), aggregate.id());
        
        let request = UpdateKeyRequest {
            name: Some(aggregate.name().to_string()),
            expiration: aggregate.expiration().map(|e| e.to_rfc3339()),
            never_expires: Some(aggregate.expiration().is_none()),
            allow: Some(KeyPermRequest {
                create_bucket: Some(aggregate.can_create_bucket()),
            }),
            deny: Some(KeyPermRequest {
                create_bucket: Some(!aggregate.can_create_bucket()),
            }),
        };

        let response: KeyUpdateResponse = self.client.post(&path, &request).await?;
        Ok(map_update_response_to_aggregate(response, aggregate.secret_access_key()))
    }
    
    async fn delete(&self, aggregate: &AccessKeyAggregate) -> Result<(), DomainError> {
        let path = format!("{}?id={}", GarageApiEndpoint::DeleteKey.path(), aggregate.id());
        self.client.post_empty(&path).await
    }
}

// ============ Query Repository ============

/// Access Key Query Repository 實現
/// 用於讀取操作，直接返回 Read Model (DTO)
pub struct GarageAccessKeyQueryRepository {
    client: GarageClient,
}

impl GarageAccessKeyQueryRepository {
    pub fn new(client: GarageClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl AccessKeyQueryRepository for GarageAccessKeyQueryRepository {
    async fn list(&self) -> Result<Vec<KeyListItemResponse>, DomainError> {
        let response: Vec<KeyListItemResponse> = self.client
            .get(GarageApiEndpoint::ListKeys.path())
            .await?;

        return Ok(response);
    }
    
    async fn find_by_id(&self, id: &str) -> Result<AccessKey, DomainError> {
        let path = format!("{}?id={}&showSecretKey=true", 
            GarageApiEndpoint::GetKeyInfo.path(), id);

        let response: KeyInfoResponse = self.client.get(&path).await?;
        
        let created = parse_datetime(&response.created).expect("Invalid created datetime");
        let expiration = response.expiration.as_deref().and_then(parse_datetime);
        
        Ok(AccessKey {
            id: response.access_key_id,
            name: response.name,
            secret_access_key: response.secret_access_key,
            created,
            expiration,
            expired: AccessKeyListItem::compute_expired(expiration),
            permissions: KeyPermissions {
                create_bucket: response.permissions.create_bucket,
            },
            buckets: response.buckets.into_iter().map(|b| KeyBucket {
                id: b.id,
                global_aliases: b.global_aliases,
                local_aliases: b.local_aliases,
                permissions: BucketPermissions {
                    read: b.permissions.read,
                    write: b.permissions.write,
                    owner: b.permissions.owner,
                },
            }).collect(),
        })
    }
}

// ============ Mapping Functions ============

/// 將 API Response 轉換為 Domain Aggregate
fn map_response_to_aggregate(response: KeyInfoResponse) -> AccessKeyAggregate {
    let created = parse_datetime(&response.created).expect("Invalid created datetime");
    let expiration = response.expiration.as_deref().and_then(parse_datetime);
    
    let buckets: Vec<BucketVO> = response.buckets.into_iter().map(|b| {
        BucketVO::new(
            b.id,
            b.global_aliases,
            b.local_aliases,
            BucketPermissionVO::new(
                b.permissions.owner,
                b.permissions.read,
                b.permissions.write,
            ),
        )
    }).collect();
    
    AccessKeyAggregate::reconstitute(
        response.access_key_id,
        response.name,
        buckets,
        created,
        expiration,
        response.permissions.create_bucket,
        response.secret_access_key,
    )
}

/// 將 Update API Response 轉換為 Domain Aggregate
/// 
/// Update 操作不返回 secretAccessKey，需要從原 Aggregate 保留
fn map_update_response_to_aggregate(response: KeyUpdateResponse, secret_key: &str) -> AccessKeyAggregate {
    let created = parse_datetime(&response.created).expect("Invalid created datetime");
    let expiration = response.expiration.as_deref().and_then(parse_datetime);
    
    let buckets: Vec<BucketVO> = response.buckets.into_iter().map(|b| {
        BucketVO::new(
            b.id,
            b.global_aliases,
            b.local_aliases,
            BucketPermissionVO::new(
                b.permissions.owner,
                b.permissions.read,
                b.permissions.write,
            ),
        )
    }).collect();
    
    AccessKeyAggregate::reconstitute(
        response.access_key_id,
        response.name,
        buckets,
        created,
        expiration,
        response.permissions.create_bucket,
        secret_key.to_string(),
    )
}
