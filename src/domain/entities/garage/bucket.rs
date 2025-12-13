use serde::{ Deserialize, Serialize };

// Garage API response types
#[derive(Debug, Deserialize)]
pub struct GarageBucketListResponse(pub Vec<GarageBucketInfo>);

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GarageBucketInfo {
    pub id: String,
    #[serde(default)]
    pub global_aliases: Vec<String>,
    #[serde(default)]
    pub local_aliases: Vec<GarageLocalAlias>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GarageLocalAlias {
    pub access_key_id: String,
    pub alias: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GarageBucketDetailResponse {
    pub id: String,
    #[serde(default)]
    pub global_aliases: Vec<String>,
    #[serde(default)]
    pub local_aliases: Vec<GarageLocalAlias>,
    #[serde(default)]
    pub website_access: bool,
    pub website_config: Option<GarageWebsiteConfig>,
    #[serde(default)]
    pub keys: Vec<GarageBucketKey>,
    #[serde(default)]
    pub quotas: GarageQuotas,
    #[serde(default)]
    pub bytes: u64,
    #[serde(default)]
    pub objects: u64,
    #[serde(default)]
    pub created: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GarageWebsiteConfig {
    pub index_document: String,
    pub error_document: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GarageBucketKey {
    pub access_key_id: String,
    pub name: String,
    pub permissions: GarageBucketKeyPermissions,
    #[serde(default)]
    pub bucket_local_aliases: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct GarageBucketKeyPermissions {
    pub read: bool,
    pub write: bool,
    pub owner: bool,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GarageQuotas {
    pub max_size: Option<i64>,
    pub max_objects: Option<i64>,
}

// Request types
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBucketRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_alias: Option<CreateLocalAliasRequest>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLocalAliasRequest {
    pub access_key_id: String,
    pub alias: String,
    pub allow: CreateLocalAliasAllow,
}

#[derive(Debug, Serialize)]
pub struct CreateLocalAliasAllow {
    pub read: bool,
    pub write: bool,
    pub owner: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBucketRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_access: Option<UpdateWebsiteAccess>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotas: Option<UpdateQuotas>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWebsiteAccess {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_document: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_document: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateQuotas {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_objects: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBucketResponse {
    pub id: String,
}
