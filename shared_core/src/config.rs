//! Configuration management
//!
//! This module provides utilities for loading and managing configuration.

use crate::error::{Result, SystemError};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Base configuration trait that all system configs should implement
pub trait Config: Sized + Serialize + for<'de> Deserialize<'de> {
    /// Load configuration from a TOML file
    fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref()).map_err(|e| {
            SystemError::io(e, format!("Failed to read config file: {:?}", path.as_ref()))
        })?;

        toml::from_str(&content).map_err(|e| {
            SystemError::config(format!("Failed to parse TOML: {}", e), None)
        })
    }

    /// Load configuration from environment variables with a prefix
    fn from_env(prefix: &str) -> Result<Self> {
        config::Config::builder()
            .add_source(config::Environment::with_prefix(prefix).separator("__"))
            .build()
            .map_err(|e| SystemError::config(format!("Failed to load from environment: {}", e), None))?
            .try_deserialize()
            .map_err(|e| SystemError::config(format!("Failed to deserialize config: {}", e), None))
    }

    /// Load configuration from multiple sources (file + env)
    fn load(file_path: Option<impl AsRef<Path>>, env_prefix: &str) -> Result<Self> {
        let mut builder = config::Config::builder();

        if let Some(path) = file_path {
            builder = builder.add_source(config::File::from(path.as_ref()));
        }

        builder = builder.add_source(config::Environment::with_prefix(env_prefix).separator("__"));

        builder
            .build()
            .map_err(|e| SystemError::config(format!("Failed to build config: {}", e), None))?
            .try_deserialize()
            .map_err(|e| SystemError::config(format!("Failed to deserialize config: {}", e), None))
    }

    /// Save configuration to a TOML file
    fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| SystemError::Serialization {
                message: e.to_string(),
                format: "TOML".to_string(),
            })?;

        std::fs::write(path.as_ref(), content).map_err(|e| {
            SystemError::io(e, format!("Failed to write config file: {:?}", path.as_ref()))
        })
    }

    /// Validate the configuration
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

/// Common server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
    /// Number of worker threads
    pub workers: Option<usize>,
    /// Request timeout in seconds
    pub timeout_secs: u64,
    /// Maximum concurrent connections
    pub max_connections: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            workers: None,
            timeout_secs: 30,
            max_connections: 1000,
        }
    }
}

/// Common database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database URL
    pub url: String,
    /// Maximum number of connections in the pool
    pub max_connections: u32,
    /// Minimum number of idle connections
    pub min_connections: u32,
    /// Connection timeout in seconds
    pub connect_timeout_secs: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite::memory:".to_string(),
            max_connections: 10,
            min_connections: 2,
            connect_timeout_secs: 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestConfig {
        name: String,
        value: u32,
    }

    impl Config for TestConfig {
        fn validate(&self) -> Result<()> {
            if self.value == 0 {
                return Err(SystemError::validation("value", "must be > 0", Some("0".to_string())));
            }
            Ok(())
        }
    }

    #[test]
    fn test_config_save_load() {
        let config = TestConfig {
            name: "test".to_string(),
            value: 42,
        };

        let temp_file = NamedTempFile::new().unwrap();
        config.save(temp_file.path()).unwrap();

        let loaded: TestConfig = TestConfig::from_file(temp_file.path()).unwrap();
        assert_eq!(loaded, config);
    }

    #[test]
    fn test_config_validation() {
        let invalid_config = TestConfig {
            name: "test".to_string(),
            value: 0,
        };

        assert!(invalid_config.validate().is_err());

        let valid_config = TestConfig {
            name: "test".to_string(),
            value: 42,
        };

        assert!(valid_config.validate().is_ok());
    }
}
