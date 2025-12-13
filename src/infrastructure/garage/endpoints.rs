//! Garage Admin API Endpoints
//! 
//! 所有 Garage Admin API v2 端點的定義

/// Garage API v2 端點
#[derive(Debug, Clone, Copy)]
pub enum GarageApiEndpoint {
    // ============ Special Endpoints ============
    /// 檢查靜態網站域名
    CheckDomain,
    /// 健康檢查
    Health,
    /// Prometheus 指標
    Metrics,
    
    // ============ Bucket ============
    /// 列出所有 buckets
    ListBuckets,
    /// 獲取 bucket 資訊
    GetBucketInfo,
    /// 創建 bucket
    CreateBucket,
    /// 更新 bucket
    UpdateBucket,
    /// 刪除 bucket
    DeleteBucket,
    /// 清理未完成的上傳
    CleanupIncompleteUploads,
    /// 檢查對象
    InspectObject,
    
    // ============ Bucket Alias ============
    /// 添加 bucket 別名
    AddBucketAlias,
    /// 移除 bucket 別名
    RemoveBucketAlias,
    
    // ============ Permission ============
    /// 允許 key 訪問 bucket
    AllowBucketKey,
    /// 拒絕 key 訪問 bucket
    DenyBucketKey,
    
    // ============ Access Key ============
    /// 列出所有 access keys
    ListKeys,
    /// 獲取 key 資訊
    GetKeyInfo,
    /// 創建 key
    CreateKey,
    /// 更新 key
    UpdateKey,
    /// 刪除 key
    DeleteKey,
    /// 導入 key
    ImportKey,
    
    // ============ Cluster ============
    /// 獲取集群狀態
    GetClusterStatus,
    /// 獲取集群健康狀態
    GetClusterHealth,
    /// 獲取集群統計資訊
    GetClusterStatistics,
    /// 連接集群節點
    ConnectClusterNodes,
    
    // ============ Cluster Layout ============
    /// 獲取集群布局
    GetClusterLayout,
    /// 更新集群布局
    UpdateClusterLayout,
    /// 應用集群布局
    ApplyClusterLayout,
    /// 還原集群布局
    RevertClusterLayout,
    /// 預覽集群布局變更
    PreviewClusterLayoutChanges,
    /// 獲取集群布局歷史
    GetClusterLayoutHistory,
    /// 跳過死節點
    ClusterLayoutSkipDeadNodes,
    
    // ============ Admin Token ============
    /// 列出所有 admin tokens
    ListAdminTokens,
    /// 獲取 admin token 資訊
    GetAdminTokenInfo,
    /// 獲取當前 admin token 資訊
    GetCurrentAdminTokenInfo,
    /// 創建 admin token
    CreateAdminToken,
    /// 更新 admin token
    UpdateAdminToken,
    /// 刪除 admin token
    DeleteAdminToken,
    
    // ============ Node ============
    /// 獲取節點資訊
    GetNodeInfo,
    /// 獲取節點統計
    GetNodeStatistics,
    /// 創建元數據快照
    CreateMetadataSnapshot,
    /// 啟動修復操作
    LaunchRepairOperation,
    
    // ============ Block ============
    /// 獲取區塊資訊
    GetBlockInfo,
    /// 列出區塊錯誤
    ListBlockErrors,
    /// 清除區塊
    PurgeBlocks,
    /// 重試區塊重同步
    RetryBlockResync,
    
    // ============ Worker ============
    /// 列出工作者
    ListWorkers,
    /// 獲取工作者資訊
    GetWorkerInfo,
    /// 獲取工作者變數
    GetWorkerVariable,
    /// 設置工作者變數
    SetWorkerVariable,
}

