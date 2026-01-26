//! ForgeKit CLI - Command line interface for building .mox applications

use anyhow::Result;
use clap::{Parser, Subcommand};
use forgekit_core::{ForgeKit, templates::TemplateType, dependencies::DependencyManager};
use std::path::PathBuf;

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
        /// Template type to use
        #[arg(short, long, default_value = "basic")]
        template: String,
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
    /// Add a dependency to the project
    Add {
        /// Package name to add
        package: String,
        /// Version to install
        #[arg(short, long, default_value = "*")]
        version: String,
        /// Path to the project (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Remove a dependency from the project
    Remove {
        /// Package name to remove
        package: String,
        /// Path to the project (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Update project dependencies
    Update {
        /// Path to the project (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Search for available packages
    Search {
        /// Search query
        query: String,
    },
    /// List available templates
    Templates,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, path, template } => {
            let project_path = path.unwrap_or_else(|| PathBuf::from(&name));
            let forgekit = ForgeKit::new();
                
            // Parse template type
            let template_type = match template.as_str() {
                "basic" => TemplateType::Basic,
                "gui" => TemplateType::Gui,
                "cli" => TemplateType::Cli,
                "service" => TemplateType::Service,
                "plugin" => TemplateType::Plugin,
                _ => {
                    eprintln!("Unknown template: {}. Using basic template.", template);
                    TemplateType::Basic
                }
            };
                
            forgekit.init_project_with_template(&name, &project_path, template_type).await?;
            println!("✅ Created new {} project '{}' at {:?}", template, name, project_path);
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
            let config =
                forgekit_core::config::ProjectConfig::load(project_path.join("forgekit.toml"))?;
            let binary_path = project_path
                .join("target")
                .join("ledokoz")
                .join("release")
                .join(&config.name);

            println!("🏃 Running application...");
            let status = tokio::process::Command::new(binary_path).status().await?;

            if status.success() {
                println!("✅ Application exited successfully");
            } else {
                println!(
                    "⚠️  Application exited with code: {}",
                    status.code().unwrap_or(-1)
                );
            }
        }
        Commands::Add { package, version, path } => {
            let project_path = match path {
                Some(p) => p,
                None => std::env::current_dir()?,
            };
            
            let dep_manager = DependencyManager::new();
            dep_manager.add_dependency(&project_path, &package, &version).await?;
            println!("✅ Added dependency: {} v{}", package, version);
        }
        Commands::Remove { package, path } => {
            let project_path = match path {
                Some(p) => p,
                None => std::env::current_dir()?,
            };
            
            let dep_manager = DependencyManager::new();
            dep_manager.remove_dependency(&project_path, &package).await?;
            println!("✅ Removed dependency: {}", package);
        }
        Commands::Update { path } => {
            let project_path = match path {
                Some(p) => p,
                None => std::env::current_dir()?,
            };
            
            let dep_manager = DependencyManager::new();
            dep_manager.update_dependencies(&project_path).await?;
            println!("✅ Dependencies updated");
        }
        Commands::Search { query } => {
            let dep_manager = DependencyManager::new();
            let results = dep_manager.search_packages(&query);
            
            if results.is_empty() {
                println!("No packages found matching '{}'", query);
            } else {
                println!("Found {} packages:", results.len());
                for pkg in results {
                    println!("  {} - {}", pkg.name, pkg.description);
                }
            }
        }
        Commands::Templates => {
            println!("Available templates:");
            println!("  basic    - Basic application template");
            println!("  gui      - Graphical user interface application");
            println!("  cli      - Command-line interface tool");
            println!("  service  - Background service/daemon");
            println!("  plugin   - ForgeKit plugin library");
        }
    }

    Ok(())
}
