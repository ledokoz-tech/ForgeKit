//! Project validation module
//!
//! This module provides functionality to validate ForgeKit projects,
//! including configuration files, directory structure, and dependencies.

use crate::config::Config;
use crate::error::ForgeKitError;
use std::path::Path;
use walkdir::WalkDir;

/// Validation report containing results of project validation
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// Whether the project is valid
    pub is_valid: bool,
    /// List of validation errors
    pub errors: Vec<String>,
    /// List of validation warnings
    pub warnings: Vec<String>,
}

impl ValidationReport {
    /// Create a new validation report
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Add an error to the report
    pub fn add_error(&mut self, error: String) {
        self.is_valid = false;
        self.errors.push(error);
    }

    /// Add a warning to the report
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Project validator for validating ForgeKit projects
pub struct ProjectValidator;

impl ProjectValidator {
    /// Validate a project at the given path
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the project directory
    ///
    /// # Returns
    ///
    /// A `ValidationReport` containing validation results
    pub async fn validate_project(path: &Path) -> Result<ValidationReport, ForgeKitError> {
        let mut report = ValidationReport::new();

        // Validate configuration file
        Self::validate_config(path, &mut report).await?;

        // Validate directory structure
        Self::validate_structure(path, &mut report)?;

        // Validate dependencies
        Self::validate_dependencies(path, &mut report).await?;

        Ok(report)
    }

    /// Validate the forgekit.toml configuration file
    async fn validate_config(path: &Path, report: &mut ValidationReport) -> Result<(), ForgeKitError> {
        let config_path = path.join("forgekit.toml");

        if !config_path.exists() {
            report.add_error("forgekit.toml not found".to_string());
            return Ok(());
        }

        match Config::load(&config_path).await {
            Ok(config) => {
                // Validate required fields
                if config.name.is_empty() {
                    report.add_error("Project name is required in forgekit.toml".to_string());
                }
                if config.version.is_empty() {
                    report.add_error("Project version is required in forgekit.toml".to_string());
                }
            }
            Err(e) => {
                report.add_error(format!("Invalid forgekit.toml: {}", e));
            }
        }

        Ok(())
    }

    /// Validate the project directory structure
    fn validate_structure(path: &Path, report: &mut ValidationReport) -> Result<(), ForgeKitError> {
        // Check for required directories
        let required_dirs = vec!["src", "assets"];

        for dir in required_dirs {
            let dir_path = path.join(dir);
            if !dir_path.exists() {
                report.add_warning(format!("Recommended directory '{}' not found", dir));
            } else if !dir_path.is_dir() {
                report.add_error(format!("'{}' exists but is not a directory", dir));
            }
        }

        Ok(())
    }

    /// Validate project dependencies
    async fn validate_dependencies(
        path: &Path,
        report: &mut ValidationReport,
    ) -> Result<(), ForgeKitError> {
        let cargo_toml = path.join("Cargo.toml");

        if !cargo_toml.exists() {
            report.add_warning("Cargo.toml not found - dependencies cannot be validated".to_string());
            return Ok(());
        }

        // Try to parse Cargo.toml to ensure it's valid
        let content = std::fs::read_to_string(&cargo_toml)?;
        match toml::from_str::<toml::Value>(&content) {
            Ok(_) => {
                // Cargo.toml is valid
            }
            Err(e) => {
                report.add_error(format!("Invalid Cargo.toml: {}", e));
            }
        }

        Ok(())
    }

    /// Validate only the configuration
    pub async fn validate_config_only(config: &Config) -> Result<(), ForgeKitError> {
        if config.name.is_empty() {
            return Err(ForgeKitError::InvalidConfig(
                "Project name is required".to_string(),
            ));
        }

        if config.version.is_empty() {
            return Err(ForgeKitError::InvalidConfig(
                "Project version is required".to_string(),
            ));
        }

        Ok(())
    }

    /// Get the count of source files in the project
    pub fn count_source_files(path: &Path) -> Result<usize, ForgeKitError> {
        let src_path = path.join("src");
        if !src_path.exists() {
            return Ok(0);
        }

        let count = WalkDir::new(&src_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .map(|ext| ext == "rs")
                    .unwrap_or(false)
            })
            .count();

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_validation_report_creation() {
        let mut report = ValidationReport::new();
        assert!(report.is_valid);
        assert!(report.errors.is_empty());
        assert!(report.warnings.is_empty());

        report.add_error("Test error".to_string());
        assert!(!report.is_valid);
        assert_eq!(report.errors.len(), 1);

        report.add_warning("Test warning".to_string());
        assert_eq!(report.warnings.len(), 1);
    }

    #[tokio::test]
    async fn test_validate_project_missing_config() {
        let temp_dir = TempDir::new().unwrap();
        let report = ProjectValidator::validate_project(temp_dir.path())
            .await
            .unwrap();

        assert!(!report.is_valid);
        assert!(report.errors.iter().any(|e| e.contains("forgekit.toml")));
    }

    #[tokio::test]
    async fn test_validate_project_missing_directories() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("forgekit.toml");
        fs::write(
            &config_path,
            r#"
name = "test-project"
version = "0.1.0"
"#,
        )
        .unwrap();

        let report = ProjectValidator::validate_project(temp_dir.path())
            .await
            .unwrap();

        assert!(report.is_valid); // Warnings don't make it invalid
        assert!(report.warnings.iter().any(|w| w.contains("src")));
    }

    #[tokio::test]
    async fn test_validate_project_valid() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("forgekit.toml");
        fs::write(
            &config_path,
            r#"
name = "test-project"
version = "0.1.0"
"#,
        )
        .unwrap();

        fs::create_dir(temp_dir.path().join("src")).unwrap();
        fs::create_dir(temp_dir.path().join("assets")).unwrap();

        let report = ProjectValidator::validate_project(temp_dir.path())
            .await
            .unwrap();

        assert!(report.is_valid);
        assert!(report.errors.is_empty());
    }

    #[test]
    fn test_count_source_files() {
        let temp_dir = TempDir::new().unwrap();
        let src_path = temp_dir.path().join("src");
        fs::create_dir(&src_path).unwrap();

        fs::write(src_path.join("main.rs"), "fn main() {}").unwrap();
        fs::write(src_path.join("lib.rs"), "pub fn test() {}").unwrap();

        let count = ProjectValidator::count_source_files(temp_dir.path()).unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_count_source_files_no_src_dir() {
        let temp_dir = TempDir::new().unwrap();
        let count = ProjectValidator::count_source_files(temp_dir.path()).unwrap();
        assert_eq!(count, 0);
    }
}
