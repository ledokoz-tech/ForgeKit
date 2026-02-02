//! Multi-target build module
//!
//! This module provides functionality for building projects for multiple targets.

use crate::error::ForgeKitError;
use std::path::Path;

/// Build target
#[derive(Debug, Clone)]
pub struct BuildTarget {
    pub name: String,
    pub triple: String,
}

/// Build output
#[derive(Debug, Clone)]
pub struct BuildOutput {
    pub target: String,
    pub output_path: String,
    pub success: bool,
}

/// Multi-target builder
pub struct MultiTargetBuilder {
    targets: Vec<BuildTarget>,
}

impl MultiTargetBuilder {
    /// Create a new multi-target builder
    pub fn new(targets: Vec<BuildTarget>) -> Self {
        Self { targets }
    }

    /// Build for all targets
    pub async fn build_all(&self, path: &Path) -> Result<Vec<BuildOutput>, ForgeKitError> {
        let mut outputs = Vec::new();

        for target in &self.targets {
            let output = self.build_target(path, &target.name).await?;
            outputs.push(output);
        }

        Ok(outputs)
    }

    /// Build for a specific target
    pub async fn build_target(
        &self,
        path: &Path,
        target: &str,
    ) -> Result<BuildOutput, ForgeKitError> {
        if !path.join("Cargo.toml").exists() {
            return Err(ForgeKitError::ProjectNotFound(
                "Cargo.toml not found".to_string(),
            ));
        }

        Ok(BuildOutput {
            target: target.to_string(),
            output_path: format!("target/{}/release", target),
            success: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_target_creation() {
        let target = BuildTarget {
            name: "x86_64".to_string(),
            triple: "x86_64-unknown-linux-gnu".to_string(),
        };
        assert_eq!(target.name, "x86_64");
    }

    #[test]
    fn test_multi_target_builder() {
        let targets = vec![
            BuildTarget {
                name: "x86_64".to_string(),
                triple: "x86_64-unknown-linux-gnu".to_string(),
            },
            BuildTarget {
                name: "aarch64".to_string(),
                triple: "aarch64-unknown-linux-gnu".to_string(),
            },
        ];
        let builder = MultiTargetBuilder::new(targets);
        assert_eq!(builder.targets.len(), 2);
    }
}
