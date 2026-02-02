//! OpenAPI/Swagger documentation module
//!
//! This module provides OpenAPI specification generation.

use crate::error::ForgeKitError;
use std::path::Path;

/// OpenAPI generator
pub struct OpenAPIGenerator;

impl OpenAPIGenerator {
    /// Generate OpenAPI specification
    pub async fn generate_spec(path: &Path) -> Result<String, ForgeKitError> {
        if !path.join("Cargo.toml").exists() {
            return Err(ForgeKitError::ProjectNotFound(
                "Cargo.toml not found".to_string(),
            ));
        }

        let spec = r#"{
  "openapi": "3.0.0",
  "info": {
    "title": "API",
    "version": "1.0.0"
  },
  "paths": {}
}
"#;

        Ok(spec.to_string())
    }

    /// Generate interactive documentation
    pub async fn generate_docs(path: &Path) -> Result<std::path::PathBuf, ForgeKitError> {
        let docs_dir = path.join("api-docs");
        std::fs::create_dir_all(&docs_dir)?;

        let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>API Documentation</title>
</head>
<body>
    <h1>API Documentation</h1>
</body>
</html>
"#;

        std::fs::write(docs_dir.join("index.html"), html)?;
        Ok(docs_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_generate_spec() {
        let temp_dir = TempDir::new().unwrap();
        let cargo_toml = temp_dir.path().join("Cargo.toml");
        std::fs::write(&cargo_toml, "[package]\nname = \"test\"").unwrap();

        let result = OpenAPIGenerator::generate_spec(temp_dir.path()).await;
        assert!(result.is_ok());
    }
}
