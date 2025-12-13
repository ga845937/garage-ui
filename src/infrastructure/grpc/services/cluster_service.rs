//! Cluster gRPC service implementation

use std::sync::Arc;
use serde::Serialize;
use tonic::{Request, Response, Status};

use crate::application::commands::cluster::{
    ConnectNodesCommand, UpdateLayoutCommand, ApplyLayoutCommand, 
    RevertLayoutCommand, SkipDeadNodesCommand, LayoutRoleChange,
};
use crate::application::commands::cluster::handlers::{
    ConnectNodesHandler, UpdateLayoutHandler, ApplyLayoutHandler,
    RevertLayoutHandler, SkipDeadNodesHandler,
};
use crate::application::queries::cluster::{
    GetClusterStatusQuery, GetClusterHealthQuery, GetClusterLayoutQuery,
    GetLayoutHistoryQuery, PreviewLayoutChangesQuery,
};
use crate::application::queries::cluster::handlers::{
    GetClusterStatusHandler, GetClusterHealthHandler, GetClusterLayoutHandler,
    GetLayoutHistoryHandler, PreviewLayoutChangesHandler,
};
use crate::infrastructure::grpc::conversions::domain_error_to_status;
use crate::grpc_log;
use crate::shared::get_trace_id;

use crate::infrastructure::grpc::generated::cluster::{
    cluster_service_server::ClusterService,
    ApiResponse, api_response::Data,
    GetClusterStatusRequest, ClusterStatusData, ClusterNode,
    GetClusterHealthRequest, ClusterHealthData,
    GetClusterLayoutRequest, ClusterLayoutData,
    GetLayoutHistoryRequest, LayoutHistoryData,
    PreviewLayoutChangesRequest, ApplyLayoutResultData,
    ConnectNodesRequest, ConnectNodesData, ConnectNodeResult,
    UpdateLayoutRequest, ApplyLayoutRequest, RevertLayoutRequest,
    SkipDeadNodesRequest, SkipDeadNodesData,
    LayoutRole, StagedRoleChange, LayoutParameters, ZoneRedundancy,
    zone_redundancy, LayoutVersion, UpdateTracker, NodeUpdateProgress,
};

/// gRPC service for cluster operations
pub struct ClusterGrpcService {
    // Command handlers
    connect_nodes_handler: Arc<ConnectNodesHandler>,
    update_layout_handler: Arc<UpdateLayoutHandler>,
    apply_layout_handler: Arc<ApplyLayoutHandler>,
    revert_layout_handler: Arc<RevertLayoutHandler>,
    skip_dead_nodes_handler: Arc<SkipDeadNodesHandler>,
    // Query handlers
    get_cluster_status_handler: Arc<GetClusterStatusHandler>,
    get_cluster_health_handler: Arc<GetClusterHealthHandler>,
    get_cluster_layout_handler: Arc<GetClusterLayoutHandler>,
    get_layout_history_handler: Arc<GetLayoutHistoryHandler>,
    preview_layout_changes_handler: Arc<PreviewLayoutChangesHandler>,
}

impl ClusterGrpcService {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        connect_nodes_handler: Arc<ConnectNodesHandler>,
        update_layout_handler: Arc<UpdateLayoutHandler>,
        apply_layout_handler: Arc<ApplyLayoutHandler>,
        revert_layout_handler: Arc<RevertLayoutHandler>,
        skip_dead_nodes_handler: Arc<SkipDeadNodesHandler>,
        get_cluster_status_handler: Arc<GetClusterStatusHandler>,
        get_cluster_health_handler: Arc<GetClusterHealthHandler>,
        get_cluster_layout_handler: Arc<GetClusterLayoutHandler>,
        get_layout_history_handler: Arc<GetLayoutHistoryHandler>,
        preview_layout_changes_handler: Arc<PreviewLayoutChangesHandler>,
    ) -> Self {
        Self {
            connect_nodes_handler,
            update_layout_handler,
            apply_layout_handler,
            revert_layout_handler,
            skip_dead_nodes_handler,
            get_cluster_status_handler,
            get_cluster_health_handler,
            get_cluster_layout_handler,
            get_layout_history_handler,
            preview_layout_changes_handler,
        }
    }
}

