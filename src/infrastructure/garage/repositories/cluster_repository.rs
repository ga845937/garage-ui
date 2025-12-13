//! Cluster Repository Implementation
//!
//! Infrastructure 層的 Repository 具體實現

use async_trait::async_trait;
use crate::domain::entities::{
    ApplyLayoutResult, ClusterHealth, ClusterLayout, ClusterLayoutHistory, ClusterNode, ClusterStatistics,
    ClusterStatus, ConnectNodeResult, LayoutParameters, LayoutRole, LayoutVersion,
    PartitionInfo, RoleChangeType, SkipDeadNodesResult, StagedRoleChange, UpdateTracker, ZoneRedundancy,
};
use crate::domain::errors::DomainError;
use crate::domain::repositories::{ClusterRepository, UpdateLayoutInput};
use crate::infrastructure::garage::api::{
    ApplyLayoutRequest, ApplyLayoutResultResponse, ClusterHealthResponse, ClusterLayoutHistoryResponse,
    ClusterLayoutResponse, ClusterStatisticsResponse, ClusterStatusResponse, ConnectNodeResultResponse,
    ConnectNodesRequest, LayoutParametersResponse, LayoutRoleResponse, LayoutVersionResponse,
    NodeRoleResponse, PartitionInfoResponse, PreviewLayoutChangesResponse, SkipDeadNodesRequest,
    SkipDeadNodesResultResponse, UpdateLayoutRequest, UpdateTrackerResponse, ZoneRedundancyResponse,
};
use crate::infrastructure::garage::client::GarageClient;
use crate::infrastructure::garage::endpoints::GarageApiEndpoint;

/// Cluster Repository 實現
pub struct GarageClusterRepository {
    client: GarageClient,
}

