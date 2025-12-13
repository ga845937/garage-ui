//! Garage API HTTP client

use reqwest::{Client, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::time::{Duration, Instant};
use tracing::{info, error};
use crate::domain::errors::DomainError;
use crate::shared::get_trace_id;

// Re-export from endpoints module
pub use super::endpoints::GarageApiEndpoint;

/// Garage API client
#[derive(Clone)]
pub struct GarageClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl GarageClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(5))
            .build()
            .unwrap_or_else(|_| Client::new());
            
        Self {
            client,
            base_url,
            api_key,
        }
    }

    /// Truncate response for logging (max 500 chars)
    fn truncate_response(response: &str) -> String {
        if response.len() > 500 {
            format!("{}...(truncated)", &response[..500])
        } else {
            response.to_string()
        }
    }

    /// Log API call result
    /// 格式: timestamp [api] trace_id | [method] uri | request | [status] response | duration_ms
    fn log_api_call(&self, method: &str, uri: &str, status: u16, duration_ms: u128, request_body: Option<&str>, response_body: &str) {
        let trace_id = get_trace_id();
        let request = Self::truncate_response(request_body.unwrap_or("{}"));
        let response = Self::truncate_response(response_body);
        info!(
            target: "api",
            trace_id = %trace_id,
            method = %method,
            uri = %uri,
            http_status = %status,
            duration_ms = %duration_ms,
            request = %request,
            response = %response,
            "[api] {} | [{}] {} | {} | [{}] {} | {}ms",
            trace_id,
            method,
            uri,
            request,
            status,
            response,
            duration_ms
        );
    }

    /// Log API call error
    fn log_api_error(&self, method: &str, uri: &str, error: &str, duration_ms: u128, request_body: Option<&str>) {
        let trace_id = get_trace_id();
        let request = Self::truncate_response(request_body.unwrap_or("{}"));
        error!(
            target: "api",
            trace_id = %trace_id,
            method = %method,
            uri = %uri,
            error = %error,
            duration_ms = %duration_ms,
            request = %request,
            "[api] {} | [{}] {} | {} | [ERROR] {} | {}ms",
            trace_id,
            method,
            uri,
            request,
            error,
            duration_ms
        );
    }

    /// Make a GET request
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, DomainError> {
        let url = format!("{}{}", self.base_url, path);
        let start = Instant::now();
        
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| {
                self.log_api_error("GET", &url, &e.to_string(), start.elapsed().as_millis(), None);
                DomainError::GarageApiError(e.to_string())
            })?;

        let status = response.status();
        let duration_ms = start.elapsed().as_millis();
        
        // Read response body as text first for logging
        let body_text = response.text().await.unwrap_or_default();
        self.log_api_call("GET", &url, status.as_u16(), duration_ms, None, &body_text);
        
        self.parse_response(status, &body_text)
    }

    /// Make a POST request
    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, DomainError> {
        let url = format!("{}{}", self.base_url, path);
        let start = Instant::now();
        let request_json = serde_json::to_string(body).unwrap_or_default();
        
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(body)
            .send()
            .await
            .map_err(|e| {
                self.log_api_error("POST", &url, &e.to_string(), start.elapsed().as_millis(), Some(&request_json));
                DomainError::GarageApiError(e.to_string())
            })?;

        let status = response.status();
        let duration_ms = start.elapsed().as_millis();
        
        let body_text = response.text().await.unwrap_or_default();
        self.log_api_call("POST", &url, status.as_u16(), duration_ms, Some(&request_json), &body_text);
        
        self.parse_response(status, &body_text)
    }

    /// Make a POST request without body (for v2 API endpoints like DeleteBucket)
    pub async fn post_empty(&self, path: &str) -> Result<(), DomainError> {
        let url = format!("{}{}", self.base_url, path);
        let start = Instant::now();
        
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| {
                self.log_api_error("POST", &url, &e.to_string(), start.elapsed().as_millis(), None);
                DomainError::GarageApiError(e.to_string())
            })?;

        let status = response.status();
        let duration_ms = start.elapsed().as_millis();
        let body_text = response.text().await.unwrap_or_default();
        self.log_api_call("POST", &url, status.as_u16(), duration_ms, None, &body_text);

        match status {
            StatusCode::OK | StatusCode::NO_CONTENT => Ok(()),
            StatusCode::NOT_FOUND => Err(DomainError::BucketNotFound("Bucket not found".to_string())),
            StatusCode::BAD_REQUEST => Err(DomainError::GarageApiError(format!("Bad request: {}", body_text))),
            _ => Err(DomainError::GarageApiError(format!(
                "API error {}: {}",
                status, body_text
            ))),
        }
    }

    /// Make a POST request without body but with response (for v2 API endpoints like RevertClusterLayout)
    pub async fn post_with_empty_body<T: DeserializeOwned>(&self, path: &str) -> Result<T, DomainError> {
        let url = format!("{}{}", self.base_url, path);
        let start = Instant::now();
        
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| {
                self.log_api_error("POST", &url, &e.to_string(), start.elapsed().as_millis(), None);
                DomainError::GarageApiError(e.to_string())
            })?;

        let status = response.status();
        let duration_ms = start.elapsed().as_millis();
        let body_text = response.text().await.unwrap_or_default();
        self.log_api_call("POST", &url, status.as_u16(), duration_ms, None, &body_text);

        self.parse_response(status, &body_text)
    }

    /// Make a PUT request
    pub async fn put<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, DomainError> {
        let url = format!("{}{}", self.base_url, path);
        let start = Instant::now();
        let request_json = serde_json::to_string(body).unwrap_or_default();
        
        let response = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(body)
            .send()
            .await
            .map_err(|e| {
                self.log_api_error("PUT", &url, &e.to_string(), start.elapsed().as_millis(), Some(&request_json));
                DomainError::GarageApiError(e.to_string())
            })?;

        let status = response.status();
        let duration_ms = start.elapsed().as_millis();
        
        let body_text = response.text().await.unwrap_or_default();
        self.log_api_call("PUT", &url, status.as_u16(), duration_ms, Some(&request_json), &body_text);
        
        self.parse_response(status, &body_text)
    }

    /// Make a DELETE request
    pub async fn delete(&self, path: &str) -> Result<(), DomainError> {
        let url = format!("{}{}", self.base_url, path);
        let start = Instant::now();
        
        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| {
                self.log_api_error("DELETE", &url, &e.to_string(), start.elapsed().as_millis(), None);
                DomainError::GarageApiError(e.to_string())
            })?;

        let status = response.status();
        let duration_ms = start.elapsed().as_millis();
        let body_text = response.text().await.unwrap_or_default();
        self.log_api_call("DELETE", &url, status.as_u16(), duration_ms, None, &body_text);

        match status {
            StatusCode::OK | StatusCode::NO_CONTENT => Ok(()),
            StatusCode::NOT_FOUND => Err(DomainError::BucketNotFound("Bucket not found".to_string())),
            _ => Err(DomainError::GarageApiError(format!(
                "API error {}: {}",
                status, body_text
            ))),
        }
    }

    /// Parse response from text
    fn parse_response<T: DeserializeOwned>(
        &self,
        status: StatusCode,
        body: &str,
    ) -> Result<T, DomainError> {
        match status {
            StatusCode::OK | StatusCode::CREATED => {
                serde_json::from_str::<T>(body)
                    .map_err(|e| DomainError::GarageApiError(format!("Failed to parse response: {}", e)))
            }
            StatusCode::NOT_FOUND => {
                Err(DomainError::BucketNotFound("Resource not found".to_string()))
            }
            StatusCode::BAD_REQUEST => {
                // Parse error response to identify specific errors
                if body.contains("Local alias already exists") {
                    Err(DomainError::LocalAliasAlreadyExists(
                        "The local alias is already in use".to_string()
                    ))
                } else {
                    Err(DomainError::GarageApiError(format!(
                        "Bad request: {}",
                        body
                    )))
                }
            }
            _ => {
                Err(DomainError::GarageApiError(format!(
                    "API error {}: {}",
                    status, body
                )))
            }
        }
    }
}

