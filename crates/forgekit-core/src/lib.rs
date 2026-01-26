//! ForgeKit Core Library
//!
//! This crate provides the core functionality for building, packaging,
//! and managing `.mox` applications for Ledokoz OS.

pub mod builder;
pub mod config;
pub mod error;
pub mod packager;
pub mod project;

/// The main ForgeKit library
pub struct ForgeKit;

impl ForgeKit {
    /// Create a new ForgeKit instance
    pub fn new() -> Self {
        Self
    }

    /// Initialize a new project
    pub async fn init_project(
        &self,
        name: &str,
        path: &std::path::Path,
    ) -> Result<(), error::ForgeKitError> {
        project::init(name, path).await
    }

    /// Build a project
    pub async fn build_project(&self, path: &std::path::Path) -> Result<(), error::ForgeKitError> {
        builder::build(path).await
    }

    /// Package a project into a .mox file
    pub async fn package_project(
        &self,
        path: &std::path::Path,
    ) -> Result<std::path::PathBuf, error::ForgeKitError> {
        packager::package(path).await
    }
}

impl Default for ForgeKit {
    fn default() -> Self {
        Self::new()
    }
}
