//! Error types for ForgeKit

use thiserror::Error;
use zip::result::ZipError;

#[derive(Error, Debug)]
pub enum ForgeKitError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),
    
    #[error("Project already exists at path: {0}")]
    ProjectExists(String),
    
    #[error("Project not found at path: {0}")]
    ProjectNotFound(String),
    
    #[error("Invalid project configuration: {0}")]
    InvalidConfig(String),
    
    #[error("Build failed: {0}")]
    BuildFailed(String),
    
    #[error("Packaging failed: {0}")]
    PackagingFailed(String),
    
    #[error("Template error: {0}")]
    TemplateError(String),
    
    #[error("ZIP error: {0}")]
    Zip(#[from] ZipError),
    
    #[error("TOML serialization error: {0}")]
    TomlSerialization(#[from] toml::ser::Error),
}
