//! ForgeKit CLI - Command line interface for building .mox applications

use anyhow::Result;
use clap::{Parser, Subcommand};
use forgekit_core::{package_manager::PackageManager, templates::TemplateType, ForgeKit};
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
enum CacheCommands {
    /// Clear the build cache
    Clear {
        /// Path to the project (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Show cache statistics
    Stats {
        /// Path to the project (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum EnvCommands {
    /// Set an environment variable
    Set {
        /// Variable name
        key: String,
        /// Variable value
        value: String,
        /// Environment file to update (defaults to .env)
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    /// List environment variables
    List {
        /// Environment to load (dev, staging, prod)
        #[arg(short, long)]
        environment: Option<String>,
        /// Path to the project (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
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
    /// Validate the current project
    Validate {
        /// Path to the project (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Manage environment variables
    Env {
        #[command(subcommand)]
        command: EnvCommands,
    },
    /// Run project tests
    Test {
        /// Path to the project (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
        /// Generate coverage report
        #[arg(long)]
        coverage: bool,
        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },
    /// Generate test scaffolding
    TestGenerate {
        /// Name of the test
        name: String,
        /// Path to the project (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Manage build cache
    Cache {
        #[command(subcommand)]
        command: CacheCommands,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            name,
            path,
            template,
        } => {
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

            forgekit
                .init_project_with_template(&name, &project_path, template_type)
                .await?;
            println!(
                "✅ Created new {} project '{}' at {:?}",
                template, name, project_path
            );
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
        Commands::Add {
            package,
            version,
            path,
        } => {
            let project_path = match path {
                Some(p) => p,
                None => std::env::current_dir()?,
            };

            let package_manager = PackageManager::new(project_path.clone())?;
            package_manager.add_dependency(&package, &version).await?;
            println!("✅ Added dependency: {} v{}", package, version);
        }
        Commands::Remove { package, path } => {
            let project_path = match path {
                Some(p) => p,
                None => std::env::current_dir()?,
            };

            let package_manager = PackageManager::new(project_path.clone())?;
            package_manager.remove_dependency(&package).await?;
            println!("✅ Removed dependency: {}", package);
        }
        Commands::Update { path } => {
            let project_path = match path {
                Some(p) => p,
                None => std::env::current_dir()?,
            };

            let package_manager = PackageManager::new(project_path.clone())?;
            package_manager.update_dependencies().await?;
            println!("✅ Dependencies updated");
        }
        Commands::Search { query } => {
            let current_dir = std::env::current_dir()?;
            let package_manager = PackageManager::new(current_dir)?;
            let results = package_manager.search_packages(&query).await?;

            if results.is_empty() {
                println!("No packages found matching '{}'", query);
            } else {
                println!("Found {} packages:", results.len());
                for result in results {
                    println!("  {}", result);
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
        Commands::Validate { path } => {
            let project_path = match path {
                Some(p) => p,
                None => std::env::current_dir()?,
            };

            let report =
                forgekit_core::validator::ProjectValidator::validate_project(&project_path).await?;

            if report.errors.is_empty() && report.warnings.is_empty() {
                println!("✅ Project validation passed");
            } else {
                if !report.errors.is_empty() {
                    println!("❌ Validation errors:");
                    for error in &report.errors {
                        println!("   - {}", error);
                    }
                }
                if !report.warnings.is_empty() {
                    println!("⚠️  Validation warnings:");
                    for warning in &report.warnings {
                        println!("   - {}", warning);
                    }
                }
            }

            if !report.is_valid {
                std::process::exit(1);
            }
        }
        Commands::Env { command } => match command {
            EnvCommands::Set { key, value, file } => {
                let env_file = file.unwrap_or_else(|| PathBuf::from(".env"));
                let mut manager =
                    forgekit_core::env_manager::EnvManager::load_from_file(&env_file)?;
                manager.set(key.clone(), value.clone());
                manager.save_to_file(&env_file)?;
                println!("✅ Set {}={}", key, value);
            }
            EnvCommands::List { environment, path } => {
                let project_path = match path {
                    Some(p) => p,
                    None => std::env::current_dir()?,
                };

                let manager = if let Some(env) = environment {
                    forgekit_core::env_manager::EnvManager::load_for_environment(
                        &env,
                        &project_path,
                    )?
                } else {
                    forgekit_core::env_manager::EnvManager::load_from_file(
                        &project_path.join(".env"),
                    )?
                };

                if manager.all().is_empty() {
                    println!("No environment variables set");
                } else {
                    println!("Environment variables:");
                    for (key, value) in manager.all() {
                        println!("  {}={}", key, value);
                    }
                }
            }
        },

        Commands::Test {
            path,
            coverage,
            format,
        } => {
            let project_path = match path {
                Some(p) => p,
                None => std::env::current_dir()?,
            };

            if coverage {
                let (test_report, coverage_report) =
                    forgekit_core::testing::TestRunner::run_tests_with_coverage(&project_path)
                        .await?;

                if format == "json" {
                    let json = serde_json::json!({
                        "tests": {
                            "total": test_report.total,
                            "passed": test_report.passed,
                            "failed": test_report.failed,
                        },
                        "coverage": {
                            "percentage": coverage_report.coverage_percentage,
                            "lines_covered": coverage_report.lines_covered,
                            "total_lines": coverage_report.total_lines,
                        }
                    });
                    println!("{}", serde_json::to_string_pretty(&json)?);
                } else {
                    println!("Test Results:");
                    println!("  Total: {}", test_report.total);
                    println!("  Passed: {}", test_report.passed);
                    println!("  Failed: {}", test_report.failed);
                    println!("\nCoverage:");
                    println!("  {:.2}%", coverage_report.coverage_percentage);
                    println!(
                        "  Lines: {}/{}",
                        coverage_report.lines_covered, coverage_report.total_lines
                    );
                }
            } else {
                let report = forgekit_core::testing::TestRunner::run_tests(&project_path).await?;

                if format == "json" {
                    let json = serde_json::json!({
                        "total": report.total,
                        "passed": report.passed,
                        "failed": report.failed,
                    });
                    println!("{}", serde_json::to_string_pretty(&json)?);
                } else {
                    println!("Test Results:");
                    println!("  Total: {}", report.total);
                    println!("  Passed: {}", report.passed);
                    println!("  Failed: {}", report.failed);

                    if report.failed > 0 {
                        println!("\n❌ Some tests failed");
                        std::process::exit(1);
                    } else {
                        println!("\n✅ All tests passed");
                    }
                }
            }
        }
        Commands::TestGenerate { name, path } => {
            let project_path = match path {
                Some(p) => p,
                None => std::env::current_dir()?,
            };

            let test_file =
                forgekit_core::testing::TestRunner::generate_test_scaffold(&name, &project_path)
                    .await?;
            println!("✅ Generated test scaffold at {:?}", test_file);
        }
        Commands::Cache { command } => match command {
            CacheCommands::Clear { path } => {
                let project_path = match path {
                    Some(p) => p,
                    None => std::env::current_dir()?,
                };

                let cache_dir = project_path.join(".forgekit").join("cache");
                let mut cache = forgekit_core::cache::BuildCache::new(cache_dir)?;
                cache.clear().await?;
                println!("✅ Cache cleared");
            }
            CacheCommands::Stats { path } => {
                let project_path = match path {
                    Some(p) => p,
                    None => std::env::current_dir()?,
                };

                let cache_dir = project_path.join(".forgekit").join("cache");
                let mut cache = forgekit_core::cache::BuildCache::new(cache_dir)?;
                cache.load_from_disk()?;

                let stats = cache.stats();
                println!("Cache Statistics:");
                println!("  Items: {}", stats.item_count);
                println!("  Size: {} bytes", stats.total_size);
                println!("  Hits: {}", stats.hits);
                println!("  Misses: {}", stats.misses);
                println!("  Hit Rate: {:.2}%", stats.hit_rate * 100.0);
            }
        },
    }

    Ok(())
}
