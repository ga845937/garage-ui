//! Logging configuration
//!
//! Provides dual logging: Console (string format) and File (JSON format)
//! 
//! Console format: TIMESTAMP [level] message (no key=value pairs)
//! File format: JSON with full details

use std::path::Path;
use std::fmt as std_fmt;
use tracing::Level;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
    fmt::{self, time::UtcTime, format::Writer, time::FormatTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer,
};

/// ANSI color codes
const YELLOW: &str = "\x1b[33m";  // grpc
const GREEN: &str = "\x1b[32m";   // api
const RED: &str = "\x1b[31m";     // error
const CYAN: &str = "\x1b[36m";    // info
const RESET: &str = "\x1b[0m";

/// Custom event formatter that only outputs the message (no fields)
struct MessageOnlyFormat;

impl<S, N> tracing_subscriber::fmt::FormatEvent<S, N> for MessageOnlyFormat
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    N: for<'a> tracing_subscriber::fmt::FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &tracing_subscriber::fmt::FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std_fmt::Result {
        // Determine color based on target
        let target = event.metadata().target();
        let color = match target {
            "grpc" => YELLOW,
            "api" => GREEN,
            "error" => RED,
            _ => CYAN,
        };
        
        // Extract the message from the event
        let mut visitor = MessageVisitor::default();
        event.record(&mut visitor);
        
        if let Some(msg) = visitor.message {
            // Get timestamp
            let timer = UtcTime::rfc_3339();
            write!(writer, "{}", color)?;
            timer.format_time(&mut writer)?;
            writeln!(writer, " {}{}", msg, RESET)?;
        }
        
        Ok(())
    }
}

/// Visitor to extract only the message field
#[derive(Default)]
struct MessageVisitor {
    message: Option<String>,
}

impl tracing::field::Visit for MessageVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std_fmt::Debug) {
        if field.name() == "message" {
            self.message = Some(format!("{:?}", value));
        }
    }
    
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.message = Some(value.to_string());
        }
    }
}

/// Initialize the logging system with dual outputs
/// - Console: Simple message format (no key=value)
/// - File: Detailed JSON format for production analysis
pub fn init_logging(log_dir: &str) -> tracing_appender::non_blocking::WorkerGuard {
    // Create log directory if it doesn't exist
    let log_path = Path::new(log_dir);
    if !log_path.exists() {
        std::fs::create_dir_all(log_path).expect("Failed to create log directory");
    }

    // File appender with daily rotation
    let file_appender = RollingFileAppender::new(Rotation::DAILY, log_dir, "garage-ui.log");
    let (non_blocking_file, guard) = tracing_appender::non_blocking(file_appender);

    // Console layer - Message only format (no key=value pairs)
    let console_layer = fmt::layer()
        .event_format(MessageOnlyFormat)
        .with_ansi(true)
        .with_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()));

    // File layer - JSON format with all context
    let file_layer = fmt::layer()
        .json()
        .with_target(true)
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .flatten_event(false)
        .with_current_span(true)
        .with_span_list(true)
        .with_timer(UtcTime::rfc_3339())
        .with_writer(non_blocking_file)
        .with_filter(EnvFilter::from_default_env().add_directive(Level::DEBUG.into()));

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();

    guard
}
