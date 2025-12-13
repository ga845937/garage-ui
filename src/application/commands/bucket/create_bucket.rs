//! Create bucket command

use crate::domain::entities::WebsiteConfig;
use crate::domain::value_objects::Quotas;
use crate::domain::errors::DomainError;

/// Command to create a bucket
#[derive(Debug, Clone)]
pub struct CreateBucketCommand {
    global_alias: Option<String>,
    local_alias: Option<CreateLocalAliasCommand>,
    quotas: Option<Quotas>,
    website_config: Option<WebsiteConfig>,
}

impl CreateBucketCommand {
    pub fn new(
        global_alias: Option<String>,
        local_alias: Option<CreateLocalAliasCommand>,
        quotas: Option<Quotas>,
        website_config: Option<WebsiteConfig>,
    ) -> Self {
        Self {
            global_alias,
            local_alias,
            quotas,
            website_config,
        }
    }

    pub fn global_alias(&self) -> Option<&String> {
        self.global_alias.as_ref()
    }

    pub fn local_alias(&self) -> Option<&CreateLocalAliasCommand> {
        self.local_alias.as_ref()
    }

    pub fn quotas(&self) -> Option<&Quotas> {
        self.quotas.as_ref()
    }

    pub fn website_config(&self) -> Option<&WebsiteConfig> {
        self.website_config.as_ref()
    }

    /// 驗證 Command 輸入資料
    pub fn validate(&self) -> Result<(), DomainError> {
        // 驗證至少有一個 alias
        if self.global_alias.is_none() && self.local_alias.is_none() {
            return Err(DomainError::ValidationError(
                "At least one alias (global or local) must be provided".to_string(),
            ));
        }

        Ok(())
    }
}

/// Command to create a local alias
#[derive(Debug, Clone)]
pub struct CreateLocalAliasCommand {
    pub access_key_id: String,
    pub alias: String,
    pub allow_read: bool,
    pub allow_write: bool,
    pub allow_owner: bool,
}
