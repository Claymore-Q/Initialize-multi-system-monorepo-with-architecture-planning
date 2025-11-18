//! Structured logging configuration
//!
//! This module provides a unified logging setup for all systems using the `tracing` crate.

use crate::error::{Result, SystemError};
use tracing::{Level, Subscriber};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer, Registry,
};

/// Log output format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    /// Human-readable pretty format (for development)
    Pretty,
    /// JSON format (for production)
    Json,
    /// Compact format
    Compact,
}

/// Logging configuration
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// Log level filter
    pub level: Level,
    /// Output format
    pub format: LogFormat,
    /// Whether to include file and line numbers
    pub include_location: bool,
    /// Whether to include target module path
    pub include_target: bool,
    /// Whether to log to file
    pub log_to_file: bool,
    /// Log file path (if log_to_file is true)
    pub log_file_path: Option<String>,
    /// Span events to log
    pub span_events: FmtSpan,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: Level::INFO,
            format: LogFormat::Pretty,
            include_location: true,
            include_target: true,
            log_to_file: false,
            log_file_path: None,
            span_events: FmtSpan::CLOSE,
        }
    }
}

impl LogConfig {
    /// Create a production configuration
    pub fn production() -> Self {
        Self {
            level: Level::INFO,
            format: LogFormat::Json,
            include_location: false,
            include_target: true,
            log_to_file: true,
            log_file_path: Some("/var/log/semantic_notary/app.log".to_string()),
            span_events: FmtSpan::NONE,
        }
    }

    /// Create a development configuration
    pub fn development() -> Self {
        Self {
            level: Level::DEBUG,
            format: LogFormat::Pretty,
            include_location: true,
            include_target: true,
            log_to_file: false,
            log_file_path: None,
            span_events: FmtSpan::CLOSE,
        }
    }

    /// Create a test configuration
    pub fn test() -> Self {
        Self {
            level: Level::TRACE,
            format: LogFormat::Compact,
            include_location: true,
            include_target: false,
            log_to_file: false,
            log_file_path: None,
            span_events: FmtSpan::FULL,
        }
    }
}

/// Initialize logging with the given configuration
///
/// Returns a `WorkerGuard` that must be kept alive for the duration of the program
/// to ensure all logs are flushed. If logging to file is disabled, returns `None`.
pub fn init_logging(config: LogConfig) -> Result<Option<WorkerGuard>> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(config.level.to_string()));

    let subscriber = Registry::default().with(env_filter);

    let guard = if config.log_to_file {
        if let Some(log_path) = &config.log_file_path {
            let file_appender = tracing_appender::rolling::daily(
                std::path::Path::new(log_path).parent().unwrap_or(std::path::Path::new(".")),
                std::path::Path::new(log_path)
                    .file_name()
                    .unwrap_or(std::ffi::OsStr::new("app.log")),
            );
            let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

            let file_layer = fmt::layer()
                .with_writer(non_blocking)
                .json()
                .with_file(config.include_location)
                .with_line_number(config.include_location)
                .with_target(config.include_target)
                .with_span_events(config.span_events);

            subscriber
                .with(file_layer)
                .with(create_stdout_layer(&config))
                .init();

            Some(guard)
        } else {
            subscriber.with(create_stdout_layer(&config)).init();
            None
        }
    } else {
        subscriber.with(create_stdout_layer(&config)).init();
        None
    };

    Ok(guard)
}

fn create_stdout_layer(
    config: &LogConfig,
) -> Box<dyn Layer<Registry> + Send + Sync + 'static> {
    match config.format {
        LogFormat::Pretty => Box::new(
            fmt::layer()
                .pretty()
                .with_file(config.include_location)
                .with_line_number(config.include_location)
                .with_target(config.include_target)
                .with_span_events(config.span_events),
        ),
        LogFormat::Json => Box::new(
            fmt::layer()
                .json()
                .with_file(config.include_location)
                .with_line_number(config.include_location)
                .with_target(config.include_target)
                .with_span_events(config.span_events),
        ),
        LogFormat::Compact => Box::new(
            fmt::layer()
                .compact()
                .with_file(config.include_location)
                .with_line_number(config.include_location)
                .with_target(config.include_target)
                .with_span_events(config.span_events),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::{info, warn};

    #[test]
    fn test_logging_initialization() {
        let config = LogConfig::test();
        let _guard = init_logging(config);

        info!("Test info log");
        warn!("Test warning log");
    }
}
