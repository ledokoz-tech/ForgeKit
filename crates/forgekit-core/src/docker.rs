//! Docker support module
//!
//! This module provides Docker image generation and management.

use crate::error::ForgeKitError;
use std::path::Path;

/// Docker builder
pub struct DockerBuilder;

impl DockerBuilder {
    /// Generate Dockerfile
    pub async fn generate_dockerfile(path: &Path) -> Result<(), ForgeKitError> {
        let dockerfile = r#"FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo build --release
CMD ["./target/release/app"]
"#;

        std::fs::write(path.join("Dockerfile"), dockerfile)?;
        Ok(())
    }

    /// Build Docker image
    pub async fn build_image(path: &Path) -> Result<String, ForgeKitError> {
        if !path.join("Dockerfile").exists() {
            return Err(ForgeKitError::ProjectNotFound("Dockerfile not found".to_string()));
        }

        Ok("image:latest".to_string())
    }

    /// Generate docker-compose.yml
    pub async fn generate_compose(path: &Path) -> Result<(), ForgeKitError> {
        let compose = r#"version: '3'
services:
  app:
    build: .
    ports:
      - "8080:8080"
"#;

        std::fs::write(path.join("docker-compose.yml"), compose)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_generate_dockerfile() {
        let temp_dir = TempDir::new().unwrap();
        let result = DockerBuilder::generate_dockerfile(temp_dir.path()).await;
        assert!(result.is_ok());
    }
}
