//! Project analytics module
//!
//! This module provides project metrics and analytics.

use crate::error::ForgeKitError;
use std::path::Path;
use std::time::Duration;

/// Code metrics
#[derive(Debug, Clone)]
pub struct CodeMetrics {
    pub lines_of_code: usize,
    pub cyclomatic_complexity: f64,
    pub test_coverage: f64,
}

/// Project metrics
#[derive(Debug, Clone)]
pub struct ProjectMetrics {
    pub build_times: Vec<Duration>,
    pub dependency_count: usize,
    pub code_metrics: CodeMetrics,
}

/// Analytics report
#[derive(Debug, Clone)]
pub struct AnalyticsReport {
    pub metrics: ProjectMetrics,
    pub generated_at: String,
}

/// Analytics collector
pub struct AnalyticsCollector;

impl AnalyticsCollector {
    /// Collect project metrics
    pub async fn collect_metrics(path: &Path) -> Result<ProjectMetrics, ForgeKitError> {
        let src_path = path.join("src");
        let mut lines_of_code = 0;

        if src_path.exists() {
            for entry in walkdir::WalkDir::new(&src_path)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.path().extension().map(|e| e == "rs").unwrap_or(false) {
                    if let Ok(content) = std::fs::read_to_string(entry.path()) {
                        lines_of_code += content.lines().count();
                    }
                }
            }
        }

        Ok(ProjectMetrics {
            build_times: Vec::new(),
            dependency_count: 0,
            code_metrics: CodeMetrics {
                lines_of_code,
                cyclomatic_complexity: 0.0,
                test_coverage: 0.0,
            },
        })
    }

    /// Generate analytics report
    pub async fn generate_report(path: &Path) -> Result<AnalyticsReport, ForgeKitError> {
        let metrics = Self::collect_metrics(path).await?;

        Ok(AnalyticsReport {
            metrics,
            generated_at: chrono::Local::now().to_rfc3339(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_collect_metrics() {
        let temp_dir = TempDir::new().unwrap();
        let result = AnalyticsCollector::collect_metrics(temp_dir.path()).await;
        assert!(result.is_ok());
    }
}
