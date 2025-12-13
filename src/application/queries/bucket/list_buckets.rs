//! List buckets query

/// Query to list all buckets
/// 
/// Query 包含分頁條件
#[derive(Debug, Clone, Default)]
pub struct ListBucketsQuery {
    // 分頁
    pub page: i32,
    pub page_size: i32,
}

impl ListBucketsQuery {
    pub fn new(page: i32, page_size: i32) -> Self {
        Self { page, page_size }
    }

    /// 從 gRPC 請求建立 Query
    pub fn from_grpc_request(page: i32, page_size: i32) -> Self {
        Self::new(page, page_size)
    }
}
