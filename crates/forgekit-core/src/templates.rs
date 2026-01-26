//! Project template system for ForgeKit

use crate::error::ForgeKitError;
use std::path::Path;
use tokio::fs;

#[derive(Debug, Clone)]
pub enum TemplateType {
    /// Basic application template
    Basic,
    /// GUI application template
    Gui,
    /// CLI tool template
    Cli,
    /// Service/daemon template
    Service,
    /// Plugin template
    Plugin,
}

impl TemplateType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TemplateType::Basic => "basic",
            TemplateType::Gui => "gui",
            TemplateType::Cli => "cli",
            TemplateType::Service => "service",
            TemplateType::Plugin => "plugin",
        }
    }
}

/// Generate project from template
pub async fn generate_from_template(
    name: &str,
    template: TemplateType,
    path: &Path,
) -> Result<(), ForgeKitError> {
    match template {
        TemplateType::Basic => generate_basic_template(name, path).await,
        TemplateType::Gui => generate_gui_template(name, path).await,
        TemplateType::Cli => generate_cli_template(name, path).await,
        TemplateType::Service => generate_service_template(name, path).await,
        TemplateType::Plugin => generate_plugin_template(name, path).await,
    }
}

async fn generate_basic_template(name: &str, path: &Path) -> Result<(), ForgeKitError> {
    // Create basic project structure
    fs::create_dir_all(path).await?;
    fs::create_dir_all(path.join("src")).await?;
    fs::create_dir_all(path.join("assets")).await?;

    // Generate main.rs
    let main_content = format!(
        r#"//! Main application for {name}
//!
//! A basic .mox application built with ForgeKit

fn main() {{
    println!("Hello from {{}}!", "{name}");
    println!("Built with ForgeKit for Ledokoz OS");
    
    // Your application logic here
}}
"#
    );
    fs::write(path.join("src").join("main.rs"), main_content).await?;

    Ok(())
}

async fn generate_gui_template(name: &str, path: &Path) -> Result<(), ForgeKitError> {
    // Create GUI project structure
    fs::create_dir_all(path).await?;
    fs::create_dir_all(path.join("src")).await?;
    fs::create_dir_all(path.join("assets")).await?;
    fs::create_dir_all(path.join("ui")).await?;

    // Generate main.rs with GUI setup
    let main_content = format!(
        r#"//! GUI application for {name}
//!
//! A GUI .mox application built with ForgeKit

fn main() {{
    println!("Starting GUI application: {{}}", "{name}");
    
    // Initialize GUI framework
    // let window = create_window("{name}");
    // window.show();
    
    println!("GUI application running...");
}}
"#
    );
    fs::write(path.join("src").join("main.rs"), main_content).await?;

    // Create UI layout file
    let ui_content = r#"<!-- Default UI layout -->
<window title="Application" width="800" height="600">
    <layout type="vertical">
        <label text="Welcome to your ForgeKit GUI App!" />
        <button text="Click Me" onclick="handle_click" />
    </layout>
</window>
"#;
    fs::write(path.join("ui").join("main.xml"), ui_content).await?;

    Ok(())
}

async fn generate_cli_template(name: &str, path: &Path) -> Result<(), ForgeKitError> {
    fs::create_dir_all(path).await?;
    fs::create_dir_all(path.join("src")).await?;
    fs::create_dir_all(path.join("src").join("commands")).await?;

    let main_content = format!(
        r#"//! CLI tool: {name}
//!
//! A command-line tool built with ForgeKit

use clap::Parser;

#[derive(Parser)]
#[command(name = "{name}")]
#[command(about = "A powerful CLI tool built with ForgeKit")]
struct Cli {{
    #[command(subcommand)]
    command: Commands,
}}

#[derive(clap::Subcommand)]
enum Commands {{
    /// Process files
    Process {{
        /// Input file path
        input: String,
        /// Output file path
        output: Option<String>,
    }},
    /// Show version information
    Version,
}}

fn main() {{
    let cli = Cli::parse();
    
    match cli.command {{
        Commands::Process {{ input, output }} => {{
            println!("Processing file: {{}}", input);
            let output_path = output.unwrap_or_else(|| format!("{{}}_processed", input));
            println!("Output will be saved to: {{}}", output_path);
        }}
        Commands::Version => {{
            println!("{{}} v0.1.0", "{name}");
            println!("Built with ForgeKit for Ledokoz OS");
        }}
    }}
}}
"#
    );
    fs::write(path.join("src").join("main.rs"), main_content).await?;

    Ok(())
}

async fn generate_service_template(name: &str, path: &Path) -> Result<(), ForgeKitError> {
    fs::create_dir_all(path).await?;
    fs::create_dir_all(path.join("src")).await?;

    let main_content = format!(
        r#"//! Service/Daemon: {name}
//!
//! A background service built with ForgeKit

use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{
    println!("Starting service: {{}}", "{name}");
    
    // Service initialization
    initialize_service().await?;
    
    // Wait for shutdown signal
    signal::ctrl_c().await?;
    println!("Shutting down service...");
    
    Ok(())
}}

async fn initialize_service() -> Result<(), Box<dyn std::error::Error>> {{
    println!("Service initialized");
    // Add your service logic here
    
    Ok(())
}}
"#
    );
    fs::write(path.join("src").join("main.rs"), main_content).await?;

    Ok(())
}

async fn generate_plugin_template(name: &str, path: &Path) -> Result<(), ForgeKitError> {
    fs::create_dir_all(path).await?;
    fs::create_dir_all(path.join("src")).await?;

    let lib_content = format!(
        r#"//! Plugin library: {name}
//!
//! A ForgeKit plugin

use forgekit_core::{{Plugin, PluginContext}};

pub struct {name_cap}Plugin;

impl Plugin for {name_cap}Plugin {{
    fn name(&self) -> &'static str {{
        "{name}"
    }}
    
    fn version(&self) -> &'static str {{
        "0.1.0"
    }}
    
    fn initialize(&mut self, ctx: &PluginContext) -> Result<(), Box<dyn std::error::Error>> {{
        println!("Initializing {{}} plugin", self.name());
        Ok(())
    }}
    
    fn execute(&self, ctx: &PluginContext, data: &str) -> Result<String, Box<dyn std::error::Error>> {{
        Ok(format!("Processed by {{}}: {{}}", self.name(), data))
    }}
}}

// Export the plugin
forgekit_core::export_plugin!({name_cap}Plugin);
"#,
        name_cap = name
            .chars()
            .next()
            .unwrap()
            .to_uppercase()
            .collect::<String>()
            + &name[1..]
    );
    fs::write(path.join("src").join("lib.rs"), lib_content).await?;

    Ok(())
}
