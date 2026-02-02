//! ForgeKit Core Library
//!
//! This crate provides the core functionality for building, packaging,
//! and managing `.mox` applications for Ledokoz OS.

pub mod analytics;
pub mod asset_optimizer;
pub mod audit;
pub mod builder;
pub mod cache;
pub mod cicd;
pub mod config;
pub mod dependencies;
pub mod dev_server;
pub mod doc_generator;
pub mod docker;
pub mod env_manager;
pub mod error;
pub mod i18n;
pub mod migrations;
pub mod monitoring;
pub mod multi_target;
pub mod openapi;
pub mod package_manager;
pub mod packager;
pub mod plugin;
pub mod profiler;
pub mod project;
pub mod registry;
pub mod secrets;
pub mod templates;
pub mod testing;
pub mod validator;
pub mod version_manager;

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

    /// Initialize a new project with a specific template
    pub async fn init_project_with_template(
        &self,
        name: &str,
        path: &std::path::Path,
        template: templates::TemplateType,
    ) -> Result<(), error::ForgeKitError> {
        templates::generate_from_template(name, template, path).await
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
