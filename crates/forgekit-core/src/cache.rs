//! Build caching module
//!
//! This module provides functionality for caching build artifacts
//! to speed up subsequent builds.

use crate::error::ForgeKitError;
use std::collections::HashMap;
use std::path::{Path, PathBuf};


/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Total cache size in bytes
    pub total_size: u64,
    /// Number of cached items
    pub item_count: usize,
    /// Cache hit rate (0.0 to 1.0)
    pub hit_rate: f64,
    /// Number of cache hits
    pub hits: usize,
    /// Number of cache misses
    pub misses: usize,
}

impl CacheStats {
    /// Create new cache statistics
    pub fn new() -> Self {
        Self {
            total_size: 0,
            item_count: 0,
            hit_rate: 0.0,
            hits: 0,
            misses: 0,
        }
    }
}

impl Default for CacheStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Build cache for storing and retrieving build artifacts
#[derive(Debug)]
pub struct BuildCache {
    cache_dir: PathBuf,
    cache_data: HashMap<String, Vec<u8>>,
    stats: CacheStats,
}

impl BuildCache {
    /// Create a new build cache
    ///
    /// # Arguments
    ///
    /// * `cache_dir` - Directory to store cache files
    pub fn new(cache_dir: PathBuf) -> Result<Self, ForgeKitError> {
        if !cache_dir.exists() {
            std::fs::create_dir_all(&cache_dir)?;
        }

        Ok(Self {
            cache_dir,
            cache_data: HashMap::new(),
            stats: CacheStats::new(),
        })
    }

    /// Get a cached value
    ///
    /// # Arguments
    ///
    /// * `key` - Cache key
    pub async fn get(&mut self, key: &str) -> Option<Vec<u8>> {
        if let Some(data) = self.cache_data.get(key) {
            self.stats.hits += 1;
            return Some(data.clone());
        }

        // Try to load from disk
        let cache_file = self.cache_dir.join(format!("{}.cache", key));
        if cache_file.exists() {
            if let Ok(data) = std::fs::read(&cache_file) {
                self.cache_data.insert(key.to_string(), data.clone());
                self.stats.hits += 1;
                return Some(data);
            }
        }

        self.stats.misses += 1;
        None
    }

    /// Set a cached value
    ///
    /// # Arguments
    ///
    /// * `key` - Cache key
    /// * `value` - Value to cache
    pub async fn set(&mut self, key: &str, value: Vec<u8>) -> Result<(), ForgeKitError> {
        let cache_file = self.cache_dir.join(format!("{}.cache", key));
        std::fs::write(&cache_file, &value)?;
        self.cache_data.insert(key.to_string(), value);
        Ok(())
    }

    /// Invalidate cache entries matching a pattern
    ///
    /// # Arguments
    ///
    /// * `pattern` - Glob pattern to match keys
    pub async fn invalidate(&mut self, pattern: &str) -> Result<(), ForgeKitError> {
        let regex = glob_to_regex(pattern);

        // Remove from memory
        self.cache_data.retain(|key, _| !regex.is_match(key));

        // Remove from disk
        if let Ok(entries) = std::fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        if let Some(filename) = entry.file_name().to_str() {
                            if filename.ends_with(".cache") {
                                let key = filename.trim_end_matches(".cache");
                                if regex.is_match(key) {
                                    let _ = std::fs::remove_file(entry.path());
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Clear all cache
    pub async fn clear(&mut self) -> Result<(), ForgeKitError> {
        self.cache_data.clear();

        if self.cache_dir.exists() {
            std::fs::remove_dir_all(&self.cache_dir)?;
            std::fs::create_dir_all(&self.cache_dir)?;
        }

        Ok(())
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let mut stats = self.stats.clone();
        stats.item_count = self.cache_data.len();

        // Calculate total size
        stats.total_size = self.cache_data.values().map(|v| v.len() as u64).sum();

        // Calculate hit rate
        let total = stats.hits + stats.misses;
        if total > 0 {
            stats.hit_rate = stats.hits as f64 / total as f64;
        }

        stats
    }

    /// Load cache from disk
    pub fn load_from_disk(&mut self) -> Result<(), ForgeKitError> {
        if !self.cache_dir.exists() {
            return Ok(());
        }

        if let Ok(entries) = std::fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        if let Some(filename) = entry.file_name().to_str() {
                            if filename.ends_with(".cache") {
                                let key = filename.trim_end_matches(".cache").to_string();
                                if let Ok(data) = std::fs::read(entry.path()) {
                                    self.cache_data.insert(key, data);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Get cache directory path
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }
}

/// Convert glob pattern to regex
fn glob_to_regex(pattern: &str) -> regex::Regex {
    let regex_pattern = pattern
        .replace(".", r"\.")
        .replace("*", ".*")
        .replace("?", ".");

    regex::Regex::new(&format!("^{}$", regex_pattern))
        .unwrap_or_else(|_| regex::Regex::new(".*").unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_cache_creation() {
        let temp_dir = TempDir::new().unwrap();
        let cache = BuildCache::new(temp_dir.path().to_path_buf()).unwrap();
        assert!(cache.cache_dir.exists());
    }

    #[tokio::test]
    async fn test_set_and_get() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = BuildCache::new(temp_dir.path().to_path_buf()).unwrap();

        let data = vec![1, 2, 3, 4, 5];
        cache.set("test_key", data.clone()).await.unwrap();

        let retrieved = cache.get("test_key").await;
        assert_eq!(retrieved, Some(data));
    }

    #[tokio::test]
    async fn test_cache_miss() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = BuildCache::new(temp_dir.path().to_path_buf()).unwrap();

        let retrieved = cache.get("nonexistent").await;
        assert_eq!(retrieved, None);
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = BuildCache::new(temp_dir.path().to_path_buf()).unwrap();

        cache.set("key1", vec![1, 2, 3]).await.unwrap();
        cache.set("key2", vec![4, 5, 6]).await.unwrap();

        cache.clear().await.unwrap();

        assert_eq!(cache.get("key1").await, None);
        assert_eq!(cache.get("key2").await, None);
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = BuildCache::new(temp_dir.path().to_path_buf()).unwrap();

        cache.set("key1", vec![1, 2, 3]).await.unwrap();
        cache.set("key2", vec![4, 5, 6]).await.unwrap();

        let _ = cache.get("key1").await;
        let _ = cache.get("key1").await;
        let _ = cache.get("nonexistent").await;

        let stats = cache.stats();
        assert_eq!(stats.item_count, 2);
        assert_eq!(stats.hits, 2);
        assert_eq!(stats.misses, 1);
    }

    #[tokio::test]
    async fn test_invalidate_pattern() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = BuildCache::new(temp_dir.path().to_path_buf()).unwrap();

        cache.set("build_1", vec![1, 2, 3]).await.unwrap();
        cache.set("build_2", vec![4, 5, 6]).await.unwrap();
        cache.set("test_1", vec![7, 8, 9]).await.unwrap();

        cache.invalidate("build_*").await.unwrap();

        assert_eq!(cache.get("build_1").await, None);
        assert_eq!(cache.get("build_2").await, None);
        assert_eq!(cache.get("test_1").await, Some(vec![7, 8, 9]));
    }

    #[test]
    fn test_load_from_disk() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = BuildCache::new(temp_dir.path().to_path_buf()).unwrap();

        // Write cache file directly
        let cache_file = temp_dir.path().join("test_key.cache");
        std::fs::write(&cache_file, vec![1, 2, 3]).unwrap();

        cache.load_from_disk().unwrap();
        assert!(cache.cache_data.contains_key("test_key"));
    }
}
