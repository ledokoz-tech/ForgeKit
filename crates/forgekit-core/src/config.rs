//! Project configuration handling

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Project configuration stored in forgekit.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Project name
    pub name: String,
    /// Project version
    pub version: String,
    /// Project description
    pub description: Option<String>,
    /// Authors
    pub authors: Vec<String>,
    /// Dependencies
    pub dependencies: Vec<Dependency>,
    /// Build settings
    pub build: BuildConfig,
}

/// Dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// Dependency name
    pub name: String,
    /// Dependency version
    pub version: String,
    /// Optional source (if not from crates.io)
    pub source: Option<String>,
}

/// Build configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Target architecture
    pub target: String,
    /// Optimization level
    pub opt_level: String,
    /// Additional rustc flags
    pub rustflags: Vec<String>,
    /// Output directory
    pub output_dir: String,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: "unnamed".to_string(),
            version: "0.1.0".to_string(),
            description: None,
            authors: vec![],
            dependencies: vec![],
            build: BuildConfig {
                target: "ledokoz".to_string(),
                opt_level: "2".to_string(),
                rustflags: vec![],
                output_dir: "target".to_string(),
            },
        }
    }
}

impl ProjectConfig {
    /// Load configuration from a TOML file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, crate::error::ForgeKitError> {
        let contents = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Save configuration to a TOML file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), crate::error::ForgeKitError> {
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }
}
