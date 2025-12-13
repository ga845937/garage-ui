//! gRPC type conversions
//!
//! 提供 proto Nullable 類型到 UpdateField 的轉換，以及 Domain Error 到 gRPC Status 的轉換

use crate::infrastructure::grpc::generated::utility::{NullableBool, NullableNumber, NullableString};
use crate::shared::UpdateField;
use crate::domain::errors::DomainError;
use tonic::Status;

// ============== NullableString ==============

/// Extension trait for converting `Option<NullableString>` to `UpdateField<String>`
/// 
/// 三態語義：
/// - `None` -> `UpdateField::NoChange` (欄位未傳送，不更新)
/// - `Some(NullableString { value: None })` -> `UpdateField::Clear` (清空欄位)
/// - `Some(NullableString { value: Some(s) })` -> `UpdateField::Set(s)` (設定新值)
pub trait NullableStringExt {
    fn into_update_field(self) -> UpdateField<String>;
}

impl NullableStringExt for Option<NullableString> {
    fn into_update_field(self) -> UpdateField<String> {
        match self {
            None => UpdateField::NoChange,
            Some(ns) => match ns.value {
                None => UpdateField::Clear,
                Some(s) if s.is_empty() => UpdateField::Clear,
                Some(s) => UpdateField::Set(s),
            }
        }
    }
}

// ============== NullableNumber ==============

/// Extension trait for converting `Option<NullableNumber>` to `UpdateField<i64>`
/// 
/// 三態語義：
/// - `None` -> `UpdateField::NoChange` (欄位未傳送，不更新)
/// - `Some(NullableNumber { value: None })` -> `UpdateField::Clear` (清空欄位)
/// - `Some(NullableNumber { value: Some(n) })` -> `UpdateField::Set(n)` (設定新值)
pub trait NullableNumberExt {
    fn into_update_field(self) -> UpdateField<i64>;
}

impl NullableNumberExt for Option<NullableNumber> {
    fn into_update_field(self) -> UpdateField<i64> {
        match self {
            None => UpdateField::NoChange,
            Some(nn) => match nn.value {
                None => UpdateField::Clear,
                Some(n) => UpdateField::Set(n),
            }
        }
    }
}

// ============== NullableBool ==============

/// Extension trait for converting `Option<NullableBool>` to `UpdateField<bool>`
/// 
/// 三態語義：
/// - `None` -> `UpdateField::NoChange` (欄位未傳送，不更新)
/// - `Some(NullableBool { value: None })` -> `UpdateField::Clear` (清空欄位)
/// - `Some(NullableBool { value: Some(b) })` -> `UpdateField::Set(b)` (設定新值)
pub trait NullableBoolExt {
    fn into_update_field(self) -> UpdateField<bool>;
}

