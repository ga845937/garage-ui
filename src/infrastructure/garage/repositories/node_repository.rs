//! Node Repository Implementation
//!
//! Infrastructure 層的 Repository 具體實現

use async_trait::async_trait;
use crate::domain::entities::{MultiNodeResponse, NodeInfo, NodeStatistics};
use crate::domain::errors::DomainError;
use crate::domain::repositories::NodeRepository;
use crate::infrastructure::garage::api::{
    LaunchRepairRequest, MultiNodeResponse as ApiMultiNodeResponse, NodeInfoResponse, NodeStatisticsResponse,
};
use crate::infrastructure::garage::client::GarageClient;
use crate::infrastructure::garage::endpoints::GarageApiEndpoint;

/// Node Repository 實現
pub struct GarageNodeRepository {
    client: GarageClient,
}

impl GarageNodeRepository {
    pub fn new(client: GarageClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl NodeRepository for GarageNodeRepository {
    async fn get_info(&self, node: &str) -> Result<MultiNodeResponse<NodeInfo>, DomainError> {
        let path = format!("{}?node={}", GarageApiEndpoint::GetNodeInfo.path(), node);
        let response: ApiMultiNodeResponse<NodeInfoResponse> = self.client.get(&path).await?;
        Ok(MultiNodeResponse {
            success: response.success.into_iter().map(|(k, v)| (k, NodeInfo {
                node_id: v.node_id,
                node_addr: v.node_addr,
                zone: v.zone,
                capacity: v.capacity,
                tags: v.tags,
                garage_version: v.garage_version,
                garage_features: v.garage_features,
                rust_version: v.rust_version,
                db_engine: v.db_engine,
            })).collect(),
            error: response.error,
        })
    }
    
    async fn get_statistics(&self, node: &str) -> Result<MultiNodeResponse<NodeStatistics>, DomainError> {
        let path = format!("{}?node={}", GarageApiEndpoint::GetNodeStatistics.path(), node);
        let response: ApiMultiNodeResponse<NodeStatisticsResponse> = self.client.get(&path).await?;
        Ok(MultiNodeResponse {
            success: response.success.into_iter().map(|(k, v)| (k, NodeStatistics {
                freeform: v.freeform,
            })).collect(),
            error: response.error,
        })
    }
    
    async fn create_metadata_snapshot(&self, node: &str) -> Result<MultiNodeResponse<()>, DomainError> {
        let path = format!("{}?node={}", GarageApiEndpoint::CreateMetadataSnapshot.path(), node);
        let response: ApiMultiNodeResponse<()> = self.client.post_with_empty_body(&path).await?;
        Ok(MultiNodeResponse {
            success: response.success.into_iter().map(|(k, _)| (k, ())).collect(),
            error: response.error,
        })
    }
    
    async fn launch_repair(&self, node: &str, repair_type: &str) -> Result<MultiNodeResponse<()>, DomainError> {
        use crate::infrastructure::garage::api::RepairTypeRequest;
        
        let path = format!("{}?node={}", GarageApiEndpoint::LaunchRepairOperation.path(), node);
        let request = LaunchRepairRequest {
            repair_type: RepairTypeRequest::Simple(repair_type.to_string()),
        };
        let response: ApiMultiNodeResponse<()> = self.client.post(&path, &request).await?;
        Ok(MultiNodeResponse {
            success: response.success.into_iter().map(|(k, _)| (k, ())).collect(),
            error: response.error,
        })
    }
}
