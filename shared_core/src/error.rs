//! Unified error types for all systems
//!
//! This module defines the common error types used throughout the monorepo.
//! All system-specific errors should wrap `SystemError` for consistency.

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Root error type for all systems
///
/// This enum represents all possible error categories that can occur across
/// the entire system. System-specific errors should use the `SystemSpecific`
/// variant to wrap their own error types.
#[derive(Debug, Error, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum SystemError {
    /// I/O error with context
    #[error("I/O error: {context} - {message}")]
    Io {
        /// The underlying I/O error message
        message: String,
        /// Additional context about where/why the error occurred
        context: String,
    },

    /// Configuration error
    #[error("Configuration error: {message}")]
    Config {
        /// Error message
        message: String,
        /// The configuration key that caused the error (if applicable)
        key: Option<String>,
    },

    /// Cryptographic operation error
    #[error("Cryptographic error during {operation}: {details}")]
    Crypto {
        /// The cryptographic operation that failed
        operation: String,
        /// Details about the failure
        details: String,
    },

    /// Concurrency error (deadlock, race condition, etc.)
    #[error("Concurrency error: {message}")]
    Concurrency {
        /// Error message
        message: String,
        /// Thread ID if applicable
        thread_id: Option<String>,
    },

    /// Validation error
    #[error("Validation error in field '{field}': {reason}")]
    Validation {
        /// The field that failed validation
        field: String,
        /// Why the validation failed
        reason: String,
        /// The invalid value (if safe to include)
        value: Option<String>,
    },

    /// Network error
    #[error("Network error: {operation} - {message}")]
    Network {
        /// The network operation that failed
        operation: String,
        /// Error message
        message: String,
        /// Retry attempt number (if applicable)
        retry_attempt: Option<u32>,
    },

    /// Database error
    #[error("Database error during {operation}: {message}")]
    Database {
        /// The database operation that failed
        operation: String,
        /// Error message
        message: String,
    },

    /// Serialization/Deserialization error
    #[error("Serialization error: {message}")]
    Serialization {
        /// Error message
        message: String,
        /// The format being serialized to/from
        format: String,
    },

    /// Timeout error
    #[error("Operation timed out after {duration_ms}ms: {operation}")]
    Timeout {
        /// The operation that timed out
        operation: String,
        /// Timeout duration in milliseconds
        duration_ms: u64,
    },

    /// Resource not found
    #[error("Resource not found: {resource_type} '{identifier}'")]
    NotFound {
        /// Type of resource
        resource_type: String,
        /// Resource identifier
        identifier: String,
    },

    /// Permission denied
    #[error("Permission denied: {operation}")]
    PermissionDenied {
        /// The operation that was denied
        operation: String,
        /// Required permission
        required_permission: Option<String>,
    },

    /// Resource already exists
    #[error("Resource already exists: {resource_type} '{identifier}'")]
    AlreadyExists {
        /// Type of resource
        resource_type: String,
        /// Resource identifier
        identifier: String,
    },

    /// Invalid state
    #[error("Invalid state: {message}")]
    InvalidState {
        /// Error message
        message: String,
        /// Current state
        current_state: Option<String>,
        /// Expected state
        expected_state: Option<String>,
    },

    /// System-specific error
    #[error("System-specific error: {system} - {message}")]
    SystemSpecific {
        /// System name
        system: String,
        /// Error message
        message: String,
        /// Additional context
        context: Option<String>,
    },

    /// Internal error (programming error, should not happen)
    #[error("Internal error: {message}")]
    Internal {
        /// Error message
        message: String,
        /// Source location (file:line)
        location: Option<String>,
    },
}

/// Result type alias used throughout the codebase
pub type Result<T> = std::result::Result<T, SystemError>;

impl SystemError {
    /// Create an I/O error with context
    pub fn io(source: impl fmt::Display, context: impl Into<String>) -> Self {
        Self::Io {
            message: source.to_string(),
            context: context.into(),
        }
    }

    /// Create a configuration error
    pub fn config(message: impl Into<String>, key: Option<String>) -> Self {
        Self::Config {
            message: message.into(),
            key,
        }
    }

    /// Create a cryptographic error
    pub fn crypto(operation: impl Into<String>, details: impl Into<String>) -> Self {
        Self::Crypto {
            operation: operation.into(),
            details: details.into(),
        }
    }

    /// Create a validation error
    pub fn validation(
        field: impl Into<String>,
        reason: impl Into<String>,
        value: Option<String>,
    ) -> Self {
        Self::Validation {
            field: field.into(),
            reason: reason.into(),
            value,
        }
    }

    /// Create a network error
    pub fn network(
        operation: impl Into<String>,
        message: impl Into<String>,
        retry_attempt: Option<u32>,
    ) -> Self {
        Self::Network {
            operation: operation.into(),
            message: message.into(),
            retry_attempt,
        }
    }

    /// Create a timeout error
    pub fn timeout(operation: impl Into<String>, duration_ms: u64) -> Self {
        Self::Timeout {
            operation: operation.into(),
            duration_ms,
        }
    }

    /// Create a not found error
    pub fn not_found(resource_type: impl Into<String>, identifier: impl Into<String>) -> Self {
        Self::NotFound {
            resource_type: resource_type.into(),
            identifier: identifier.into(),
        }
    }

    /// Create an internal error
    pub fn internal(message: impl Into<String>, location: Option<String>) -> Self {
        Self::Internal {
            message: message.into(),
            location,
        }
    }
}

// Implement From for common error types
impl From<std::io::Error> for SystemError {
    fn from(err: std::io::Error) -> Self {
        Self::io(err, "I/O operation failed")
    }
}

impl From<serde_json::Error> for SystemError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization {
            message: err.to_string(),
            format: "JSON".to_string(),
        }
    }
}

impl From<toml::de::Error> for SystemError {
    fn from(err: toml::de::Error) -> Self {
        Self::Serialization {
            message: err.to_string(),
            format: "TOML".to_string(),
        }
    }
}

impl From<bincode::Error> for SystemError {
    fn from(err: bincode::Error) -> Self {
        Self::Serialization {
            message: err.to_string(),
            format: "bincode".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = SystemError::validation("email", "Invalid format", Some("not-an-email".to_string()));
        assert!(err.to_string().contains("email"));
        assert!(err.to_string().contains("Invalid format"));
    }

    #[test]
    fn test_error_serialization() {
        let err = SystemError::config("Missing required field".to_string(), Some("database.url".to_string()));
        let json = serde_json::to_string(&err).unwrap();
        let deserialized: SystemError = serde_json::from_str(&json).unwrap();
        assert!(matches!(deserialized, SystemError::Config { .. }));
    }
}
