//! Telemetry and observability utilities
//!
//! This module provides OpenTelemetry integration for distributed tracing and metrics.

use crate::error::{Result, SystemError};
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;

/// Telemetry configuration
#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    /// Service name for telemetry
    pub service_name: String,
    /// Service version
    pub service_version: String,
    /// Enable OpenTelemetry tracing
    pub enable_tracing: bool,
    /// OpenTelemetry collector endpoint
    pub otel_endpoint: Option<String>,
    /// Enable Prometheus metrics
    pub enable_metrics: bool,
    /// Prometheus metrics endpoint
    pub metrics_endpoint: Option<SocketAddr>,
    /// Trace sampling ratio (0.0 to 1.0)
    pub trace_sampling_ratio: f64,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            service_name: "semantic_notary".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            enable_tracing: false,
            otel_endpoint: None,
            enable_metrics: false,
            metrics_endpoint: None,
            trace_sampling_ratio: 1.0,
        }
    }
}

impl TelemetryConfig {
    /// Create a production telemetry configuration
    pub fn production(service_name: String) -> Self {
        Self {
            service_name,
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            enable_tracing: true,
            otel_endpoint: Some("http://localhost:4317".to_string()),
            enable_metrics: true,
            metrics_endpoint: Some("0.0.0.0:9090".parse().unwrap()),
            trace_sampling_ratio: 0.1, // Sample 10% in production
        }
    }

    /// Create a development telemetry configuration
    pub fn development(service_name: String) -> Self {
        Self {
            service_name,
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            enable_tracing: true,
            otel_endpoint: Some("http://localhost:4317".to_string()),
            enable_metrics: true,
            metrics_endpoint: Some("127.0.0.1:9090".parse().unwrap()),
            trace_sampling_ratio: 1.0, // Sample everything in development
        }
    }
}

/// Initialize telemetry based on configuration
pub fn init_telemetry(config: &TelemetryConfig) -> Result<()> {
    if config.enable_metrics {
        if let Some(addr) = config.metrics_endpoint {
            PrometheusBuilder::new()
                .with_http_listener(addr)
                .install()
                .map_err(|e| {
                    SystemError::config(
                        format!("Failed to initialize Prometheus metrics: {}", e),
                        None,
                    )
                })?;
        }
    }

    // OpenTelemetry tracing initialization would go here
    // Simplified for now since full OTEL setup is complex

    Ok(())
}

/// Record a counter metric
#[macro_export]
macro_rules! count {
    ($name:expr, $value:expr $(, $key:expr => $val:expr)*) => {
        metrics::counter!($name $(, $key => $val)*).increment($value);
    };
}

/// Record a gauge metric
#[macro_export]
macro_rules! gauge {
    ($name:expr, $value:expr $(, $key:expr => $val:expr)*) => {
        metrics::gauge!($name $(, $key => $val)*).set($value);
    };
}

/// Record a histogram metric
#[macro_export]
macro_rules! histogram {
    ($name:expr, $value:expr $(, $key:expr => $val:expr)*) => {
        metrics::histogram!($name $(, $key => $val)*).record($value);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_config() {
        let config = TelemetryConfig::default();
        assert_eq!(config.service_name, "semantic_notary");
        assert!(!config.enable_tracing);
    }
}
