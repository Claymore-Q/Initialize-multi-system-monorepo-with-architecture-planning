//! Plugin System
//!
//! Provides a flexible plugin architecture for extending system functionality.
//! All systems can load and execute plugins dynamically.

use crate::{Result, SystemError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// Unique plugin identifier
    pub id: String,

    /// Human-readable plugin name
    pub name: String,

    /// Plugin version
    pub version: String,

    /// Plugin author
    pub author: String,

    /// Plugin description
    pub description: String,

    /// Plugin capabilities/features
    pub capabilities: Vec<String>,

    /// Minimum system version required
    pub min_system_version: String,
}

impl PluginMetadata {
    /// Create new plugin metadata
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            author: String::new(),
            description: String::new(),
            capabilities: Vec::new(),
            min_system_version: "0.1.0".into(),
        }
    }

    /// Add a capability
    pub fn with_capability(mut self, capability: impl Into<String>) -> Self {
        self.capabilities.push(capability.into());
        self
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Set author
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = author.into();
        self
    }
}

/// Plugin lifecycle state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginState {
    /// Plugin is loaded but not initialized
    Loaded,

    /// Plugin is initialized and ready
    Ready,

    /// Plugin is active and running
    Active,

    /// Plugin is paused
    Paused,

    /// Plugin encountered an error
    Error,

    /// Plugin is unloaded
    Unloaded,
}

/// Plugin trait that all plugins must implement
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> &PluginMetadata;

    /// Initialize the plugin
    async fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    /// Start the plugin (make it active)
    async fn start(&mut self) -> Result<()> {
        Ok(())
    }

    /// Stop the plugin
    async fn stop(&mut self) -> Result<()> {
        Ok(())
    }

    /// Pause the plugin
    async fn pause(&mut self) -> Result<()> {
        Ok(())
    }

    /// Resume the plugin
    async fn resume(&mut self) -> Result<()> {
        Ok(())
    }

    /// Execute plugin-specific logic
    async fn execute(&mut self, input: PluginInput) -> Result<PluginOutput>;

    /// Get plugin state
    fn state(&self) -> PluginState;

    /// Health check
    async fn health_check(&self) -> Result<()> {
        Ok(())
    }

    /// Get plugin as Any for downcasting
    fn as_any(&self) -> &dyn Any;

    /// Get mutable plugin as Any for downcasting
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Input data for plugin execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInput {
    /// Input data as key-value pairs
    pub data: HashMap<String, serde_json::Value>,

    /// Plugin-specific context
    pub context: HashMap<String, String>,
}

impl PluginInput {
    /// Create new plugin input
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            context: HashMap::new(),
        }
    }

    /// Add data field
    pub fn with_data(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.data.insert(key.into(), value);
        self
    }

    /// Add context field
    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.insert(key.into(), value.into());
        self
    }

    /// Get data field
    pub fn get_data(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }

    /// Get context field
    pub fn get_context(&self, key: &str) -> Option<&String> {
        self.context.get(key)
    }
}

impl Default for PluginInput {
    fn default() -> Self {
        Self::new()
    }
}

/// Output data from plugin execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginOutput {
    /// Success flag
    pub success: bool,

    /// Output data as key-value pairs
    pub data: HashMap<String, serde_json::Value>,

    /// Optional error message
    pub error: Option<String>,

    /// Execution metrics
    pub metrics: HashMap<String, f64>,
}

impl PluginOutput {
    /// Create successful output
    pub fn success() -> Self {
        Self {
            success: true,
            data: HashMap::new(),
            error: None,
            metrics: HashMap::new(),
        }
    }

    /// Create failed output
    pub fn failure(error: impl Into<String>) -> Self {
        Self {
            success: false,
            data: HashMap::new(),
            error: Some(error.into()),
            metrics: HashMap::new(),
        }
    }

    /// Add data field
    pub fn with_data(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.data.insert(key.into(), value);
        self
    }

    /// Add metric
    pub fn with_metric(mut self, key: impl Into<String>, value: f64) -> Self {
        self.metrics.insert(key.into(), value);
        self
    }
}

/// Plugin registry for managing loaded plugins
pub struct PluginRegistry {
    plugins: Arc<RwLock<HashMap<String, Box<dyn Plugin>>>>,
    states: Arc<RwLock<HashMap<String, PluginState>>>,
}