#[tonic::async_trait]
impl ClusterService for ClusterGrpcService {
    async fn get_cluster_status(
        &self,
        _request: Request<GetClusterStatusRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let log = grpc_log!("ClusterService", "GetClusterStatus", &EmptyRequest {});
        let trace_id = get_trace_id();
        
        let status = self
            .get_cluster_status_handler
            .handle(GetClusterStatusQuery)
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let nodes: Vec<ClusterNode> = status
            .nodes
            .into_iter()
            .map(|n| ClusterNode {
                id: n.id,
                role: n.role.as_ref().map(|r| r.zone.clone()).unwrap_or_default(),
                addr: n.addr,
                hostname: n.hostname.unwrap_or_default(),
                is_up: n.is_up,
                last_seen_secs_ago: n.last_seen_secs_ago.unwrap_or(0),
                cluster_layout_current: true,
                cluster_layout_staging: None,
            })
            .collect();

        let response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::ClusterStatus(ClusterStatusData {
                layout_version: status.layout_version,
                nodes: nodes.clone(),
            })),
        };

        log.ok(&ApiResponseLog {
            trace_id: &trace_id,
            data: ClusterStatusLogSimple {
                layout_version: status.layout_version,
                node_count: nodes.len(),
            },
        });
        Ok(Response::new(response))
    }

    async fn get_cluster_health(
        &self,
        _request: Request<GetClusterHealthRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let log = grpc_log!("ClusterService", "GetClusterHealth", &EmptyRequest {});
        let trace_id = get_trace_id();
        
        let health = self
            .get_cluster_health_handler
            .handle(GetClusterHealthQuery)
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::ClusterHealth(ClusterHealthData {
                status: health.status.clone(),
                known_nodes: health.known_nodes,
                connected_nodes: health.connected_nodes,
                storage_nodes: health.storage_nodes,
                storage_nodes_ok: health.storage_nodes_up,
                partitions: health.partitions,
                partitions_quorum: health.partitions_quorum,
                partitions_all_ok: health.partitions_all_ok,
            })),
        };

        log.ok(&ApiResponseLog {
            trace_id: &trace_id,
            data: ClusterHealthLogSimple {
                status: &health.status,
                connected_nodes: health.connected_nodes,
            },
        });
        Ok(Response::new(response))
    }

    async fn get_cluster_layout(
        &self,
        _request: Request<GetClusterLayoutRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let log = grpc_log!("ClusterService", "GetClusterLayout", &EmptyRequest {});
        let trace_id = get_trace_id();
        
        let layout = self
            .get_cluster_layout_handler
            .handle(GetClusterLayoutQuery)
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let layout_data = convert_layout_response(&layout);
        let response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::ClusterLayout(layout_data)),
        };

        log.ok(&ApiResponseLog {
            trace_id: &trace_id,
            data: LayoutLogSimple {
                version: layout.version,
            },
        });
        Ok(Response::new(response))
    }

    async fn get_layout_history(
        &self,
        _request: Request<GetLayoutHistoryRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let log = grpc_log!("ClusterService", "GetLayoutHistory", &EmptyRequest {});
        let trace_id = get_trace_id();
        
        let history = self
            .get_layout_history_handler
            .handle(GetLayoutHistoryQuery)
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let versions: Vec<LayoutVersion> = history
            .versions
            .iter()
            .map(|v| LayoutVersion {
                version: v.version,
                update_time: String::new(),
            })
            .collect();

        let update_tracker = if !history.update_trackers.is_empty() {
            let nodes: Vec<NodeUpdateProgress> = history
                .update_trackers
                .iter()
                .map(|(node_id, tracker)| NodeUpdateProgress {
                    node_id: node_id.clone(),
                    sync_version: Some(tracker.sync),
                    write_version: Some(tracker.ack),
                })
                .collect();
            Some(UpdateTracker { nodes })
        } else {
            None
        };

        let response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::LayoutHistory(LayoutHistoryData {
                versions: versions.clone(),
                update_tracker,
            })),
        };

        log.ok(&ApiResponseLog {
            trace_id: &trace_id,
            data: HistoryLogSimple {
                version_count: versions.len(),
            },
        });
        Ok(Response::new(response))
    }

    async fn preview_layout_changes(
        &self,
        _request: Request<PreviewLayoutChangesRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let log = grpc_log!("ClusterService", "PreviewLayoutChanges", &EmptyRequest {});
        let trace_id = get_trace_id();
        
        let result = self
            .preview_layout_changes_handler
            .handle(PreviewLayoutChangesQuery)
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::PreviewLayout(ApplyLayoutResultData {
                current_layout_version: result.layout.version,
                partition_info: vec![],
            })),
        };

        log.ok(&ApiResponseLog {
            trace_id: &trace_id,
            data: LayoutLogSimple {
                version: result.layout.version,
            },
        });
        Ok(Response::new(response))
    }

    async fn connect_nodes(
        &self,
        request: Request<ConnectNodesRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("ClusterService", "ConnectNodes", &ConnectNodesReq { count: req.node_addresses.len() });
        let trace_id = get_trace_id();
        
        let results = self
            .connect_nodes_handler
            .handle(ConnectNodesCommand::new(req.node_addresses))
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let connect_results: Vec<ConnectNodeResult> = results
            .iter()
            .map(|r| ConnectNodeResult {
                success: r.success,
                error: r.error.clone(),
            })
            .collect();

        let response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::ConnectNodes(ConnectNodesData {
                results: connect_results.clone(),
            })),
        };

        let success_count = connect_results.iter().filter(|r| r.success).count();
        log.ok(&ApiResponseLog {
            trace_id: &trace_id,
            data: ConnectResultLog {
                total: connect_results.len(),
                success: success_count,
            },
        });
        Ok(Response::new(response))
    }

    async fn update_layout(
        &self,
        request: Request<UpdateLayoutRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("ClusterService", "UpdateLayout", &UpdateLayoutReq { changes: req.role_changes.len() });
        let trace_id = get_trace_id();
        
        let role_changes: Vec<LayoutRoleChange> = req
            .role_changes
            .into_iter()
            .map(|rc| LayoutRoleChange {
                node_id: rc.node_id,
                zone: rc.zone,
                capacity: rc.capacity,
                tags: if rc.tags.is_empty() { None } else { Some(rc.tags) },
                remove: rc.remove,
            })
            .collect();

        let layout = self
            .update_layout_handler
            .handle(UpdateLayoutCommand::new(role_changes))
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let layout_data = convert_layout_response(&layout);
        let response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::UpdateLayout(layout_data)),
        };

        log.ok(&ApiResponseLog {
            trace_id: &trace_id,
            data: LayoutLogSimple {
                version: layout.version,
            },
        });
        Ok(Response::new(response))
    }

    async fn apply_layout(
        &self,
        request: Request<ApplyLayoutRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("ClusterService", "ApplyLayout", &VersionReq { version: req.version });
        let trace_id = get_trace_id();
        
        let result = self
            .apply_layout_handler
            .handle(ApplyLayoutCommand::new(req.version))
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::ApplyLayout(ApplyLayoutResultData {
                current_layout_version: result.layout.version,
                partition_info: vec![],
            })),
        };

        log.ok(&ApiResponseLog {
            trace_id: &trace_id,
            data: LayoutLogSimple {
                version: result.layout.version,
            },
        });
        Ok(Response::new(response))
    }

    async fn revert_layout(
        &self,
        _request: Request<RevertLayoutRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let log = grpc_log!("ClusterService", "RevertLayout", &EmptyRequest {});
        let trace_id = get_trace_id();
        
        let layout = self
            .revert_layout_handler
            .handle(RevertLayoutCommand::new())
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let layout_data = convert_layout_response(&layout);
        let response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::RevertLayout(layout_data)),
        };

        log.ok(&ApiResponseLog {
            trace_id: &trace_id,
            data: LayoutLogSimple {
                version: layout.version,
            },
        });
        Ok(Response::new(response))
    }

    async fn skip_dead_nodes(
        &self,
        request: Request<SkipDeadNodesRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        let req = request.into_inner();
        let log = grpc_log!("ClusterService", "SkipDeadNodes", &SkipDeadNodesReq { version: req.version, allow_missing: req.allow_missing_data });
        let trace_id = get_trace_id();
        
        let result = self
            .skip_dead_nodes_handler
            .handle(SkipDeadNodesCommand::new(req.version, req.allow_missing_data))
            .await
            .map_err(|e| {
                log.err(&e.to_string());
                domain_error_to_status(e)
            })?;

        let response = ApiResponse {
            trace_id: trace_id.clone(),
            data: Some(Data::SkipDeadNodes(SkipDeadNodesData {
                current_layout_version: req.version,
                partition_info: vec![],
            })),
        };

        log.ok(&ApiResponseLog {
            trace_id: &trace_id,
            data: SkipDeadNodesLogSimple {
                ack_updated: result.ack_updated.len(),
                sync_updated: result.sync_updated.len(),
            },
        });
        Ok(Response::new(response))
    }
}

