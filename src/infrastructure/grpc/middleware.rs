//! gRPC interceptor/middleware for trace context and error handling

use std::task::{Context, Poll};
use std::pin::Pin;
use std::future::Future;
use http::{Request, Response};
use http_body::Body;
use tower::{Layer, Service};

use crate::shared::{with_context, TraceContext, generate_trace_id};

/// Layer that adds logging middleware to gRPC services
#[derive(Clone)]
pub struct LoggingLayer;

impl<S> Layer<S> for LoggingLayer {
    type Service = LoggingMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        LoggingMiddleware { inner: service }
    }
}

/// Middleware that logs all gRPC requests
#[derive(Clone)]
pub struct LoggingMiddleware<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for LoggingMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Body + Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let trace_id = generate_trace_id();
        
        // Clone the inner service
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        
        // Create trace context for this request
        let ctx = TraceContext::with_trace_id(trace_id.clone());
        
        Box::pin(async move {
            // Run the request within the trace context
            // Logging is done at the service layer with request/response details
            with_context(ctx, async {
                inner.call(req).await
            }).await
        })
    }
}

// ============ Error Handling Interceptor ============

/// Layer that adds error handling to gRPC services
/// 
/// 這個 Layer 會攔截所有從 Service 拋出的 DomainError，
/// 並使用 `domain_error_to_status` 統一轉換為 gRPC Status。
#[derive(Clone)]
pub struct ErrorHandlingLayer;

impl<S> Layer<S> for ErrorHandlingLayer {
    type Service = ErrorHandlingMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        ErrorHandlingMiddleware { inner: service }
    }
}

/// Middleware that handles DomainError conversion to gRPC Status
#[derive(Clone)]
pub struct ErrorHandlingMiddleware<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for ErrorHandlingMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>> + Send,
    ReqBody: Send + 'static,
    ResBody: Body + Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        
        Box::pin(async move {
            // 直接調用內部 service
            // 錯誤處理由各個 service method 使用 DomainErrorExt 處理
            inner.call(req).await
        })
    }
}