impl GarageApiEndpoint {
    /// 獲取 API 路徑
    pub fn path(&self) -> &'static str {
        match self {
            // Special Endpoints
            Self::CheckDomain => "/check",
            Self::Health => "/health",
            Self::Metrics => "/metrics",
            
            // Bucket
            Self::ListBuckets => "/v2/ListBuckets",
            Self::GetBucketInfo => "/v2/GetBucketInfo",
            Self::CreateBucket => "/v2/CreateBucket",
            Self::UpdateBucket => "/v2/UpdateBucket",
            Self::DeleteBucket => "/v2/DeleteBucket",
            Self::CleanupIncompleteUploads => "/v2/CleanupIncompleteUploads",
            Self::InspectObject => "/v2/InspectObject",
            
            // Bucket Alias
            Self::AddBucketAlias => "/v2/AddBucketAlias",
            Self::RemoveBucketAlias => "/v2/RemoveBucketAlias",
            
            // Permission
            Self::AllowBucketKey => "/v2/AllowBucketKey",
            Self::DenyBucketKey => "/v2/DenyBucketKey",
            
            // Access Key
            Self::ListKeys => "/v2/ListKeys",
            Self::GetKeyInfo => "/v2/GetKeyInfo",
            Self::CreateKey => "/v2/CreateKey",
            Self::UpdateKey => "/v2/UpdateKey",
            Self::DeleteKey => "/v2/DeleteKey",
            Self::ImportKey => "/v2/ImportKey",
            
            // Cluster
            Self::GetClusterStatus => "/v2/GetClusterStatus",
            Self::GetClusterHealth => "/v2/GetClusterHealth",
            Self::GetClusterStatistics => "/v2/GetClusterStatistics",
            Self::ConnectClusterNodes => "/v2/ConnectClusterNodes",
            
            // Cluster Layout
            Self::GetClusterLayout => "/v2/GetClusterLayout",
            Self::UpdateClusterLayout => "/v2/UpdateClusterLayout",
            Self::ApplyClusterLayout => "/v2/ApplyClusterLayout",
            Self::RevertClusterLayout => "/v2/RevertClusterLayout",
            Self::PreviewClusterLayoutChanges => "/v2/PreviewClusterLayoutChanges",
            Self::GetClusterLayoutHistory => "/v2/GetClusterLayoutHistory",
            Self::ClusterLayoutSkipDeadNodes => "/v2/ClusterLayoutSkipDeadNodes",
            
            // Admin Token
            Self::ListAdminTokens => "/v2/ListAdminTokens",
            Self::GetAdminTokenInfo => "/v2/GetAdminTokenInfo",
            Self::GetCurrentAdminTokenInfo => "/v2/GetCurrentAdminTokenInfo",
            Self::CreateAdminToken => "/v2/CreateAdminToken",
            Self::UpdateAdminToken => "/v2/UpdateAdminToken",
            Self::DeleteAdminToken => "/v2/DeleteAdminToken",
            
            // Node
            Self::GetNodeInfo => "/v2/GetNodeInfo",
            Self::GetNodeStatistics => "/v2/GetNodeStatistics",
            Self::CreateMetadataSnapshot => "/v2/CreateMetadataSnapshot",
            Self::LaunchRepairOperation => "/v2/LaunchRepairOperation",
            
            // Block
            Self::GetBlockInfo => "/v2/GetBlockInfo",
            Self::ListBlockErrors => "/v2/ListBlockErrors",
            Self::PurgeBlocks => "/v2/PurgeBlocks",
            Self::RetryBlockResync => "/v2/RetryBlockResync",
            
            // Worker
            Self::ListWorkers => "/v2/ListWorkers",
            Self::GetWorkerInfo => "/v2/GetWorkerInfo",
            Self::GetWorkerVariable => "/v2/GetWorkerVariable",
            Self::SetWorkerVariable => "/v2/SetWorkerVariable",
        }
    }
    
    /// 是否需要認證
    pub fn requires_auth(&self) -> bool {
        !matches!(self, Self::CheckDomain | Self::Health)
    }
}
