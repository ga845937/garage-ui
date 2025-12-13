//! Request context for tracing
//!
//! Provides a shared context across async operations within a single request

use std::future::Future;
use tokio::task_local;

use crate::shared::trace_id::generate_trace_id;

task_local! {
    static TRACE_CONTEXT: TraceContext;
}

/// Context shared across a single request
#[derive(Clone, Debug)]
pub struct TraceContext {
    pub trace_id: String,
}

impl TraceContext {
    pub fn new() -> Self {
        Self {
            trace_id: generate_trace_id(),
        }
    }

    pub fn with_trace_id(trace_id: String) -> Self {
        Self { trace_id }
    }
}

impl Default for TraceContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Run an async block with a trace context
pub async fn with_context<F, T>(ctx: TraceContext, f: F) -> T
where
    F: Future<Output = T>,
{
    TRACE_CONTEXT.scope(ctx, f).await
}

/// Get the current trace_id, or generate a new one if not in context
pub fn get_trace_id() -> String {
    TRACE_CONTEXT
        .try_with(|ctx| ctx.trace_id.clone())
        .unwrap_or_else(|_| generate_trace_id())
}

/// Check if we're currently in a trace context
pub fn has_context() -> bool {
    TRACE_CONTEXT.try_with(|_| ()).is_ok()
}
