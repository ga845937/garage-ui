//! Application configuration

use std::env;

/// Application configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub garage_api_url: String,
    pub garage_api_key: String,
    pub grpc_server_addr: String,
    pub log_dir: String,
    pub s3_config: S3Config,
}

/// S3 configuration for Garage S3-compatible API
#[derive(Debug, Clone)]
pub struct S3Config {
    /// S3 endpoint URL (e.g., http://localhost:3900)
    pub endpoint_url: String,
    /// S3 region (Garage uses 'garage' as default)
    pub region: String,
    /// S3 Access Key ID
    pub access_key_id: String,
    /// S3 Secret Access Key
    pub secret_access_key: String,
}

impl AppConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        let garage_api_url = env::var("GARAGE_API_URL")
            .map_err(|_| ConfigError::MissingEnvVar("GARAGE_API_URL".to_string()))?;
        
        let garage_api_key = env::var("GARAGE_API_KEY")
            .map_err(|_| ConfigError::MissingEnvVar("GARAGE_API_KEY".to_string()))?;
        
        let grpc_server_addr = env::var("GRPC_SERVER_ADDR")
            .unwrap_or_else(|_| "0.0.0.0:50051".to_string());

        let log_dir = "./logs".to_string();

        // S3 Configuration
        let s3_endpoint_url = env::var("S3_ENDPOINT_URL")
            .map_err(|_| ConfigError::MissingEnvVar("S3_ENDPOINT_URL".to_string()))?;
        
        let s3_region = env::var("S3_REGION")
            .unwrap_or_else(|_| "garage".to_string());
        
        let s3_access_key_id = env::var("S3_ACCESS_KEY_ID")
            .map_err(|_| ConfigError::MissingEnvVar("S3_ACCESS_KEY_ID".to_string()))?;
        
        let s3_secret_access_key = env::var("S3_SECRET_ACCESS_KEY")
            .map_err(|_| ConfigError::MissingEnvVar("S3_SECRET_ACCESS_KEY".to_string()))?;

        let s3_config = S3Config {
            endpoint_url: s3_endpoint_url,
            region: s3_region,
            access_key_id: s3_access_key_id,
            secret_access_key: s3_secret_access_key,
        };

        Ok(Self {
            garage_api_url,
            garage_api_key,
            grpc_server_addr,
            log_dir,
            s3_config,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),
}
