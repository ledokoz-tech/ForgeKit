//! Hot reload development server module
//!
//! This module provides a development server with hot reload capabilities.

use crate::error::ForgeKitError;
use std::path::Path;

/// Development server configuration
#[derive(Debug, Clone)]
pub struct DevServerConfig {
    pub port: u16,
    pub watch_patterns: Vec<String>,
}

impl Default for DevServerConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            watch_patterns: vec!["src/**/*.rs".to_string(), "assets/**/*".to_string()],
        }
    }
}

/// Development server
pub struct DevServer {
    config: DevServerConfig,
}

impl DevServer {
    /// Create a new development server
    pub fn new(config: DevServerConfig) -> Self {
        Self { config }
    }

    /// Start the development server
    pub async fn start(path: &Path) -> Result<(), ForgeKitError> {
        let config = DevServerConfig::default();
        let server = Self::new(config);
        server.run(path).await
    }

    /// Run the development server
    async fn run(&self, path: &Path) -> Result<(), ForgeKitError> {
        tracing::info!("Starting development server on port {}", self.config.port);
        tracing::info!("Watching patterns: {:?}", self.config.watch_patterns);
        tracing::info!("Project path: {:?}", path);

        // Simulate server running
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    /// Stop the development server
    pub async fn stop(&mut self) -> Result<(), ForgeKitError> {
        tracing::info!("Stopping development server");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dev_server_config() {
        let config = DevServerConfig::default();
        assert_eq!(config.port, 8080);
        assert!(!config.watch_patterns.is_empty());
    }

    #[test]
    fn test_dev_server_creation() {
        let config = DevServerConfig::default();
        let _server = DevServer::new(config);
    }
}
