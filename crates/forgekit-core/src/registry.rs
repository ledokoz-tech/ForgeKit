//! ForgeKit Package Registry System
//!
//! This module provides functionality for managing a custom package registry
//! that can download packages from GitHub repositories, similar to Cargo's
//! registry but tailored for ForgeKit's ecosystem.

use crate::error::ForgeKitError;
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tokio::fs as tokio_fs;

/// Registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    /// Base URL for the registry
    pub base_url: String,
    /// GitHub token for authenticated requests (optional)
    pub github_token: Option<String>,
    /// Cache directory
    pub cache_dir: PathBuf,
    /// Index directory
    pub index_dir: PathBuf,
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            base_url: "https://github.com".to_string(),
            github_token: None,
            cache_dir: dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("forgekit")
                .join("cache"),
            index_dir: dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("forgekit")
                .join("index"),
        }
    }
}

/// Package metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    /// Package name
    pub name: String,
    /// Package version
    pub version: String,
    /// Package description
    pub description: String,
    /// Package authors
    pub authors: Vec<String>,
    /// Repository URL
    pub repository: String,
    /// License
    pub license: String,
    /// Keywords
    pub keywords: Vec<String>,
    /// Categories
    pub categories: Vec<String>,
    /// Dependencies
    pub dependencies: Vec<DependencySpec>,
    /// Build targets
    pub targets: Vec<String>,
    /// Release date
    pub release_date: String,
    /// Download count
    pub downloads: u64,
}

/// Dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencySpec {
    /// Dependency name
    pub name: String,
    /// Version requirement
    pub version: String,
    /// Optional dependency
    pub optional: bool,
    /// Development dependency
    pub dev: bool,
}

/// Package index entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    /// Package name
    pub name: String,
    /// Available versions
    pub versions: HashMap<String, VersionInfo>,
    /// Latest version
    pub latest: String,
}

/// Version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    /// Version string
    pub version: String,
    /// Git tag or commit
    pub git_ref: String,
    /// Archive URL
    pub archive_url: String,
    /// Published date
    pub published: String,
    /// Package checksum
    pub checksum: String,
}

/// ForgeKit Registry Client
pub struct RegistryClient {
    config: RegistryConfig,
    client: reqwest::Client,
}

