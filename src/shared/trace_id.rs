//! Trace ID generation using ObjectId format (millisecond precision)

use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// Generate a unique trace ID in ObjectId-like format (24 hex chars)
/// Format: 8 chars timestamp (seconds) + 6 chars machine/process + 6 chars counter + 4 chars milliseconds
pub fn generate_trace_id() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    
    let timestamp_secs = now.as_secs() as u32;
    let millis = now.subsec_millis() as u16;
    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
    
    // Process ID (truncated to 3 bytes)
    let pid = std::process::id() as u32 & 0xFFFFFF;
    
    format!(
        "{:08x}{:06x}{:06x}{:04x}",
        timestamp_secs,
        pid,
        counter & 0xFFFFFF,
        millis
    )
}

/// Parse a trace_id back to DateTime<Utc>
/// Returns None if the trace_id format is invalid
pub fn parse_trace_id_time(trace_id: &str) -> Option<DateTime<Utc>> {
    if trace_id.len() != 24 {
        return None;
    }
    
    // Parse timestamp (first 8 hex chars = seconds)
    let timestamp_secs = u64::from_str_radix(&trace_id[0..8], 16).ok()?;
    // Parse milliseconds (last 4 hex chars)
    let millis = u64::from_str_radix(&trace_id[20..24], 16).ok()?;
    
    let duration = Duration::from_secs(timestamp_secs) + Duration::from_millis(millis);
    let system_time = UNIX_EPOCH + duration;
    
    Some(DateTime::<Utc>::from(system_time))
}

/// Parse a trace_id and return formatted time string (ISO 8601)
pub fn trace_id_to_time_string(trace_id: &str) -> Option<String> {
    parse_trace_id_time(trace_id).map(|dt| dt.format("%Y-%m-%d %H:%M:%S%.3f UTC").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_id_length() {
        let id = generate_trace_id();
        assert_eq!(id.len(), 24);
    }

    #[test]
    fn test_trace_id_unique() {
        let id1 = generate_trace_id();
        let id2 = generate_trace_id();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_parse_trace_id_time() {
        let id = generate_trace_id();
        let parsed = parse_trace_id_time(&id);
        assert!(parsed.is_some());
        
        // Should be within last few seconds
        let now = Utc::now();
        let parsed_time = parsed.unwrap();
        let diff = now.signed_duration_since(parsed_time);
        assert!(diff.num_seconds().abs() < 5);
    }

    #[test]
    fn test_trace_id_to_time_string() {
        let id = generate_trace_id();
        let time_str = trace_id_to_time_string(&id);
        assert!(time_str.is_some());
        assert!(time_str.unwrap().contains("UTC"));
    }

    #[test]
    fn test_invalid_trace_id() {
        assert!(parse_trace_id_time("invalid").is_none());
        assert!(parse_trace_id_time("").is_none());
        assert!(parse_trace_id_time("12345").is_none());
    }
}
