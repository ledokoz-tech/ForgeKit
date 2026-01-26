//! Package Manager for ForgeKit
//!
//! Handles package installation, updating, and management with support
//! for both local and remote package sources.

use crate::config::{Dependency, ProjectConfig};
use crate::error::ForgeKitError;
use crate::registry::{RegistryClient, RegistryConfig};
use std::fs;
use std::path::{Path, PathBuf};
use tokio::fs as tokio_fs;

/// Package manager for ForgeKit projects
pub struct PackageManager {
    registry_client: RegistryClient,
    project_root: PathBuf,
}

impl PackageManager {
    /// Create a new package manager for a project
    pub fn new(project_root: PathBuf) -> Result<Self, ForgeKitError> {
        let registry_config = RegistryConfig::default();
        let registry_client = RegistryClient::new(registry_config)?;
        
        Ok(Self {
            registry_client,
            project_root,
        })
    }

    /// Add a dependency to the project
    pub async fn add_dependency(
        &self,
        package_name: &str,
        version: &str,
    ) -> Result<(), ForgeKitError> {
        println!("Adding dependency: {} v{}", package_name, version);
        
        // Download the package
        let package_path = self.registry_client.download_package(package_name, version).await?;
        println!("Downloaded package to: {:?}", package_path);
        
        // Extract and install the package
        self.install_package(package_name, version, &package_path).await?;
        
        // Update project configuration
        self.update_project_config(package_name, version).await?;
        
        println!("Successfully added {} v{}", package_name, version);
        Ok(())
    }

    /// Remove a dependency from the project
    pub async fn remove_dependency(&self, package_name: &str) -> Result<(), ForgeKitError> {
        println!("Removing dependency: {}", package_name);
        
        // Remove from project config
        self.remove_from_config(package_name).await?;
        
        // Remove installed files
        let install_path = self.project_root.join("vendor").join(package_name);
        if install_path.exists() {
            tokio_fs::remove_dir_all(&install_path).await?;
            println!("Removed package files from: {:?}", install_path);
        }
        
        println!("Successfully removed {}", package_name);
        Ok(())
    }

    /// Update all dependencies to their latest versions
    pub async fn update_dependencies(&self) -> Result<(), ForgeKitError> {
        println!("Updating dependencies...");
        
        let config_path = self.project_root.join("forgekit.toml");
        let config = ProjectConfig::load(&config_path)?;
        
        for dep in config.dependencies {
            println!("Updating {}...", dep.name);
            // For now, we'll just reinstall the same version
            // In a real implementation, this would resolve to latest compatible version
            self.add_dependency(&dep.name, &dep.version).await?;
        }
        
        println!("Dependencies updated successfully");
        Ok(())
    }

    /// Install a downloaded package
    async fn install_package(
        &self,
        name: &str,
        version: &str,
        package_path: &Path,
    ) -> Result<(), ForgeKitError> {
        let vendor_dir = self.project_root.join("vendor");
        tokio_fs::create_dir_all(&vendor_dir).await?;
        
        let install_path = vendor_dir.join(format!("{}-{}", name, version));
        
        // Extract the tar.gz file (simplified - in reality would use tar crate)
        // For demo purposes, we'll just copy the file
        tokio_fs::copy(package_path, install_path.join("package.tar.gz")).await?;
        
        // Create a basic package structure
        let src_dir = install_path.join("src");
        tokio_fs::create_dir_all(&src_dir).await?;
        
        let lib_rs = r#"//! Auto-generated library file
pub fn hello() {
    println!("Hello from {}!", env!("CARGO_PKG_NAME"));
}
"#;
        tokio_fs::write(src_dir.join("lib.rs"), lib_rs).await?;
        
        println!("Installed package to: {:?}", install_path);
        Ok(())
    }

    /// Update project configuration with new dependency
    async fn update_project_config(
        &self,
        package_name: &str,
        version: &str,
    ) -> Result<(), ForgeKitError> {
        let config_path = self.project_root.join("forgekit.toml");
        let mut config = ProjectConfig::load(&config_path)?;
        
        // Check if dependency already exists
        if config.dependencies.iter().any(|d| d.name == package_name) {
            // Update existing dependency
            for dep in &mut config.dependencies {
                if dep.name == package_name {
                    dep.version = version.to_string();
                    break;
                }
            }
        } else {
            // Add new dependency
            config.dependencies.push(Dependency {
                name: package_name.to_string(),
                version: version.to_string(),
                source: Some("registry".to_string()),
            });
        }
        
        config.save(&config_path)?;
        Ok(())
    }

    /// Remove dependency from project configuration
    async fn remove_from_config(&self, package_name: &str) -> Result<(), ForgeKitError> {
        let config_path = self.project_root.join("forgekit.toml");
        let mut config = ProjectConfig::load(&config_path)?;
        
        config.dependencies.retain(|d| d.name != package_name);
        config.save(&config_path)?;
        
        Ok(())
    }

    /// Search for packages in the registry
    pub async fn search_packages(&self, query: &str) -> Result<Vec<String>, ForgeKitError> {
        let packages = self.registry_client.search_packages(query).await?;
        
        let results: Vec<String> = packages
            .into_iter()
            .map(|pkg| format!("{} - {}", pkg.name, pkg.description))
            .collect();
        
        Ok(results)
    }

    /// List all installed packages
    pub async fn list_installed(&self) -> Result<Vec<String>, ForgeKitError> {
        let vendor_dir = self.project_root.join("vendor");
        if !vendor_dir.exists() {
            return Ok(vec![]);
        }
        
        let mut packages = Vec::new();
        let mut entries = tokio_fs::read_dir(&vendor_dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_dir() {
                let package_name = entry.file_name().to_string_lossy().to_string();
                packages.push(package_name);
            }
        }
        
        Ok(packages)
    }

    /// Update the package registry index
    pub async fn update_registry(&self) -> Result<(), ForgeKitError> {
        println!("Updating package registry...");
        self.registry_client.update_index().await?;
        println!("Registry updated successfully");
        Ok(())
    }

    /// Get package information
    pub async fn get_package_info(
        &self,
        name: &str,
        version: &str,
    ) -> Result<String, ForgeKitError> {
        let info = self.registry_client.get_package_info(name, version).await?;
        Ok(format!(
            "Package: {}\nVersion: {}\nDescription: {}\nRepository: {}\nLicense: {}",
            info.name, info.version, info.description, info.repository, info.license
        ))
    }
}

// Utility functions for global package management

/// Global package cache directory
pub fn get_global_cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("forgekit")
        .join("global-cache")
}

/// Initialize global package cache
pub async fn init_global_cache() -> Result<(), ForgeKitError> {
    let cache_dir = get_global_cache_dir();
    tokio_fs::create_dir_all(&cache_dir).await?;
    Ok(())
}

/// Clean global package cache
pub async fn clean_global_cache() -> Result<(), ForgeKitError> {
    let cache_dir = get_global_cache_dir();
    if cache_dir.exists() {
        tokio_fs::remove_dir_all(&cache_dir).await?;
    }
    init_global_cache().await
}

/// List globally cached packages
pub async fn list_cached_packages() -> Result<Vec<String>, ForgeKitError> {
    let cache_dir = get_global_cache_dir();
    if !cache_dir.exists() {
        return Ok(vec![]);
    }
    
    let mut packages = Vec::new();
    let mut entries = tokio_fs::read_dir(&cache_dir).await?;
    
    while let Some(entry) = entries.next_entry().await? {
        if entry.file_type().await?.is_file() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name.ends_with(".tar.gz") {
                packages.push(file_name);
            }
        }
    }
    
    Ok(packages)
}