//! Shared utilities
//!
//! 跨層共用的工具模組，不屬於特定 DDD 層級
//! 
//! - `pagination`: 分頁工具
//! - `datetime`: 日期時間解析
//! - `update_field`: 更新欄位三態語義
//! - `trace_id`: 請求追蹤 ID 生成
//! - `context`: 請求上下文

mod context;
mod datetime;
mod pagination;
mod trace_id;
mod update_field;

pub use context::{get_trace_id, has_context, with_context, TraceContext};
pub use datetime::parse_datetime;
pub use pagination::{paginate, PaginationResult};
pub use trace_id::{generate_trace_id, parse_trace_id_time, trace_id_to_time_string};
pub use update_field::UpdateField;