impl NullableBoolExt for Option<NullableBool> {
    fn into_update_field(self) -> UpdateField<bool> {
        match self {
            None => UpdateField::NoChange,
            Some(nb) => match nb.value {
                None => UpdateField::Clear,
                Some(b) => UpdateField::Set(b),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============== NullableString Tests ==============

    #[test]
    fn test_string_none_is_no_change() {
        let input: Option<NullableString> = None;
        assert_eq!(input.into_update_field(), UpdateField::NoChange);
    }

    #[test]
    fn test_string_some_none_is_clear() {
        let input = Some(NullableString { value: None });
        assert_eq!(input.into_update_field(), UpdateField::Clear);
    }

    #[test]
    fn test_string_some_empty_is_clear() {
        let input = Some(NullableString { value: Some(String::new()) });
        assert_eq!(input.into_update_field(), UpdateField::Clear);
    }

    #[test]
    fn test_string_some_value_is_set() {
        let input = Some(NullableString { value: Some("2025-12-31".to_string()) });
        assert_eq!(input.into_update_field(), UpdateField::Set("2025-12-31".to_string()));
    }

    // ============== NullableNumber Tests ==============

    #[test]
    fn test_number_none_is_no_change() {
        let input: Option<NullableNumber> = None;
        assert_eq!(input.into_update_field(), UpdateField::NoChange);
    }

    #[test]
    fn test_number_some_none_is_clear() {
        let input = Some(NullableNumber { value: None });
        assert_eq!(input.into_update_field(), UpdateField::Clear);
    }

    #[test]
    fn test_number_some_value_is_set() {
        let input = Some(NullableNumber { value: Some(42) });
        assert_eq!(input.into_update_field(), UpdateField::Set(42));
    }

    #[test]
    fn test_number_some_zero_is_set() {
        let input = Some(NullableNumber { value: Some(0) });
        assert_eq!(input.into_update_field(), UpdateField::Set(0));
    }

    // ============== NullableBool Tests ==============

    #[test]
    fn test_bool_none_is_no_change() {
        let input: Option<NullableBool> = None;
        assert_eq!(input.into_update_field(), UpdateField::NoChange);
    }

    #[test]
    fn test_bool_some_none_is_clear() {
        let input = Some(NullableBool { value: None });
        assert_eq!(input.into_update_field(), UpdateField::Clear);
    }

    #[test]
    fn test_bool_some_true_is_set() {
        let input = Some(NullableBool { value: Some(true) });
        assert_eq!(input.into_update_field(), UpdateField::Set(true));
    }

    #[test]
    fn test_bool_some_false_is_set() {
        let input = Some(NullableBool { value: Some(false) });
        assert_eq!(input.into_update_field(), UpdateField::Set(false));
    }
}

// ============== Domain Error to gRPC Status ==============

/// 將 Domain Error 轉換為 gRPC Status
/// 
/// 這個函數提供統一的錯誤處理邏輯，確保所有 gRPC Service 都能一致地處理 Domain Error。
/// 
/// # 錯誤映射規則
/// 
/// - `ValidationError` → `INVALID_ARGUMENT` (400)
/// - `NotFound` 系列 → `NOT_FOUND` (404)
/// - `AlreadyExists` 系列 → `ALREADY_EXISTS` (409)
/// - `LayoutVersionMismatch` → `FAILED_PRECONDITION` (412)
/// - `GarageApiError` → `INTERNAL` (500)
/// - `InternalError` → `INTERNAL` (500)
/// - 其他未明確映射的錯誤 → `UNKNOWN` (500)
pub fn domain_error_to_status(err: DomainError) -> Status {
    match err {
        // ============ Validation Errors ============
        DomainError::ValidationError(msg) => {
            Status::invalid_argument(msg)
        }
        
        // ============ Not Found Errors (404) ============
        DomainError::BucketNotFound(msg) => {
            Status::not_found(msg)
        }
        DomainError::AccessKeyNotFound(msg) => {
            Status::not_found(msg)
        }
        DomainError::AdminTokenNotFound(msg) => {
            Status::not_found(msg)
        }
        DomainError::NodeNotFound(msg) => {
            Status::not_found(msg)
        }
        DomainError::ObjectNotFound(msg) => {
            Status::not_found(msg)
        }
        
        // ============ Already Exists Errors (409) ============
        DomainError::BucketAlreadyExists(msg) => {
            Status::already_exists(msg)
        }
        DomainError::AccessKeyAlreadyExists(msg) => {
            Status::already_exists(msg)
        }
        DomainError::LocalAliasAlreadyExists(msg) => {
            Status::already_exists(msg)
        }
        
        // ============ Validation/Business Rule Errors ============
        DomainError::InvalidBucketName(msg) => {
            Status::invalid_argument(format!("Invalid bucket name: {}", msg))
        }
        
        // ============ Precondition Failed (412) ============
        DomainError::LayoutVersionMismatch { expected, actual } => {
            Status::failed_precondition(format!(
                "Layout version mismatch: expected {}, got {}",
                expected, actual
            ))
        }
        
        // ============ Cluster Errors ============
        DomainError::ClusterOperationFailed(msg) => {
            Status::internal(format!("Cluster operation failed: {}", msg))
        }
        
        // ============ Infrastructure Errors (500) ============
        DomainError::GarageApiError(msg) => {
            Status::internal(format!("Garage API error: {}", msg))
        }
        DomainError::InternalError(msg) => {
            Status::internal(format!("Internal error: {}", msg))
        }
    }
}

/// Extension trait for `Result<T, DomainError>`，讓它能直接轉換為 `Result<T, Status>`
pub trait DomainErrorExt<T> {
    /// 將 `Result<T, DomainError>` 轉換為 `Result<T, Status>`
    fn into_grpc_result(self) -> Result<T, Status>;
}

impl<T> DomainErrorExt<T> for Result<T, DomainError> {
    fn into_grpc_result(self) -> Result<T, Status> {
        self.map_err(domain_error_to_status)
    }
}
