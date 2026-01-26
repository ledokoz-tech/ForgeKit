//! Project packaging into .mox format

use crate::config::ProjectConfig;
use crate::error::ForgeKitError;
use std::path::{Path, PathBuf};
use tokio::fs;
use zip::{write::FileOptions, ZipWriter};

/// Package a built project into a .mox file
pub async fn package(project_path: &Path) -> Result<PathBuf, ForgeKitError> {
    tracing::info!("Packaging project at {:?}", project_path);
    
    // Check if project exists
    if !project_path.exists() {
        return Err(ForgeKitError::ProjectNotFound(
            project_path.to_string_lossy().to_string()
        ));
    }
    
    // Load project config
    let config_path = project_path.join("forgekit.toml");
    let config = ProjectConfig::load(&config_path)?;
    
    // Check if binary exists
    let binary_path = project_path.join("target").join("ledokoz").join("release").join(&config.name);
    if !binary_path.exists() {
        return Err(ForgeKitError::PackagingFailed(
            "Binary not found. Please build the project first.".to_string()
        ));
    }
    
    // Create output directory
    let output_dir = project_path.join(&config.build.output_dir);
    fs::create_dir_all(&output_dir).await?;
    
    // Create .mox file path
    let mox_filename = format!("{}.mox", config.name);
    let mox_path = output_dir.join(&mox_filename);
    
    // Create ZIP archive
    let file = std::fs::File::create(&mox_path)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    
    // Add binary to archive
    let binary_data = fs::read(&binary_path).await?;
    zip.start_file("app.bin", options)?;
    zip.write_all(&binary_data)?;
    
    // Add config to archive
    let config_data = toml::to_string_pretty(&config)?;
    zip.start_file("forgekit.toml", options)?;
    zip.write_all(config_data.as_bytes())?;
    
    // Add assets if they exist
    let assets_path = project_path.join("assets");
    if assets_path.exists() {
        add_assets_to_zip(&mut zip, &assets_path, options).await?;
    }
    
    // Finish ZIP
    zip.finish()?;
    
    tracing::info!("Package created at {:?}", mox_path);
    Ok(mox_path)
}

/// Recursively add assets to the ZIP archive
async fn add_assets_to_zip(
    zip: &mut ZipWriter<std::fs::File>,
    assets_path: &Path,
    options: FileOptions,
) -> Result<(), ForgeKitError> {
    let mut entries = fs::read_dir(assets_path).await?;
    
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        let name = path.strip_prefix(assets_path)
            .map_err(|_| ForgeKitError::PackagingFailed("Failed to strip prefix".to_string()))?;
        
        if path.is_file() {
            let data = fs::read(&path).await?;
            let zip_path = format!("assets/{}", name.to_string_lossy());
            zip.start_file(&zip_path, options)?;
            zip.write_all(&data)?;
        } else if path.is_dir() {
            add_assets_to_zip(zip, &path, options).await?;
        }
    }
    
    Ok(())
}

trait WriteAll {
    fn write_all(&mut self, data: &[u8]) -> Result<(), std::io::Error>;
}

impl WriteAll for ZipWriter<std::fs::File> {
    fn write_all(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        self.write(data)?;
        Ok(())
    }
}