impl PluginRegistry {
    /// Create a new plugin registry
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            states: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a plugin
    pub async fn register(&self, plugin: Box<dyn Plugin>) -> Result<()> {
        let id = plugin.metadata().id.clone();

        let mut plugins = self.plugins.write().await;
        let mut states = self.states.write().await;

        if plugins.contains_key(&id) {
            return Err(SystemError::Validation {
                field: "plugin_id".into(),
                reason: format!("Plugin with ID '{}' already registered", id),
                value: Some(id.clone()),
            });
        }

        states.insert(id.clone(), PluginState::Loaded);
        plugins.insert(id, plugin);

        Ok(())
    }

    /// Unregister a plugin
    pub async fn unregister(&self, plugin_id: &str) -> Result<()> {
        let mut plugins = self.plugins.write().await;
        let mut states = self.states.write().await;

        plugins.remove(plugin_id).ok_or_else(|| SystemError::Validation {
            field: "plugin_id".into(),
            reason: format!("Plugin '{}' not found", plugin_id),
            value: Some(plugin_id.to_string()),
        })?;

        states.insert(plugin_id.to_string(), PluginState::Unloaded);

        Ok(())
    }

    /// Get a plugin by ID
    pub async fn get(&self, plugin_id: &str) -> Result<String> {
        let plugins = self.plugins.read().await;

        plugins
            .get(plugin_id)
            .map(|p| p.metadata().name.clone())
            .ok_or_else(|| SystemError::Validation {
                field: "plugin_id".into(),
                reason: format!("Plugin '{}' not found", plugin_id),
                value: Some(plugin_id.to_string()),
            })
    }

    /// Initialize a plugin
    pub async fn initialize(&self, plugin_id: &str) -> Result<()> {
        let mut plugins = self.plugins.write().await;
        let mut states = self.states.write().await;

        let plugin = plugins.get_mut(plugin_id).ok_or_else(|| SystemError::Validation {
            field: "plugin_id".into(),
            reason: format!("Plugin '{}' not found", plugin_id),
            value: Some(plugin_id.to_string()),
        })?;

        plugin.initialize().await?;
        states.insert(plugin_id.to_string(), PluginState::Ready);

        Ok(())
    }

    /// Start a plugin
    pub async fn start(&self, plugin_id: &str) -> Result<()> {
        let mut plugins = self.plugins.write().await;
        let mut states = self.states.write().await;

        let plugin = plugins.get_mut(plugin_id).ok_or_else(|| SystemError::Validation {
            field: "plugin_id".into(),
            reason: format!("Plugin '{}' not found", plugin_id),
            value: Some(plugin_id.to_string()),
        })?;

        plugin.start().await?;
        states.insert(plugin_id.to_string(), PluginState::Active);

        Ok(())
    }

    /// Stop a plugin
    pub async fn stop(&self, plugin_id: &str) -> Result<()> {
        let mut plugins = self.plugins.write().await;
        let mut states = self.states.write().await;

        let plugin = plugins.get_mut(plugin_id).ok_or_else(|| SystemError::Validation {
            field: "plugin_id".into(),
            reason: format!("Plugin '{}' not found", plugin_id),
            value: Some(plugin_id.to_string()),
        })?;

        plugin.stop().await?;
        states.insert(plugin_id.to_string(), PluginState::Ready);

        Ok(())
    }

    /// Execute a plugin
    pub async fn execute(&self, plugin_id: &str, input: PluginInput) -> Result<PluginOutput> {
        let mut plugins = self.plugins.write().await;

        let plugin = plugins.get_mut(plugin_id).ok_or_else(|| SystemError::Validation {
            field: "plugin_id".into(),
            reason: format!("Plugin '{}' not found", plugin_id),
            value: Some(plugin_id.to_string()),
        })?;

        plugin.execute(input).await
    }

    /// List all registered plugins
    pub async fn list(&self) -> Vec<PluginMetadata> {
        let plugins = self.plugins.read().await;
        plugins.values().map(|p| p.metadata().clone()).collect()
    }

    /// Get plugin state
    pub async fn get_state(&self, plugin_id: &str) -> Option<PluginState> {
        let states = self.states.read().await;
        states.get(plugin_id).copied()
    }