impl GarageClusterRepository {
    pub fn new(client: GarageClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl ClusterRepository for GarageClusterRepository {
    async fn get_status(&self) -> Result<ClusterStatus, DomainError> {
        let response: ClusterStatusResponse = self.client.get(GarageApiEndpoint::GetClusterStatus.path()).await?;
        Ok(ClusterStatus {
            layout_version: response.layout_version,
            nodes: response.nodes.into_iter().map(|n| ClusterNode {
                id: n.id,
                addr: n.addr,
                is_up: n.is_up,
                last_seen_secs_ago: n.last_seen_secs_ago,
                hostname: n.hostname,
                role: n.role.map(|r| convert_node_role(r)),
                data_partition: n.data_partition.map(|p| convert_partition_info(p)),
                metadata_partition: n.metadata_partition.map(|p| convert_partition_info(p)),
            }).collect(),
        })
    }
    
    async fn get_health(&self) -> Result<ClusterHealth, DomainError> {
        let response: ClusterHealthResponse = self.client.get(GarageApiEndpoint::GetClusterHealth.path()).await?;
        Ok(ClusterHealth {
            status: response.status,
            known_nodes: response.known_nodes,
            connected_nodes: response.connected_nodes,
            storage_nodes: response.storage_nodes,
            storage_nodes_up: response.storage_nodes_up,
            partitions: response.partitions,
            partitions_quorum: response.partitions_quorum,
            partitions_all_ok: response.partitions_all_ok,
        })
    }
    
    async fn get_statistics(&self) -> Result<ClusterStatistics, DomainError> {
        let response: ClusterStatisticsResponse = self.client.get(GarageApiEndpoint::GetClusterStatistics.path()).await?;
        Ok(ClusterStatistics {
            freeform: response.freeform,
        })
    }
    
    async fn connect_nodes(&self, nodes: Vec<String>) -> Result<Vec<ConnectNodeResult>, DomainError> {
        let request: ConnectNodesRequest = nodes;
        let response: Vec<ConnectNodeResultResponse> = self.client.post(GarageApiEndpoint::ConnectClusterNodes.path(), &request).await?;
        Ok(response.into_iter().map(|r| ConnectNodeResult {
            success: r.success,
            error: r.error,
        }).collect())
    }
    
    async fn get_layout(&self) -> Result<ClusterLayout, DomainError> {
        let response: ClusterLayoutResponse = self.client.get(GarageApiEndpoint::GetClusterLayout.path()).await?;
        Ok(convert_layout(response))
    }
    
    async fn update_layout(&self, roles: Vec<UpdateLayoutInput>) -> Result<ClusterLayout, DomainError> {
        use crate::infrastructure::garage::api::RoleChangeRequest;
        
        let request = UpdateLayoutRequest {
            roles: Some(roles.into_iter().map(|r| RoleChangeRequest {
                id: r.node_id,
                remove: None,
                zone: r.zone,
                capacity: r.capacity.map(|c| c as i64),
                tags: r.tags,
            }).collect()),
            parameters: None,
        };
        let response: ClusterLayoutResponse = self.client.post(GarageApiEndpoint::UpdateClusterLayout.path(), &request).await?;
        Ok(convert_layout(response))
    }
    
    async fn apply_layout(&self, version: i64) -> Result<ApplyLayoutResult, DomainError> {
        let request = ApplyLayoutRequest { version };
        let response: ApplyLayoutResultResponse = self.client.post(GarageApiEndpoint::ApplyClusterLayout.path(), &request).await?;
        Ok(ApplyLayoutResult {
            layout: convert_layout(response.layout),
            message: response.message,
        })
    }
    
    async fn revert_layout(&self) -> Result<ClusterLayout, DomainError> {
        let response: ClusterLayoutResponse = self.client.post_with_empty_body(GarageApiEndpoint::RevertClusterLayout.path()).await?;
        Ok(convert_layout(response))
    }
    
    async fn preview_layout_changes(&self) -> Result<ApplyLayoutResult, DomainError> {
        let response: PreviewLayoutChangesResponse = self.client.post_with_empty_body(GarageApiEndpoint::PreviewClusterLayoutChanges.path()).await?;
        Ok(ApplyLayoutResult {
            layout: convert_layout(response.new_layout),
            message: response.message,
        })
    }
    
    async fn get_layout_history(&self) -> Result<ClusterLayoutHistory, DomainError> {
        let response: ClusterLayoutHistoryResponse = self.client.get(GarageApiEndpoint::GetClusterLayoutHistory.path()).await?;
        Ok(ClusterLayoutHistory {
            current_version: response.current_version,
            min_ack: response.min_ack,
            versions: response.versions.into_iter().map(|v| convert_layout_version(v)).collect(),
            update_trackers: response.update_trackers.into_iter()
                .map(|(k, v)| (k, convert_update_tracker(v)))
                .collect(),
        })
    }
    
    async fn skip_dead_nodes(&self, version: i64, allow_missing_data: bool) -> Result<SkipDeadNodesResult, DomainError> {
        let request = SkipDeadNodesRequest { version, allow_missing_data };
        let response: SkipDeadNodesResultResponse = self.client.post(GarageApiEndpoint::ClusterLayoutSkipDeadNodes.path(), &request).await?;
        Ok(SkipDeadNodesResult {
            ack_updated: response.ack_updated,
            sync_updated: response.sync_updated,
        })
    }
}

// ============ Conversion helpers ============

fn convert_node_role(response: NodeRoleResponse) -> crate::domain::entities::NodeRole {
    crate::domain::entities::NodeRole {
        zone: response.zone,
        capacity: response.capacity,
        tags: response.tags,
    }
}

fn convert_partition_info(response: PartitionInfoResponse) -> PartitionInfo {
    PartitionInfo {
        available: response.available,
        total: response.total,
    }
}

fn convert_layout(response: ClusterLayoutResponse) -> ClusterLayout {
    ClusterLayout {
        version: response.version,
        partition_size: response.partition_size,
        roles: response.roles.into_iter().map(|r| convert_layout_role(r)).collect(),
        staged_role_changes: response.staged_role_changes.into_iter().map(|r| {
            StagedRoleChange {
                id: r.id.clone(),
                change: if r.remove.unwrap_or(false) {
                    RoleChangeType::Remove { remove: true }
                } else {
                    RoleChangeType::Assign {
                        zone: r.zone.unwrap_or_default(),
                        capacity: r.capacity,
                        tags: r.tags,
                    }
                },
            }
        }).collect(),
        parameters: response.parameters.map(|p| convert_layout_parameters(p)),
        staged_parameters: response.staged_parameters.map(|p| convert_layout_parameters(p)),
    }
}

fn convert_layout_role(response: LayoutRoleResponse) -> LayoutRole {
    LayoutRole {
        id: response.id,
        zone: response.zone,
        capacity: response.capacity,
        tags: response.tags,
    }
}

fn convert_layout_parameters(response: LayoutParametersResponse) -> LayoutParameters {
    LayoutParameters {
        zone_redundancy: response.zone_redundancy.map(|zr| match zr {
            ZoneRedundancyResponse::Value(v) => ZoneRedundancy::Value(v),
            ZoneRedundancyResponse::Maximum { maximum } => ZoneRedundancy::Maximum { maximum },
        }),
    }
}

fn convert_layout_version(response: LayoutVersionResponse) -> LayoutVersion {
    LayoutVersion {
        version: response.version,
        partition_size: response.partition_size,
        roles: response.roles.into_iter().map(|r| convert_layout_role(r)).collect(),
    }
}

fn convert_update_tracker(response: UpdateTrackerResponse) -> UpdateTracker {
    UpdateTracker {
        ack: response.ack,
        sync: response.sync,
    }
}