impl RegistryClient {
    /// Create a new registry client
    pub fn new(config: RegistryConfig) -> Result<Self, ForgeKitError> {
        let mut builder = reqwest::Client::builder();

        if let Some(token) = &config.github_token {
            builder = builder.default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                let auth_value = format!("Bearer {}", token);
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&auth_value).unwrap(),
                );
                headers
            });
        }

        let client = builder.build()?;

        // Ensure directories exist
        fs::create_dir_all(&config.cache_dir)?;
        fs::create_dir_all(&config.index_dir)?;

        Ok(Self { config, client })
    }

    /// Search for packages
    pub async fn search_packages(
        &self,
        query: &str,
    ) -> Result<Vec<PackageMetadata>, ForgeKitError> {
        // First check local index
        let local_results = self.search_local_index(query).await?;
        if !local_results.is_empty() {
            return Ok(local_results);
        }

        // Fall back to GitHub search
        self.search_github_packages(query).await
    }

    /// Search local package index
    async fn search_local_index(&self, query: &str) -> Result<Vec<PackageMetadata>, ForgeKitError> {
        let mut results = Vec::new();
        let index_path = self.config.index_dir.join("packages.json");

        if index_path.exists() {
            let content = fs::read_to_string(&index_path)?;
            let index: HashMap<String, IndexEntry> = serde_json::from_str(&content)?;

            for (name, entry) in index {
                if name.contains(query)
                    || entry.versions.values().any(|v| v.version.contains(query))
                {
                    // Convert to PackageMetadata (simplified)
                    results.push(PackageMetadata {
                        name: name.clone(),
                        version: entry.latest.clone(),
                        description: format!("Package {}", name),
                        authors: vec![],
                        repository: format!("{}/{}", self.config.base_url, name),
                        license: "MIT".to_string(),
                        keywords: vec![],
                        categories: vec![],
                        dependencies: vec![],
                        targets: vec!["ledokoz".to_string()],
                        release_date: entry
                            .versions
                            .get(&entry.latest)
                            .map(|v| v.published.clone())
                            .unwrap_or_default(),
                        downloads: 0,
                    });
                }
            }
        }

        Ok(results)
    }

    /// Search GitHub for ForgeKit packages
    async fn search_github_packages(
        &self,
        query: &str,
    ) -> Result<Vec<PackageMetadata>, ForgeKitError> {
        let search_url = format!(
            "https://api.github.com/search/repositories?q={}+topic:forgekit-package&sort=stars&order=desc",
            query
        );

        let response = self.client.get(&search_url).send().await?;
        let json: serde_json::Value = response.json().await?;

        let mut packages = Vec::new();

        if let Some(items) = json["items"].as_array() {
            for item in items.iter().take(20) {
                // Extract package info
                let name = item["name"].as_str().unwrap_or("unknown").to_string();
                let full_name = item["full_name"].as_str().unwrap_or("").to_string();
                let description = item["description"].as_str().unwrap_or("").to_string();
                let html_url = item["html_url"].as_str().unwrap_or("").to_string();

                packages.push(PackageMetadata {
                    name,
                    version: "0.1.0".to_string(), // Default version
                    description,
                    authors: vec![full_name.split('/').next().unwrap_or("").to_string()],
                    repository: html_url,
                    license: "MIT".to_string(),
                    keywords: vec!["forgekit".to_string()],
                    categories: vec![],
                    dependencies: vec![],
                    targets: vec!["ledokoz".to_string()],
                    release_date: chrono::Utc::now().to_rfc3339(),
                    downloads: 0,
                });
            }
        }

        Ok(packages)
    }

    /// Download a package
    pub async fn download_package(
        &self,
        name: &str,
        version: &str,
    ) -> Result<PathBuf, ForgeKitError> {
        // Check if already cached
        let cache_path = self
            .config
            .cache_dir
            .join(format!("{}-{}.tar.gz", name, version));
        if cache_path.exists() {
            return Ok(cache_path);
        }

        // Get package info
        let package_info = self.get_package_info(name, version).await?;

        // Download from GitHub
        let download_url = format!(
            "https://github.com/{}/archive/refs/tags/v{}.tar.gz",
            name.replace("forgekit-", ""),
            version
        );

        let response = self.client.get(&download_url).send().await?;
        let bytes = response.bytes().await?;

        // Save to cache
        tokio_fs::write(&cache_path, bytes).await?;

        Ok(cache_path)
    }

    /// Get package information
    pub async fn get_package_info(
        &self,
        name: &str,
        version: &str,
    ) -> Result<PackageMetadata, ForgeKitError> {
        // Try to get from local index first
        let index_path = self.config.index_dir.join("packages.json");
        if index_path.exists() {
            let content = fs::read_to_string(&index_path)?;
            let index: HashMap<String, IndexEntry> = serde_json::from_str(&content)?;

            if let Some(entry) = index.get(name) {
                if let Some(version_info) = entry.versions.get(version) {
                    return Ok(PackageMetadata {
                        name: name.to_string(),
                        version: version.to_string(),
                        description: format!("Package {}", name),
                        authors: vec![],
                        repository: format!("{}/{}", self.config.base_url, name),
                        license: "MIT".to_string(),
                        keywords: vec![],
                        categories: vec![],
                        dependencies: vec![],
                        targets: vec!["ledokoz".to_string()],
                        release_date: version_info.published.clone(),
                        downloads: 0,
                    });
                }
            }
        }

        // Fallback to GitHub API
        let api_url = format!(
            "https://api.github.com/repos/{}/releases/tags/v{}",
            name.replace("forgekit-", ""),
            version
        );

        let response = self.client.get(&api_url).send().await?;
        let release_info: serde_json::Value = response.json().await?;

        Ok(PackageMetadata {
            name: name.to_string(),
            version: version.to_string(),
            description: release_info["body"]
                .as_str()
                .unwrap_or("No description")
                .to_string(),
            authors: vec![name.split('/').next().unwrap_or("").to_string()],
            repository: format!("https://github.com/{}", name),
            license: "MIT".to_string(),
            keywords: vec!["forgekit".to_string()],
            categories: vec![],
            dependencies: vec![],
            targets: vec!["ledokoz".to_string()],
            release_date: release_info["published_at"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            downloads: 0,
        })
    }

    /// Update local package index
    pub async fn update_index(&self) -> Result<(), ForgeKitError> {
        // This would typically fetch from a central registry
        // For now, we'll create a basic index
        let index_path = self.config.index_dir.join("packages.json");

        let mut index = HashMap::new();

        // Add some sample packages to the index
        let sample_packages = [
            ("forgekit-serde", "0.1.0"),
            ("forgekit-tokio", "0.1.0"),
            ("forgekit-http", "0.1.0"),
            ("forgekit-gui", "0.1.0"),
        ];

        for (name, version) in &sample_packages {
            let entry = IndexEntry {
                name: name.to_string(),
                versions: {
                    let mut versions = HashMap::new();
                    versions.insert(
                        version.to_string(),
                        VersionInfo {
                            version: version.to_string(),
                            git_ref: format!("v{}", version),
                            archive_url: format!(
                                "https://github.com/ledokoz-tech/{}/archive/v{}.tar.gz",
                                name, version
                            ),
                            published: chrono::Utc::now().to_rfc3339(),
                            checksum: "".to_string(),
                        },
                    );
                    versions
                },
                latest: version.to_string(),
            };
            index.insert(name.to_string(), entry);
        }

        let index_json = serde_json::to_string_pretty(&index)?;
        fs::write(&index_path, index_json)?;

        Ok(())
    }

    /// List all available packages
    pub async fn list_packages(&self) -> Result<Vec<String>, ForgeKitError> {
        let index_path = self.config.index_dir.join("packages.json");
        if index_path.exists() {
            let content = fs::read_to_string(&index_path)?;
            let index: HashMap<String, IndexEntry> = serde_json::from_str(&content)?;
            Ok(index.keys().cloned().collect())
        } else {
            Ok(vec![])
        }
    }
}

impl Default for RegistryClient {
    fn default() -> Self {
        Self::new(RegistryConfig::default()).unwrap()
    }
}