    /// Health check all plugins
    pub async fn health_check_all(&self) -> HashMap<String, Result<()>> {
        let plugins = self.plugins.read().await;
        let mut results = HashMap::new();

        for (id, plugin) in plugins.iter() {
            let result = plugin.health_check().await;
            results.insert(id.clone(), result);
        }

        results
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for PluginRegistry {
    fn clone(&self) -> Self {
        Self {
            plugins: Arc::clone(&self.plugins),
            states: Arc::clone(&self.states),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlugin {
        metadata: PluginMetadata,
        state: PluginState,
    }

    impl TestPlugin {
        fn new() -> Self {
            Self {
                metadata: PluginMetadata::new("test-plugin", "Test Plugin", "1.0.0")
                    .with_capability("testing")
                    .with_description("A test plugin"),
                state: PluginState::Loaded,
            }
        }
    }

    #[async_trait]
    impl Plugin for TestPlugin {
        fn metadata(&self) -> &PluginMetadata {
            &self.metadata
        }

        async fn initialize(&mut self) -> Result<()> {
            self.state = PluginState::Ready;
            Ok(())
        }

        async fn start(&mut self) -> Result<()> {
            self.state = PluginState::Active;
            Ok(())
        }

        async fn stop(&mut self) -> Result<()> {
            self.state = PluginState::Ready;
            Ok(())
        }

        async fn execute(&mut self, _input: PluginInput) -> Result<PluginOutput> {
            Ok(PluginOutput::success().with_data("result", serde_json::json!("test")))
        }

        fn state(&self) -> PluginState {
            self.state
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[tokio::test]
    async fn test_plugin_metadata() {
        let metadata = PluginMetadata::new("test", "Test Plugin", "1.0.0")
            .with_capability("testing")
            .with_description("Test description")
            .with_author("Test Author");

        assert_eq!(metadata.id, "test");
        assert_eq!(metadata.name, "Test Plugin");
        assert_eq!(metadata.version, "1.0.0");
        assert_eq!(metadata.capabilities.len(), 1);
        assert_eq!(metadata.description, "Test description");
        assert_eq!(metadata.author, "Test Author");
    }

    #[tokio::test]
    async fn test_plugin_input_output() {
        let input = PluginInput::new()
            .with_data("key", serde_json::json!("value"))
            .with_context("context_key", "context_value");

        assert_eq!(input.get_data("key"), Some(&serde_json::json!("value")));
        assert_eq!(input.get_context("context_key"), Some(&"context_value".to_string()));

        let output = PluginOutput::success()
            .with_data("result", serde_json::json!(42))
            .with_metric("duration", 100.0);

        assert!(output.success);
        assert_eq!(output.data.get("result"), Some(&serde_json::json!(42)));
        assert_eq!(output.metrics.get("duration"), Some(&100.0));
    }

    #[tokio::test]
    async fn test_plugin_registry() {
        let registry = PluginRegistry::new();
        let plugin = Box::new(TestPlugin::new());

        // Register plugin
        registry.register(plugin).await.unwrap();

        // Get plugin
        let name = registry.get("test-plugin").await.unwrap();
        assert_eq!(name, "Test Plugin");

        // Initialize plugin
        registry.initialize("test-plugin").await.unwrap();
        assert_eq!(registry.get_state("test-plugin").await, Some(PluginState::Ready));

        // Start plugin
        registry.start("test-plugin").await.unwrap();
        assert_eq!(registry.get_state("test-plugin").await, Some(PluginState::Active));

        // Execute plugin
        let input = PluginInput::new();
        let output = registry.execute("test-plugin", input).await.unwrap();
        assert!(output.success);

        // Stop plugin
        registry.stop("test-plugin").await.unwrap();
        assert_eq!(registry.get_state("test-plugin").await, Some(PluginState::Ready));

        // Unregister plugin
        registry.unregister("test-plugin").await.unwrap();
        assert_eq!(registry.get_state("test-plugin").await, Some(PluginState::Unloaded));
    }

    #[tokio::test]
    async fn test_plugin_list() {
        let registry = PluginRegistry::new();
        let plugin1 = Box::new(TestPlugin::new());

        registry.register(plugin1).await.unwrap();

        let list = registry.list().await;
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, "test-plugin");
    }

    #[tokio::test]
    async fn test_duplicate_registration() {
        let registry = PluginRegistry::new();
        let plugin1 = Box::new(TestPlugin::new());
        let plugin2 = Box::new(TestPlugin::new());

        registry.register(plugin1).await.unwrap();
        let result = registry.register(plugin2).await;
        assert!(result.is_err());
    }
}