// ============ Log Structs ============

#[derive(Serialize)]
struct EmptyRequest {}

#[derive(Serialize)]
struct VersionReq {
    version: i64,
}

#[derive(Serialize)]
struct ConnectNodesReq {
    count: usize,
}

#[derive(Serialize)]
struct UpdateLayoutReq {
    changes: usize,
}

#[derive(Serialize)]
struct SkipDeadNodesReq {
    version: i64,
    allow_missing: bool,
}

#[derive(Serialize)]
struct ApiResponseLog<'a, T: Serialize> {
    trace_id: &'a str,
    data: T,
}

#[derive(Serialize)]
struct ClusterStatusLogSimple {
    layout_version: i64,
    node_count: usize,
}

#[derive(Serialize)]
struct ClusterHealthLogSimple<'a> {
    status: &'a str,
    connected_nodes: i32,
}

#[derive(Serialize)]
struct LayoutLogSimple {
    version: i64,
}

#[derive(Serialize)]
struct HistoryLogSimple {
    version_count: usize,
}

#[derive(Serialize)]
struct ConnectResultLog {
    total: usize,
    success: usize,
}

#[derive(Serialize)]
struct SkipDeadNodesLogSimple {
    ack_updated: usize,
    sync_updated: usize,
}

// ============ Helpers ============

