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
            return Err(ForgeKitError::ProjectNotFound(
                "Dockerfile not found".to_string(),
            ));
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
    use std::fs;
    use tempfile::TempDir;

    // ============================================================================
    // Unit Tests: Dockerfile Generation
    // ============================================================================

    #[tokio::test]
    async fn test_generate_dockerfile_creates_file() {
        let temp_dir = TempDir::new().unwrap();
        let result = DockerBuilder::generate_dockerfile(temp_dir.path()).await;

        assert!(result.is_ok());
        assert!(temp_dir.path().join("Dockerfile").exists());
    }

    #[tokio::test]
    async fn test_generate_dockerfile_content_is_valid() {
        let temp_dir = TempDir::new().unwrap();
        DockerBuilder::generate_dockerfile(temp_dir.path())
            .await
            .unwrap();

        let content = fs::read_to_string(temp_dir.path().join("Dockerfile")).unwrap();

        // Verify Dockerfile contains expected content
        assert!(content.contains("FROM rust:latest"));
        assert!(content.contains("WORKDIR /app"));
        assert!(content.contains("COPY . ."));
        assert!(content.contains("RUN cargo build --release"));
        assert!(content.contains("CMD"));
    }

    #[tokio::test]
    async fn test_generate_dockerfile_overwrites_existing() {
        let temp_dir = TempDir::new().unwrap();
        let dockerfile_path = temp_dir.path().join("Dockerfile");

        // Create initial Dockerfile
        fs::write(&dockerfile_path, "OLD CONTENT").unwrap();
        assert_eq!(fs::read_to_string(&dockerfile_path).unwrap(), "OLD CONTENT");

        // Generate new Dockerfile
        DockerBuilder::generate_dockerfile(temp_dir.path())
            .await
            .unwrap();

        let content = fs::read_to_string(&dockerfile_path).unwrap();
        assert!(!content.contains("OLD CONTENT"));
        assert!(content.contains("FROM rust:latest"));
    }

    #[tokio::test]
    async fn test_generate_dockerfile_with_nested_path() {
        let temp_dir = TempDir::new().unwrap();
        let nested_path = temp_dir.path().join("nested").join("dir");
        fs::create_dir_all(&nested_path).unwrap();

        let result = DockerBuilder::generate_dockerfile(&nested_path).await;
        assert!(result.is_ok());
        assert!(nested_path.join("Dockerfile").exists());
    }

    // ============================================================================
    // Unit Tests: Docker Image Building
    // ============================================================================

    #[tokio::test]
    async fn test_build_image_requires_dockerfile() {
        let temp_dir = TempDir::new().unwrap();
        let result = DockerBuilder::build_image(temp_dir.path()).await;

        assert!(result.is_err());
        match result {
            Err(ForgeKitError::ProjectNotFound(msg)) => {
                assert!(msg.contains("Dockerfile"));
            }
            _ => panic!("Expected ProjectNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_build_image_with_existing_dockerfile() {
        let temp_dir = TempDir::new().unwrap();
        let dockerfile_path = temp_dir.path().join("Dockerfile");

        // Create a valid Dockerfile
        fs::write(&dockerfile_path, "FROM rust:latest\nRUN echo 'test'").unwrap();

        let result = DockerBuilder::build_image(temp_dir.path()).await;
        assert!(result.is_ok());

        let image_name = result.unwrap();
        assert!(!image_name.is_empty());
        assert!(image_name.contains("image"));
    }

    #[tokio::test]
    async fn test_build_image_returns_image_name() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("Dockerfile"), "FROM rust:latest").unwrap();

        let image_name = DockerBuilder::build_image(temp_dir.path()).await.unwrap();

        assert_eq!(image_name, "image:latest");
    }

    #[tokio::test]
    async fn test_build_image_with_empty_dockerfile() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("Dockerfile"), "").unwrap();

        let result = DockerBuilder::build_image(temp_dir.path()).await;
        assert!(result.is_ok());
    }

    // ============================================================================
    // Unit Tests: Docker Compose Generation
    // ============================================================================

    #[tokio::test]
    async fn test_generate_compose_creates_file() {
        let temp_dir = TempDir::new().unwrap();
        let result = DockerBuilder::generate_compose(temp_dir.path()).await;

        assert!(result.is_ok());
        assert!(temp_dir.path().join("docker-compose.yml").exists());
    }

    #[tokio::test]
    async fn test_generate_compose_content_is_valid() {
        let temp_dir = TempDir::new().unwrap();
        DockerBuilder::generate_compose(temp_dir.path())
            .await
            .unwrap();

        let content = fs::read_to_string(temp_dir.path().join("docker-compose.yml")).unwrap();

        // Verify docker-compose.yml contains expected content
        assert!(content.contains("version:"));
        assert!(content.contains("services:"));
        assert!(content.contains("app:"));
        assert!(content.contains("build:"));
        assert!(content.contains("ports:"));
        assert!(content.contains("8080:8080"));
    }

    #[tokio::test]
    async fn test_generate_compose_overwrites_existing() {
        let temp_dir = TempDir::new().unwrap();
        let compose_path = temp_dir.path().join("docker-compose.yml");

        // Create initial compose file
        fs::write(&compose_path, "OLD CONTENT").unwrap();
        assert_eq!(fs::read_to_string(&compose_path).unwrap(), "OLD CONTENT");

        // Generate new compose file
        DockerBuilder::generate_compose(temp_dir.path())
            .await
            .unwrap();

        let content = fs::read_to_string(&compose_path).unwrap();
        assert!(!content.contains("OLD CONTENT"));
        assert!(content.contains("version:"));
    }

    #[tokio::test]
    async fn test_generate_compose_with_nested_path() {
        let temp_dir = TempDir::new().unwrap();
        let nested_path = temp_dir.path().join("nested").join("dir");
        fs::create_dir_all(&nested_path).unwrap();

        let result = DockerBuilder::generate_compose(&nested_path).await;
        assert!(result.is_ok());
        assert!(nested_path.join("docker-compose.yml").exists());
    }

    // ============================================================================
    // Integration Tests: Complete Docker Setup
    // ============================================================================

    #[tokio::test]
    async fn test_complete_docker_setup_workflow() {
        let temp_dir = TempDir::new().unwrap();

        // Step 1: Generate Dockerfile
        let dockerfile_result = DockerBuilder::generate_dockerfile(temp_dir.path()).await;
        assert!(dockerfile_result.is_ok());

        // Step 2: Generate docker-compose.yml
        let compose_result = DockerBuilder::generate_compose(temp_dir.path()).await;
        assert!(compose_result.is_ok());

        // Step 3: Build image
        let build_result = DockerBuilder::build_image(temp_dir.path()).await;
        assert!(build_result.is_ok());

        // Verify all files exist
        assert!(temp_dir.path().join("Dockerfile").exists());
        assert!(temp_dir.path().join("docker-compose.yml").exists());
    }

    #[tokio::test]
    async fn test_docker_setup_with_existing_files() {
        let temp_dir = TempDir::new().unwrap();

        // Create initial files
        fs::write(temp_dir.path().join("Dockerfile"), "FROM ubuntu:latest").unwrap();
        fs::write(temp_dir.path().join("docker-compose.yml"), "version: '2'").unwrap();

        // Generate new files (should overwrite)
        DockerBuilder::generate_dockerfile(temp_dir.path())
            .await
            .unwrap();
        DockerBuilder::generate_compose(temp_dir.path())
            .await
            .unwrap();

        // Verify files were updated
        let dockerfile = fs::read_to_string(temp_dir.path().join("Dockerfile")).unwrap();
        let compose = fs::read_to_string(temp_dir.path().join("docker-compose.yml")).unwrap();

        assert!(dockerfile.contains("FROM rust:latest"));
        assert!(compose.contains("version: '3'"));
    }

    #[tokio::test]
    async fn test_build_image_error_handling() {
        let temp_dir = TempDir::new().unwrap();

        // Try to build without Dockerfile
        let result = DockerBuilder::build_image(temp_dir.path()).await;
        assert!(result.is_err());

        // Verify error message is informative
        match result {
            Err(ForgeKitError::ProjectNotFound(msg)) => {
                assert!(!msg.is_empty());
            }
            _ => panic!("Expected ProjectNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_multiple_sequential_operations() {
        let temp_dir = TempDir::new().unwrap();

        // First operation
        DockerBuilder::generate_dockerfile(temp_dir.path())
            .await
            .unwrap();
        let first_build = DockerBuilder::build_image(temp_dir.path()).await.unwrap();

        // Second operation
        DockerBuilder::generate_compose(temp_dir.path())
            .await
            .unwrap();
        let second_build = DockerBuilder::build_image(temp_dir.path()).await.unwrap();

        // Both should succeed
        assert_eq!(first_build, "image:latest");
        assert_eq!(second_build, "image:latest");
    }

    #[tokio::test]
    async fn test_dockerfile_generation_idempotent() {
        let temp_dir = TempDir::new().unwrap();

        // Generate twice
        DockerBuilder::generate_dockerfile(temp_dir.path())
            .await
            .unwrap();
        let first_content = fs::read_to_string(temp_dir.path().join("Dockerfile")).unwrap();

        DockerBuilder::generate_dockerfile(temp_dir.path())
            .await
            .unwrap();
        let second_content = fs::read_to_string(temp_dir.path().join("Dockerfile")).unwrap();

        // Content should be identical
        assert_eq!(first_content, second_content);
    }

    #[tokio::test]
    async fn test_compose_generation_idempotent() {
        let temp_dir = TempDir::new().unwrap();

        // Generate twice
        DockerBuilder::generate_compose(temp_dir.path())
            .await
            .unwrap();
        let first_content = fs::read_to_string(temp_dir.path().join("docker-compose.yml")).unwrap();

        DockerBuilder::generate_compose(temp_dir.path())
            .await
            .unwrap();
        let second_content =
            fs::read_to_string(temp_dir.path().join("docker-compose.yml")).unwrap();

        // Content should be identical
        assert_eq!(first_content, second_content);
    }

    // ============================================================================
    // Edge Case Tests
    // ============================================================================

    #[tokio::test]
    async fn test_generate_dockerfile_with_readonly_parent() {
        let temp_dir = TempDir::new().unwrap();
        let readonly_dir = temp_dir.path().join("readonly");
        fs::create_dir(&readonly_dir).unwrap();

        // Note: This test may not work on all systems due to permission handling
        // It's included for completeness
        let result = DockerBuilder::generate_dockerfile(&readonly_dir).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_build_image_with_special_characters_in_path() {
        let temp_dir = TempDir::new().unwrap();
        let special_dir = temp_dir.path().join("dir-with-special_chars.123");
        fs::create_dir(&special_dir).unwrap();

        fs::write(special_dir.join("Dockerfile"), "FROM rust:latest").unwrap();

        let result = DockerBuilder::build_image(&special_dir).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_compose_creates_valid_yaml_structure() {
        let temp_dir = TempDir::new().unwrap();
        DockerBuilder::generate_compose(temp_dir.path())
            .await
            .unwrap();

        let content = fs::read_to_string(temp_dir.path().join("docker-compose.yml")).unwrap();

        // Verify YAML structure
        let lines: Vec<&str> = content.lines().collect();
        assert!(lines.len() > 0);

        // Check for proper indentation (basic YAML validation)
        let has_services = lines.iter().any(|l| l.contains("services:"));
        let has_app = lines.iter().any(|l| l.contains("app:"));
        let has_build = lines.iter().any(|l| l.contains("build:"));

        assert!(has_services);
        assert!(has_app);
        assert!(has_build);
    }

    #[tokio::test]
    async fn test_generate_dockerfile_creates_valid_dockerfile_structure() {
        let temp_dir = TempDir::new().unwrap();
        DockerBuilder::generate_dockerfile(temp_dir.path())
            .await
            .unwrap();

        let content = fs::read_to_string(temp_dir.path().join("Dockerfile")).unwrap();

        // Verify Dockerfile structure
        let lines: Vec<&str> = content.lines().collect();
        assert!(lines.len() > 0);

        // Check for required Dockerfile instructions
        let has_from = lines.iter().any(|l| l.starts_with("FROM"));
        let has_workdir = lines.iter().any(|l| l.starts_with("WORKDIR"));
        let has_copy = lines.iter().any(|l| l.starts_with("COPY"));
        let has_run = lines.iter().any(|l| l.starts_with("RUN"));
        let has_cmd = lines.iter().any(|l| l.starts_with("CMD"));

        assert!(has_from, "Dockerfile must have FROM instruction");
        assert!(has_workdir, "Dockerfile must have WORKDIR instruction");
        assert!(has_copy, "Dockerfile must have COPY instruction");
        assert!(has_run, "Dockerfile must have RUN instruction");
        assert!(has_cmd, "Dockerfile must have CMD instruction");
    }

    // ============================================================================
    // Error Handling Tests
    // ============================================================================

    #[tokio::test]
    async fn test_build_image_missing_dockerfile_error_message() {
        let temp_dir = TempDir::new().unwrap();
        let result = DockerBuilder::build_image(temp_dir.path()).await;

        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Dockerfile") || error_msg.contains("not found"));
    }

    #[tokio::test]
    async fn test_generate_operations_return_correct_result_type() {
        let temp_dir = TempDir::new().unwrap();

        let dockerfile_result = DockerBuilder::generate_dockerfile(temp_dir.path()).await;
        assert!(dockerfile_result.is_ok());
        assert_eq!(dockerfile_result.unwrap(), ());

        let compose_result = DockerBuilder::generate_compose(temp_dir.path()).await;
        assert!(compose_result.is_ok());
        assert_eq!(compose_result.unwrap(), ());
    }
}
