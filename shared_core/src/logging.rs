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

    let guard = if config.log_to_file {
        if let Some(log_path) = &config.log_file_path {
            let file_appender = tracing_appender::rolling::daily(
                std::path::Path::new(log_path).parent().unwrap_or(std::path::Path::new(".")),
                std::path::Path::new(log_path)
                    .file_name()
                    .unwrap_or(std::ffi::OsStr::new("app.log")),
            );
            let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

            let subscriber = tracing_subscriber::fmt()
                .with_writer(non_blocking)
                .with_env_filter(env_filter)
                .json()
                .finish();

            tracing::subscriber::set_global_default(subscriber)
                .map_err(|e| SystemError::Concurrency {
                    message: format!("Failed to set global subscriber: {}", e),
                    thread_id: None,
                })?;

            Some(guard)
        } else {
            init_simple(config)?;
            None
        }
    } else {
        init_simple(config)?;
        None
    };

    Ok(guard)
}

fn init_simple(config: LogConfig) -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(config.level.to_string()));

    // Simplified: always use JSON format for consistency
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .json()
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| SystemError::Concurrency {
            message: format!("Failed to set global subscriber: {}", e),
            thread_id: None,
        })?;

    Ok(())
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
