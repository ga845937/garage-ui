//! Shared types for bucket alias commands

/// Type of bucket alias (global or local)
#[derive(Debug, Clone)]
pub enum AliasType {
    Global(String),
    Local { access_key_id: String, alias: String },
}
