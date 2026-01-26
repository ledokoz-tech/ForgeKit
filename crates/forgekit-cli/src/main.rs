//! ForgeKit CLI - Command line interface for building .mox applications

use anyhow::Result;
use clap::{Parser, Subcommand};
use forgekit_core::ForgeKit;
use std::path::PathBuf;
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "forgekit")]
#[command(about = "A modern Rust framework for building .mox applications for Ledokoz OS")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new .mox application
    New {
        /// Name of the new project
        name: String,
        /// Path where to create the project (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Build the current project
    Build {
        /// Path to the project (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Package the project into a .mox file
    Package {
        /// Path to the project (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Build and package the project
    BuildPackage {
        /// Path to the project (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Run the project locally (for testing)
    Run {
        /// Path to the project (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::New { name, path } => {
            let project_path = path.unwrap_or_else(|| PathBuf::from(&name));
            let forgekit = ForgeKit::new();
            
            forgekit.init_project(&name, &project_path).await?;
            println!("✅ Created new project '{}' at {:?}", name, project_path);
            println!("📁 Navigate to the project directory:");
            println!("   cd {}", project_path.display());
            println!("🔨 Build your project:");
            println!("   forgekit build");
        }
        Commands::Build { path } => {
            let project_path = match path {
                Some(p) => p,
                None => std::env::current_dir()?,
            };
            let forgekit = ForgeKit::new();
            
            forgekit.build_project(&project_path).await?;
            println!("✅ Build completed successfully");
        }
        Commands::Package { path } => {
            let project_path = match path {
                Some(p) => p,
                None => std::env::current_dir()?,
            };
            let forgekit = ForgeKit::new();
            
            let package_path = forgekit.package_project(&project_path).await?;
            println!("✅ Package created at {:?}", package_path);
        }
        Commands::BuildPackage { path } => {
            let project_path = match path {
                Some(p) => p,
                None => std::env::current_dir()?,
            };
            let forgekit = ForgeKit::new();
            
            // Build first
            forgekit.build_project(&project_path).await?;
            println!("✅ Build completed");
            
            // Then package
            let package_path = forgekit.package_project(&project_path).await?;
            println!("✅ Package created at {:?}", package_path);
        }
        Commands::Run { path } => {
            let project_path = match path {
                Some(p) => p,
                None => std::env::current_dir()?,
            };
            let forgekit = ForgeKit::new();
            
            // Build first
            forgekit.build_project(&project_path).await?;
            println!("✅ Build completed");
            
            // Run the binary
            let config = forgekit_core::config::ProjectConfig::load(project_path.join("forgekit.toml"))?;
            let binary_path = project_path.join("target").join("ledokoz").join("release").join(&config.name);
            
            println!("🏃 Running application...");
            let status = tokio::process::Command::new(binary_path)
                .status()
                .await?;
                
            if status.success() {
                println!("✅ Application exited successfully");
            } else {
                println!("⚠️  Application exited with code: {}", status.code().unwrap_or(-1));
            }
        }
    }
    
    Ok(())
}