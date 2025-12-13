//! List access keys query

use chrono::{DateTime, Utc};
use crate::domain::entities::garage::KeyListItemResponse;
use crate::shared::parse_datetime;

/// Query to list all access keys
/// 
/// Query 本身就是查詢規格，包含過濾、分頁等條件
#[derive(Debug, Clone, Default)]
pub struct ListKeysQuery {
    // 分頁
    pub page: i32,
    pub page_size: i32,
    
    // 過濾條件
    /// 名稱包含（模糊搜尋，不分大小寫）
    pub name: Option<String>,
    pub created_start: Option<DateTime<Utc>>,
    pub created_end: Option<DateTime<Utc>>,
    pub expiration_start: Option<DateTime<Utc>>,
    pub expiration_end: Option<DateTime<Utc>>,
    pub expired: Option<bool>,
}

impl ListKeysQuery {
    pub fn new(page: i32, page_size: i32) -> Self {
        Self { 
            page, 
            page_size,
            ..Default::default()
        }
    }

    /// 從 gRPC 請求建立 Query
    pub fn from_grpc_request(
        page: i32,
        page_size: i32,
        name: Option<String>,
        created_start: Option<String>,
        created_end: Option<String>,
        expiration_start: Option<String>,
        expiration_end: Option<String>,
    ) -> Self {
        Self::new(page, page_size)
            .with_name(name)
            .with_created_range(
                created_start.and_then(|s| parse_datetime(&s)),
                created_end.and_then(|s| parse_datetime(&s)),
            )
            .with_expiration_range(
                expiration_start.and_then(|s| parse_datetime(&s)),
                expiration_end.and_then(|s| parse_datetime(&s)),
            )
    }

    // ============ Builder Methods ============

    pub fn with_name(mut self, name: Option<String>) -> Self {
        self.name = name;
        self
    }

    pub fn with_created_range(
        mut self, 
        start: Option<DateTime<Utc>>, 
        end: Option<DateTime<Utc>>
    ) -> Self {
        self.created_start = start;
        self.created_end = end;
        self
    }

    pub fn with_expiration_range(
        mut self, 
        start: Option<DateTime<Utc>>, 
        end: Option<DateTime<Utc>>
    ) -> Self {
        self.expiration_start = start;
        self.expiration_end = end;
        self
    }

    pub fn with_expired(mut self, expired: Option<bool>) -> Self {
        self.expired = expired;
        self
    }

    // ============ Filter Logic ============

    /// 是否有任何過濾條件
    pub fn has_filter(&self) -> bool {
        self.name.is_some()
            || self.created_start.is_some()
            || self.created_end.is_some()
            || self.expiration_start.is_some()
            || self.expiration_end.is_some()
            || self.expired.is_some()
    }

    /// 檢查項目是否符合所有過濾條件
    pub fn matches(&self, item: &KeyListItemResponse) -> bool {
        self.matches_name(item)
            && self.matches_created(item)
            && self.matches_expiration(item)
            && self.matches_expired(item)
    }

    fn matches_name(&self, item: &KeyListItemResponse) -> bool {
        match &self.name {
            Some(search) => item.name.to_lowercase().contains(&search.to_lowercase()),
            None => true,
        }
    }

    fn matches_created(&self, item: &KeyListItemResponse) -> bool {
        let convert_created = parse_datetime(item.created.as_str())
            .expect("Invalid created datetime format in item");

        if let Some(ref start) = self.created_start {
            if &convert_created < start {
                return false;
            }
        }
        if let Some(ref end) = self.created_end {
            if &convert_created > end {
                return false;
            }
        }
        true
    }

    fn matches_expiration(&self, item: &KeyListItemResponse) -> bool {
        if let Some(ref start) = self.expiration_start {
            match &item.expiration {
                Some(exp) if &parse_datetime(exp).unwrap() < start => return false,
                None => return false, // 永不過期視為不符合過期範圍篩選
                _ => {}
            }
        }
        if let Some(ref end) = self.expiration_end {
            match &item.expiration {
                Some(exp) if &parse_datetime(exp).unwrap() > end => return false,
                None => return false,
                _ => {}
            }
        }
        true
    }

    fn matches_expired(&self, item: &KeyListItemResponse) -> bool {
        match self.expired {
            Some(expired) => item.expired == expired,
            None => true,
        }
    }
}
