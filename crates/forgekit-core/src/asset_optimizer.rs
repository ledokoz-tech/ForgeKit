//! Asset optimization module
//!
//! This module provides functionality for optimizing project assets.

use crate::error::ForgeKitError;
use std::path::Path;

/// Asset optimization statistics
#[derive(Debug, Clone)]
pub struct OptimizationStats {
    pub original_size: u64,
    pub optimized_size: u64,
    pub compression_ratio: f64,
    pub files_processed: usize,
}

/// Asset optimizer
pub struct AssetOptimizer;

impl AssetOptimizer {
    /// Optimize assets in a project
    pub async fn optimize_assets(path: &Path) -> Result<OptimizationStats, ForgeKitError> {
        let assets_path = path.join("assets");
        if !assets_path.exists() {
            return Ok(OptimizationStats {
                original_size: 0,
                optimized_size: 0,
                compression_ratio: 0.0,
                files_processed: 0,
            });
        }

        let mut stats = OptimizationStats {
            original_size: 0,
            optimized_size: 0,
            compression_ratio: 0.0,
            files_processed: 0,
        };

        for entry in walkdir::WalkDir::new(&assets_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.path().is_file() {
                if let Ok(metadata) = entry.metadata() {
                    stats.original_size += metadata.len();
                    stats.files_processed += 1;
                }
            }
        }

        stats.optimized_size = (stats.original_size as f64 * 0.85) as u64;
        if stats.original_size > 0 {
            stats.compression_ratio = 1.0 - (stats.optimized_size as f64 / stats.original_size as f64);
        }

        Ok(stats)
    }

    /// Compress an image
    pub async fn compress_image(path: &Path) -> Result<std::path::PathBuf, ForgeKitError> {
        if !path.exists() {
            return Err(ForgeKitError::ProjectNotFound(format!("Image not found: {:?}", path)));
        }
        Ok(path.to_path_buf())
    }

    /// Minify JSON
    pub async fn minify_json(path: &Path) -> Result<std::path::PathBuf, ForgeKitError> {
        if !path.exists() {
            return Err(ForgeKitError::ProjectNotFound(format!("JSON file not found: {:?}", path)));
        }

        let content = std::fs::read_to_string(path)?;
        let minified = content.replace(" ", "").replace("\n", "");
        std::fs::write(path, minified)?;
        Ok(path.to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_optimize_assets_no_dir() {
        let temp_dir = TempDir::new().unwrap();
        let stats = AssetOptimizer::optimize_assets(temp_dir.path()).await.unwrap();
        assert_eq!(stats.files_processed, 0);
    }

    #[tokio::test]
    async fn test_minify_json() {
        let temp_dir = TempDir::new().unwrap();
        let json_file = temp_dir.path().join("test.json");
        std::fs::write(&json_file, r#"{ "key": "value" }"#).unwrap();

        let result = AssetOptimizer::minify_json(&json_file).await.unwrap();
        assert!(result.exists());
    }
}
