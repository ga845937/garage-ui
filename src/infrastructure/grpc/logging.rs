//! gRPC logging utilities
//!
//! Provides clean logging for gRPC requests and responses

use std::panic::Location;
use std::time::Instant;
use tracing::{info, error};
use serde::Serialize;
use crate::shared::get_trace_id;

/// Truncate string for logging (max 500 chars)
fn truncate_for_log(s: &str) -> String {
    if s.len() > 500 {
        format!("{}...(truncated)", &s[..500])
    } else {
        s.to_string()
    }
}

/// gRPC call logger - handles timing and logging automatically
pub struct GrpcLogger {
    proto: &'static str,
    method: &'static str,
    start: Instant,
    request_json: String,
}

impl GrpcLogger {
    pub fn new<R: Serialize>(proto: &'static str, method: &'static str, request: &R) -> Self {
        Self {
            proto,
            method,
            start: Instant::now(),
            request_json: serde_json::to_string(request).unwrap_or_else(|_| "{}".to_string()),
        }
    }

    pub fn ok<T: Serialize>(&self, response: &T) {
        let trace_id = get_trace_id();
        let req = truncate_for_log(&self.request_json);
        let res = truncate_for_log(&serde_json::to_string(response).unwrap_or_else(|_| "{}".to_string()));
        let duration_ms = self.start.elapsed().as_millis();
        
        info!(
            target: "grpc",
            trace_id = %trace_id,
            proto = %self.proto,
            method = %self.method,
            request = %req,
            response = %res,
            duration_ms = %duration_ms,
            "[grpc] {} | {}/{} | {} | {} | {}ms",
            trace_id,
            self.proto,
            self.method,
            req,
            res,
            duration_ms
        );
    }

    /// Log error response with caller location
    #[track_caller]
    pub fn err(&self, error: &str) {
        let trace_id = get_trace_id();
        let req = truncate_for_log(&self.request_json);
        let duration_ms = self.start.elapsed().as_millis();
        let location = Location::caller();
        
        error!(
            target: "error",
            trace_id = %trace_id,
            proto = %self.proto,
            method = %self.method,
            request = %req,
            error = %error,
            duration_ms = %duration_ms,
            file = %location.file(),
            line = %location.line(),
            "[error] {} | {}/{} | {} | ERROR: {} | {}ms | at {}:{}",
            trace_id,
            self.proto,
            self.method,
            req,
            error,
            duration_ms,
            location.file(),
            location.line()
        );
    }

    /// Log error with full backtrace (for debugging)
    #[track_caller]
    pub fn err_with_backtrace(&self, error: &str) {
        let trace_id = get_trace_id();
        let req = truncate_for_log(&self.request_json);
        let duration_ms = self.start.elapsed().as_millis();
        let location = Location::caller();
        let backtrace = std::backtrace::Backtrace::capture();
        
        error!(
            target: "error",
            trace_id = %trace_id,
            proto = %self.proto,
            method = %self.method,
            request = %req,
            error = %error,
            duration_ms = %duration_ms,
            file = %location.file(),
            line = %location.line(),
            backtrace = %backtrace,
            "[error] {} | {}/{} | {} | ERROR: {} | {}ms | at {}:{}\nBacktrace:\n{}",
            trace_id,
            self.proto,
            self.method,
            req,
            error,
            duration_ms,
            location.file(),
            location.line(),
            backtrace
        );
    }
}

/// Macro for easy gRPC logging
/// Usage: grpc_log!("BucketService", "ListBuckets", &request)
#[macro_export]
macro_rules! grpc_log {
    ($proto:expr, $method:expr, $request:expr) => {
        $crate::infrastructure::grpc::logging::GrpcLogger::new($proto, $method, $request)
    };
}
