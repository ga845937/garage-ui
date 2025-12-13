//! Admin Token Repository Implementation
//!
//! Infrastructure 層的 Repository 具體實現

use async_trait::async_trait;
use crate::domain::entities::{AdminTokenCreated, AdminTokenInfo, AdminTokenListItem};
use crate::domain::errors::DomainError;
use crate::domain::repositories::{
    AdminTokenRepository, CreateAdminTokenInput, UpdateAdminTokenInput,
};
use crate::infrastructure::garage::api::{
    AdminTokenCreatedResponse, AdminTokenInfoResponse, AdminTokenListItemResponse,
    CreateAdminTokenRequest, UpdateAdminTokenRequest,
};
use crate::infrastructure::garage::client::GarageClient;
use crate::infrastructure::garage::endpoints::GarageApiEndpoint;

/// Admin Token Repository 實現
pub struct GarageAdminTokenRepository {
    client: GarageClient,
}

impl GarageAdminTokenRepository {
    pub fn new(client: GarageClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl AdminTokenRepository for GarageAdminTokenRepository {
    async fn list(&self) -> Result<Vec<AdminTokenListItem>, DomainError> {
        let response: Vec<AdminTokenListItemResponse> = self.client.get(GarageApiEndpoint::ListAdminTokens.path()).await?;
        Ok(response.into_iter().map(|t| AdminTokenListItem {
            id: t.id,
            name: t.name,
            created: t.created,
            expiration: t.expiration,
            expired: t.expired,
            scope: t.scope,
        }).collect())
    }
    
    async fn get(&self, id: &str) -> Result<AdminTokenInfo, DomainError> {
        let path = format!("{}?id={}", GarageApiEndpoint::GetAdminTokenInfo.path(), id);
        let response: AdminTokenInfoResponse = self.client.get(&path).await?;
        Ok(convert_token_info(response))
    }
    
    async fn search(&self, query: &str) -> Result<AdminTokenInfo, DomainError> {
        let path = format!("{}?search={}", GarageApiEndpoint::GetAdminTokenInfo.path(), query);
        let response: AdminTokenInfoResponse = self.client.get(&path).await?;
        Ok(convert_token_info(response))
    }
    
    async fn get_current(&self) -> Result<AdminTokenInfo, DomainError> {
        let response: AdminTokenInfoResponse = self.client.get(GarageApiEndpoint::GetCurrentAdminTokenInfo.path()).await?;
        Ok(convert_token_info(response))
    }
    
    async fn create(&self, input: CreateAdminTokenInput) -> Result<AdminTokenCreated, DomainError> {
        let request = CreateAdminTokenRequest {
            name: input.name,
            expiration: input.expiration,
            never_expires: None,
            scope: input.scope,
        };
        let response: AdminTokenCreatedResponse = self.client.post(GarageApiEndpoint::CreateAdminToken.path(), &request).await?;
        Ok(AdminTokenCreated {
            id: response.id,
            name: response.name,
            created: response.created,
            expiration: response.expiration,
            expired: response.expired,
            scope: response.scope,
            secret_token: response.secret_token,
        })
    }
    
    async fn update(&self, id: &str, input: UpdateAdminTokenInput) -> Result<AdminTokenInfo, DomainError> {
        let path = format!("{}?id={}", GarageApiEndpoint::UpdateAdminToken.path(), id);
        let request = UpdateAdminTokenRequest {
            name: input.name,
            expiration: input.expiration,
            never_expires: None,
            scope: input.scope,
        };
        let response: AdminTokenInfoResponse = self.client.post(&path, &request).await?;
        Ok(convert_token_info(response))
    }
    
    async fn delete(&self, id: &str) -> Result<(), DomainError> {
        let path = format!("{}?id={}", GarageApiEndpoint::DeleteAdminToken.path(), id);
        self.client.post_empty(&path).await
    }
}

fn convert_token_info(response: AdminTokenInfoResponse) -> AdminTokenInfo {
    AdminTokenInfo {
        id: response.id,
        name: response.name,
        created: response.created,
        expiration: response.expiration,
        expired: response.expired,
        scope: response.scope,
    }
}
