//! Dependency management for ForgeKit projects

use crate::config::{Dependency, ProjectConfig};
use crate::error::ForgeKitError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyRegistry {
    /// Registry of available packages
    packages: HashMap<String, PackageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    /// Package name
    pub name: String,
    /// Available versions
    pub versions: Vec<PackageVersion>,
    /// Package description
    pub description: String,
    /// Package keywords
    pub keywords: Vec<String>,
    /// Package repository
    pub repository: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageVersion {
    /// Version string
    pub version: String,
    /// Release date
    pub released: String,
    /// Compatibility information
    pub compatible_targets: Vec<String>,
    /// Download URL
    pub download_url: String,
}

impl DependencyRegistry {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
        }
    }

    pub fn add_package(&mut self, info: PackageInfo) {
        self.packages.insert(info.name.clone(), info);
    }

    pub fn find_package(&self, name: &str) -> Option<&PackageInfo> {
        self.packages.get(name)
    }

    pub fn search_packages(&self, query: &str) -> Vec<&PackageInfo> {
        self.packages
            .values()
            .filter(|pkg| {
                pkg.name.contains(query)
                    || pkg.description.contains(query)
                    || pkg.keywords.iter().any(|kw| kw.contains(query))
            })
            .collect()
    }

    pub fn resolve_dependencies(
        &self,
        dependencies: &[Dependency],
    ) -> Result<Vec<ResolvedDependency>, ForgeKitError> {
        let mut resolved = Vec::new();

        for dep in dependencies {
            let package = self
                .find_package(&dep.name)
                .ok_or_else(|| ForgeKitError::InvalidConfig(format!("Package not found: {}", dep.name)))?;

            let version = self.resolve_version(package, &dep.version)?;
            resolved.push(ResolvedDependency {
                name: dep.name.clone(),
                version: version.version.clone(),
                download_url: version.download_url.clone(),
            });
        }

        Ok(resolved)
    }

    fn resolve_version<'a>(
        &self,
        package: &'a PackageInfo,
        version_req: &str,
    ) -> Result<&'a PackageVersion, ForgeKitError> {
        // Simple version resolution (exact match for now)
        package
            .versions
            .iter()
            .find(|v| v.version == version_req)
            .ok_or_else(|| {
                ForgeKitError::InvalidConfig(format!(
                    "Version {} not found for package {}",
                    version_req, package.name
                ))
            })
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedDependency {
    pub name: String,
    pub version: String,
    pub download_url: String,
}

/// Manage project dependencies
pub struct DependencyManager {
    registry: DependencyRegistry,
}

impl DependencyManager {
    pub fn new() -> Self {
        Self {
            registry: DependencyRegistry::new(),
        }
    }

    pub fn add_to_registry(&mut self, info: PackageInfo) {
        self.registry.add_package(info);
    }

    pub async fn add_dependency(
        &self,
        project_path: &Path,
        package_name: &str,
        version: &str,
    ) -> Result<(), ForgeKitError> {
        let config_path = project_path.join("forgekit.toml");
        let mut config = ProjectConfig::load(&config_path)?;

        // Check if dependency already exists
        if config
            .dependencies
            .iter()
            .any(|dep| dep.name == package_name)
        {
            return Err(ForgeKitError::InvalidConfig(format!(
                "Dependency {} already exists",
                package_name
            )));
        }

        // Resolve the dependency
        let dep_info = self
            .registry
            .find_package(package_name)
            .ok_or_else(|| ForgeKitError::InvalidConfig(format!("Package {} not found", package_name)))?;

        let _resolved_version = self
            .registry
            .resolve_version(dep_info, version)?;

        // Add to config
        config.dependencies.push(Dependency {
            name: package_name.to_string(),
            version: version.to_string(),
            source: None,
        });

        // Save updated config
        config.save(&config_path)?;

        // Download and install dependency (placeholder)
        self.install_dependency(package_name, version).await?;

        Ok(())
    }

    pub async fn remove_dependency(
        &self,
        project_path: &Path,
        package_name: &str,
    ) -> Result<(), ForgeKitError> {
        let config_path = project_path.join("forgekit.toml");
        let mut config = ProjectConfig::load(&config_path)?;

        // Remove from dependencies
        config.dependencies.retain(|dep| dep.name != package_name);

        // Save updated config
        config.save(&config_path)?;

        // Clean up installed files (placeholder)
        self.uninstall_dependency(package_name).await?;

        Ok(())
    }

    pub async fn update_dependencies(&self, project_path: &Path) -> Result<(), ForgeKitError> {
        let config_path = project_path.join("forgekit.toml");
        let config = ProjectConfig::load(&config_path)?;

        // Update each dependency to latest compatible version
        for dep in &config.dependencies {
            // Placeholder for update logic
            println!("Updating {} to latest version", dep.name);
        }

        Ok(())
    }

    async fn install_dependency(&self, name: &str, version: &str) -> Result<(), ForgeKitError> {
        println!("Installing {} v{}", name, version);
        // Placeholder for actual installation logic
        // This would download and extract the package
        Ok(())
    }

    async fn uninstall_dependency(&self, name: &str) -> Result<(), ForgeKitError> {
        println!("Removing {}", name);
        // Placeholder for actual removal logic
        Ok(())
    }

    pub fn list_available_packages(&self) -> Vec<&PackageInfo> {
        self.registry.packages.values().collect()
    }

    pub fn search_packages(&self, query: &str) -> Vec<&PackageInfo> {
        self.registry.search_packages(query)
    }
}

impl Default for DependencyManager {
    fn default() -> Self {
        Self::new()
    }
}