//! Plugin system module
//!
//! This module provides a plugin system for extending ForgeKit functionality.

use crate::error::ForgeKitError;
use std::collections::HashMap;
use std::path::Path;

/// Build context passed to plugins
#[derive(Debug, Clone)]
pub struct BuildContext {
    pub project_path: String,
    pub target: String,
}

/// Package context passed to plugins
#[derive(Debug, Clone)]
pub struct PackageContext {
    pub project_path: String,
    pub output_path: String,
}

/// Plugin trait that all plugins must implement
pub trait Plugin: Send + Sync {
    /// Get plugin name
    fn name(&self) -> &str;

    /// Get plugin version
    fn version(&self) -> &str;

    /// Called before build starts
    fn on_pre_build(&self, _context: &BuildContext) -> Result<(), ForgeKitError> {
        Ok(())
    }

    /// Called after build completes
    fn on_post_build(&self, _context: &BuildContext) -> Result<(), ForgeKitError> {
        Ok(())
    }

    /// Called during packaging
    fn on_package(&self, _context: &PackageContext) -> Result<(), ForgeKitError> {
        Ok(())
    }
}

/// Plugin manager for loading and managing plugins
pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    registry: HashMap<String, String>,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            registry: HashMap::new(),
        }
    }

    /// Register a plugin
    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        self.registry
            .insert(plugin.name().to_string(), plugin.version().to_string());
        self.plugins.push(plugin);
    }

    /// Get list of registered plugins
    pub fn list_plugins(&self) -> Vec<(String, String)> {
        self.registry
            .iter()
            .map(|(name, version)| (name.clone(), version.clone()))
            .collect()
    }

    /// Call pre-build hooks
    pub fn call_pre_build(&self, context: &BuildContext) -> Result<(), ForgeKitError> {
        for plugin in &self.plugins {
            plugin.on_pre_build(context)?;
        }
        Ok(())
    }

    /// Call post-build hooks
    pub fn call_post_build(&self, context: &BuildContext) -> Result<(), ForgeKitError> {
        for plugin in &self.plugins {
            plugin.on_post_build(context)?;
        }
        Ok(())
    }

    /// Call package hooks
    pub fn call_package(&self, context: &PackageContext) -> Result<(), ForgeKitError> {
        for plugin in &self.plugins {
            plugin.on_package(context)?;
        }
        Ok(())
    }

    /// Get plugin count
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Built-in example plugin
pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn name(&self) -> &str {
        "example-plugin"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn on_pre_build(&self, context: &BuildContext) -> Result<(), ForgeKitError> {
        tracing::info!(
            "Example plugin: pre-build hook for {}",
            context.project_path
        );
        Ok(())
    }

    fn on_post_build(&self, context: &BuildContext) -> Result<(), ForgeKitError> {
        tracing::info!(
            "Example plugin: post-build hook for {}",
            context.project_path
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlugin;

    impl Plugin for TestPlugin {
        fn name(&self) -> &str {
            "test-plugin"
        }

        fn version(&self) -> &str {
            "1.0.0"
        }
    }

    #[test]
    fn test_plugin_manager_creation() {
        let manager = PluginManager::new();
        assert_eq!(manager.plugin_count(), 0);
    }

    #[test]
    fn test_register_plugin() {
        let mut manager = PluginManager::new();
        manager.register(Box::new(TestPlugin));
        assert_eq!(manager.plugin_count(), 1);
    }

    #[test]
    fn test_list_plugins() {
        let mut manager = PluginManager::new();
        manager.register(Box::new(TestPlugin));
        let plugins = manager.list_plugins();
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].0, "test-plugin");
        assert_eq!(plugins[0].1, "1.0.0");
    }

    #[test]
    fn test_pre_build_hook() {
        let manager = PluginManager::new();
        let context = BuildContext {
            project_path: "/test".to_string(),
            target: "debug".to_string(),
        };
        assert!(manager.call_pre_build(&context).is_ok());
    }
}
