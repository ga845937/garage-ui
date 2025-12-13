//! UpdateField Value Object
//!
//! 用於區分「不更新」、「清除」和「設為新值」三種更新意圖

use serde::{Deserialize, Serialize};

/// 更新欄位的三種狀態
/// 
/// - `NoChange`: 不更新此欄位，保留原值
/// - `Clear`: 清除此欄位（設為 null/空值）
/// - `Set(T)`: 設為新值
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UpdateField<T> {
    /// 不更新此欄位
    NoChange,
    /// 清除此欄位（設為 null/空值）
    Clear,
    /// 設為新值
    Set(T),
}

impl<T> Default for UpdateField<T> {
    fn default() -> Self {
        UpdateField::NoChange
    }
}

impl<T> UpdateField<T> {
    /// 檢查是否有變更
    pub fn has_change(&self) -> bool {
        !matches!(self, UpdateField::NoChange)
    }

    /// 檢查是否為清除操作
    pub fn is_clear(&self) -> bool {
        matches!(self, UpdateField::Clear)
    }

    /// 檢查是否為設值操作
    pub fn is_set(&self) -> bool {
        matches!(self, UpdateField::Set(_))
    }

    /// 取得設定的值（如果有的話）
    pub fn get(&self) -> Option<&T> {
        match self {
            UpdateField::Set(v) => Some(v),
            _ => None,
        }
    }

    /// 取得設定的值（消耗 self）
    pub fn into_value(self) -> Option<T> {
        match self {
            UpdateField::Set(v) => Some(v),
            _ => None,
        }
    }

    /// 從 Option 轉換
    /// - None -> NoChange
    /// - Some(v) -> Set(v)
    pub fn from_option(opt: Option<T>) -> Self {
        match opt {
            Some(v) => UpdateField::Set(v),
            None => UpdateField::NoChange,
        }
    }

    /// 從雙層 Option 轉換（用於需要區分三種狀態的場景）
    /// - None -> NoChange
    /// - Some(None) -> Clear
    /// - Some(Some(v)) -> Set(v)
    pub fn from_double_option(opt: Option<Option<T>>) -> Self {
        match opt {
            None => UpdateField::NoChange,
            Some(None) => UpdateField::Clear,
            Some(Some(v)) => UpdateField::Set(v),
        }
    }

    /// 應用更新到現有值
    pub fn apply(self, current: T) -> T
    where
        T: Default,
    {
        match self {
            UpdateField::NoChange => current,
            UpdateField::Clear => T::default(),
            UpdateField::Set(v) => v,
        }
    }

    /// 應用更新到 Option 值
    pub fn apply_option(self, current: Option<T>) -> Option<T> {
        match self {
            UpdateField::NoChange => current,
            UpdateField::Clear => None,
            UpdateField::Set(v) => Some(v),
        }
    }

    /// 轉換為 Option（用於 API 請求）
    /// - NoChange -> None（不傳送此欄位）
    /// - Clear -> Some(default)（傳送空值）
    /// - Set(v) -> Some(v)
    pub fn to_option(self) -> Option<T>
    where
        T: Default,
    {
        match self {
            UpdateField::NoChange => None,
            UpdateField::Clear => Some(T::default()),
            UpdateField::Set(v) => Some(v),
        }
    }

    /// 轉換內部值的類型
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> UpdateField<U> {
        match self {
            UpdateField::NoChange => UpdateField::NoChange,
            UpdateField::Clear => UpdateField::Clear,
            UpdateField::Set(v) => UpdateField::Set(f(v)),
        }
    }
}

/// 為 String 類型提供特殊的清除判斷
impl UpdateField<String> {
    /// 從 Option<String> 轉換，空字串視為 Clear
    /// - None -> NoChange
    /// - Some("") -> Clear
    /// - Some(v) -> Set(v)
    pub fn from_option_with_empty_as_clear(opt: Option<String>) -> Self {
        match opt {
            None => UpdateField::NoChange,
            Some(s) if s.is_empty() => UpdateField::Clear,
            Some(s) => UpdateField::Set(s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_option() {
        assert_eq!(UpdateField::<String>::from_option(None), UpdateField::NoChange);
        assert_eq!(
            UpdateField::from_option(Some("test".to_string())),
            UpdateField::Set("test".to_string())
        );
    }

    #[test]
    fn test_apply() {
        let current = "original".to_string();
        
        assert_eq!(UpdateField::<String>::NoChange.apply(current.clone()), "original");
        assert_eq!(UpdateField::<String>::Clear.apply(current.clone()), "");
        assert_eq!(UpdateField::Set("new".to_string()).apply(current), "new");
    }

    #[test]
    fn test_apply_option() {
        let current = Some("original".to_string());
        
        assert_eq!(UpdateField::<String>::NoChange.apply_option(current.clone()), Some("original".to_string()));
        assert_eq!(UpdateField::<String>::Clear.apply_option(current.clone()), None);
        assert_eq!(UpdateField::Set("new".to_string()).apply_option(current), Some("new".to_string()));
    }
}
