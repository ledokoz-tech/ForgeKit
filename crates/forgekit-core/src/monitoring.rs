//! Monitoring and logging integration module
//!
//! This module provides monitoring and logging setup.

use crate::error::ForgeKitError;
use std::path::Path;

/// Monitoring setup
pub struct MonitoringSetup;

impl MonitoringSetup {
    /// Generate logging configuration
    pub async fn generate_logging_config(path: &Path) -> Result<(), ForgeKitError> {
        let config = r#"[logging]
level = "info"
format = "json"
"#;

        std::fs::write(path.join("logging.toml"), config)?;
        Ok(())
    }

    /// Setup monitoring
    pub async fn setup_monitoring(provider: &str) -> Result<(), ForgeKitError> {
        tracing::info!("Setting up monitoring with provider: {}", provider);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_generate_logging_config() {
        let temp_dir = TempDir::new().unwrap();
        let result = MonitoringSetup::generate_logging_config(temp_dir.path()).await;
        assert!(result.is_ok());
    }
}
