//! CI/CD integration module
//!
//! This module provides CI/CD template generation.

use crate::error::ForgeKitError;
use std::path::Path;

/// CI/CD generator
pub struct CICDGenerator;

impl CICDGenerator {
    /// Generate GitHub Actions workflow
    pub async fn generate_github_actions(path: &Path) -> Result<(), ForgeKitError> {
        let workflows_dir = path.join(".github").join("workflows");
        std::fs::create_dir_all(&workflows_dir)?;

        let workflow = r#"name: Build and Test
on: [push, pull_request]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --verbose
      - run: cargo test --verbose
"#;

        std::fs::write(workflows_dir.join("build.yml"), workflow)?;
        Ok(())
    }

    /// Generate GitLab CI configuration
    pub async fn generate_gitlab_ci(path: &Path) -> Result<(), ForgeKitError> {
        let config = r#"stages:
  - build
  - test

build:
  stage: build
  script:
    - cargo build --verbose

test:
  stage: test
  script:
    - cargo test --verbose
"#;

        std::fs::write(path.join(".gitlab-ci.yml"), config)?;
        Ok(())
    }

    /// Generate Jenkins pipeline
    pub async fn generate_jenkins(path: &Path) -> Result<(), ForgeKitError> {
        let pipeline = r#"pipeline {
    agent any
    stages {
        stage('Build') {
            steps {
                sh 'cargo build --verbose'
            }
        }
        stage('Test') {
            steps {
                sh 'cargo test --verbose'
            }
        }
    }
}
"#;

        std::fs::write(path.join("Jenkinsfile"), pipeline)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_generate_github_actions() {
        let temp_dir = TempDir::new().unwrap();
        let result = CICDGenerator::generate_github_actions(temp_dir.path()).await;
        assert!(result.is_ok());
    }
}
