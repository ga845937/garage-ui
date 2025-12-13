//! DateTime Parser
//!
//! 共用的日期時間解析工具

use chrono::{DateTime, Utc};

/// 解析多種日期時間格式
/// 
/// 支援：
/// - RFC3339: "2025-01-04T23:59:59Z", "2025-01-04T23:59:59+08:00"
/// - ISO 8601 無時區: "2025-01-04T23:59:59", "2025-01-04T23:59"
/// - 常見格式: "2025/01/04 23:59:59", "2025-01-04 23:59:59"
/// - 僅日期: "2025/01/04", "2025-01-04"
/// 
/// 無時區的格式會被視為 UTC
pub fn parse_datetime(s: &str) -> Option<DateTime<Utc>> {
    if s.is_empty() {
        return None;
    }
    
    // 嘗試 RFC3339 (最標準，含時區)
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Utc));
    }
    
    // 嘗試常見格式（無時區，視為 UTC）
    let datetime_formats = [
        "%Y-%m-%dT%H:%M:%S",  // 2025-01-04T23:59:59 (ISO 8601 無時區)
        "%Y-%m-%dT%H:%M",     // 2025-01-04T23:59
        "%Y/%m/%d %H:%M:%S",  // 2025/01/04 23:59:59
        "%Y-%m-%d %H:%M:%S",  // 2025-01-04 23:59:59
        "%Y/%m/%d %H:%M",     // 2025/01/04 23:59
        "%Y-%m-%d %H:%M",     // 2025-01-04 23:59
    ];
    
    for fmt in datetime_formats {
        if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(s, fmt) {
            return Some(naive.and_utc());
        }
    }
    
    // 僅日期格式，補上 00:00:00
    let date_formats = [
        "%Y/%m/%d",  // 2025/01/04
        "%Y-%m-%d",  // 2025-01-04
    ];
    
    for fmt in date_formats {
        if let Ok(date) = chrono::NaiveDate::parse_from_str(s, fmt) {
            return Some(date.and_hms_opt(0, 0, 0)?.and_utc());
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike};

    use super::*;

    #[test]
    fn test_rfc3339() {
        let dt = parse_datetime("2025-01-04T23:59:59Z").unwrap();
        assert_eq!(dt.to_rfc3339(), "2025-01-04T23:59:59+00:00");
    }

    #[test]
    fn test_rfc3339_with_offset() {
        let dt = parse_datetime("2025-01-04T23:59:59+08:00").unwrap();
        assert_eq!(dt.hour(), 15); // 23 - 8 = 15 UTC
    }

    #[test]
    fn test_iso8601_no_timezone() {
        let dt = parse_datetime("2025-01-04T23:59:59").unwrap();
        assert_eq!(dt.to_rfc3339(), "2025-01-04T23:59:59+00:00");
    }

    #[test]
    fn test_iso8601_no_seconds() {
        let dt = parse_datetime("2025-01-04T23:59").unwrap();
        assert_eq!(dt.minute(), 59);
    }

    #[test]
    fn test_slash_format() {
        let dt = parse_datetime("2025/01/04 23:59:59").unwrap();
        assert_eq!(dt.day(), 4);
    }

    #[test]
    fn test_date_only() {
        let dt = parse_datetime("2025-01-04").unwrap();
        assert_eq!(dt.hour(), 0);
        assert_eq!(dt.minute(), 0);
    }

    #[test]
    fn test_empty_string() {
        assert!(parse_datetime("").is_none());
    }

    #[test]
    fn test_invalid_format() {
        assert!(parse_datetime("invalid").is_none());
    }
}
