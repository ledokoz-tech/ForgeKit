//! Project building functionality

use crate::error::ForgeKitError;
use std::path::Path;
use tokio::process::Command;

/// Build a project at the given path
pub async fn build(project_path: &Path) -> Result<(), ForgeKitError> {
    tracing::info!("Building project at {:?}", project_path);

    // Check if project exists
    if !project_path.exists() {
        return Err(ForgeKitError::ProjectNotFound(
            project_path.to_string_lossy().to_string(),
        ));
    }

    // Change to project directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(project_path)?;

    // Run cargo build with custom target
    let output = Command::new("cargo")
        .args(["build", "--target", "ledokoz", "--release"])
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ForgeKitError::BuildFailed(stderr.to_string()));
    }

    // Restore original directory
    std::env::set_current_dir(original_dir)?;

    tracing::info!("Build completed successfully");
    Ok(())
}