fn convert_layout_response(layout: &crate::domain::entities::ClusterLayout) -> ClusterLayoutData {
    let roles: Vec<LayoutRole> = layout
        .roles
        .iter()
        .map(|r| LayoutRole {
            id: r.id.clone(),
            zone: r.zone.clone(),
            capacity: r.capacity.unwrap_or(0),
            tags: r.tags.clone(),
        })
        .collect();

    let staged_changes: Vec<StagedRoleChange> = layout
        .staged_role_changes
        .iter()
        .map(|s| {
            use crate::domain::entities::RoleChangeType;
            match &s.change {
                RoleChangeType::Remove { remove: _ } => StagedRoleChange {
                    id: s.id.clone(),
                    role: None,
                    remove: true,
                },
                RoleChangeType::Assign { zone, capacity, tags } => StagedRoleChange {
                    id: s.id.clone(),
                    role: Some(LayoutRole {
                        id: s.id.clone(),
                        zone: zone.clone(),
                        capacity: capacity.unwrap_or(0),
                        tags: tags.clone(),
                    }),
                    remove: false,
                },
            }
        })
        .collect();

    let zone_redundancy = layout.parameters.as_ref().and_then(|p| p.zone_redundancy.as_ref()).map(|zr| {
        use crate::domain::entities::ZoneRedundancy as DomainZoneRedundancy;
        match zr {
            DomainZoneRedundancy::Maximum { maximum: _ } => ZoneRedundancy {
                value: Some(zone_redundancy::Value::Maximum(true)),
            },
            DomainZoneRedundancy::Value(v) => ZoneRedundancy {
                value: Some(zone_redundancy::Value::Fixed(*v)),
            },
        }
    });

    ClusterLayoutData {
        version: layout.version,
        roles,
        staged_role_changes: staged_changes,
        parameters: Some(LayoutParameters {
            zone_redundancy,
        }),
    }
}
