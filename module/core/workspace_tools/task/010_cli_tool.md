# Task 010: CLI Tool

**Priority**: 🛠️ High Visibility Impact  
**Phase**: 4 (Tooling Ecosystem)  
**Estimated Effort**: 5-6 days  
**Dependencies**: Tasks 001-003 (Core features), Task 002 (Templates)  

## **Objective**
Create a comprehensive CLI tool (`cargo-workspace-tools`) that makes workspace_tools visible to all Rust developers and provides immediate utility for workspace management, scaffolding, and validation.

## **Technical Requirements**

### **Core Features**
1. **Workspace Management**
   - Initialize new workspaces with standard structure
   - Validate workspace configuration and structure
   - Show workspace information and diagnostics

2. **Project Scaffolding**
   - Create projects from built-in templates
   - Custom template support
   - Interactive project creation wizard

3. **Configuration Management**
   - Validate configuration files
   - Show resolved configuration values
   - Environment-aware configuration display

4. **Development Tools**
   - Watch mode for configuration changes
   - Workspace health checks
   - Integration with other cargo commands

### **CLI Structure**
```bash
# Installation
cargo install workspace-tools-cli

# Main commands
cargo workspace-tools init [--template=TYPE] [PATH]
cargo workspace-tools validate [--config] [--structure]
cargo workspace-tools info [--json] [--verbose]
cargo workspace-tools scaffold --template=TYPE [--interactive]
cargo workspace-tools config [show|validate|watch] [NAME]
cargo workspace-tools templates [list|validate] [TEMPLATE]
cargo workspace-tools doctor [--fix]
```

### **Implementation Steps**

#### **Step 1: CLI Foundation and Structure** (Day 1)
```rust
// Create new crate: workspace-tools-cli/Cargo.toml
[package]
name = "workspace-tools-cli"
version = "0.1.0"
edition = "2021"
authors = ["workspace_tools contributors"]
description = "Command-line interface for workspace_tools"
license = "MIT"

[[bin]]
name = "cargo-workspace-tools"
path = "src/main.rs"

[dependencies]
workspace_tools = { path = "../workspace_tools", features = ["full"] }
clap = { version = "4.0", features = ["derive", "color", "suggestions"] }
clap_complete = "4.0"
anyhow = "1.0"
console = "0.15"
dialoguer = "0.10"
indicatif = "0.17"
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"], optional = true }

[features]
default = ["async"]
async = ["tokio", "workspace_tools/async"]

// src/main.rs
use clap::{Parser, Subcommand};
use anyhow::Result;

mod commands;
mod utils;
mod templates;

#[derive(Parser)]
#[command(
    name = "cargo-workspace-tools",
    version = env!("CARGO_PKG_VERSION"),
    author = "workspace_tools contributors",
    about = "A CLI tool for workspace management with workspace_tools",
    long_about = "Provides workspace creation, validation, scaffolding, and management capabilities"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
    
    /// Output format (text, json)
    #[arg(long, global = true, default_value = "text")]
    format: OutputFormat,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new workspace
    Init {
        /// Path to create workspace in
        path: Option<std::path::PathBuf>,
        
        /// Template to use for initialization
        #[arg(short, long)]
        template: Option<String>,
        
        /// Skip interactive prompts
        #[arg(short, long)]
        quiet: bool,
    },
    
    /// Validate workspace structure and configuration
    Validate {
        /// Validate configuration files
        #[arg(short, long)]
        config: bool,
        
        /// Validate directory structure
        #[arg(short, long)]
        structure: bool,
        
        /// Fix issues automatically where possible
        #[arg(short, long)]
        fix: bool,
    },
    
    /// Show workspace information
    Info {
        /// Output detailed information
        #[arg(short, long)]
        verbose: bool,
        
        /// Show configuration values
        #[arg(short, long)]
        config: bool,
        
        /// Show workspace statistics
        #[arg(short, long)]
        stats: bool,
    },
    
    /// Create new components from templates
    Scaffold {
        /// Template type to use
        #[arg(short, long)]
        template: String,
        
        /// Interactive mode
        #[arg(short, long)]
        interactive: bool,
        
        /// Component name
        name: Option<String>,
    },
    
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    
    /// Template management
    Templates {
        #[command(subcommand)]
        action: TemplateAction,
    },
    
    /// Run workspace health diagnostics
    Doctor {
        /// Attempt to fix issues
        #[arg(short, long)]
        fix: bool,
        
        /// Only check specific areas
        #[arg(short, long)]
        check: Vec<String>,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Show configuration values
    Show {
        /// Configuration name to show
        name: Option<String>,
        
        /// Show all configurations
        #[arg(short, long)]
        all: bool,
    },
    
    /// Validate configuration files
    Validate {
        /// Configuration name to validate
        name: Option<String>,
    },
    
    /// Watch configuration files for changes
    #[cfg(feature = "async")]
    Watch {
        /// Configuration name to watch
        name: Option<String>,
    },
}

#[derive(Subcommand)]
enum TemplateAction {
    /// List available templates
    List,
    
    /// Validate a template
    Validate {
        /// Template name or path
        template: String,
    },
    
    /// Create a new custom template
    Create {
        /// Template name
        name: String,
        
        /// Base on existing template
        #[arg(short, long)]
        base: Option<String>,
    },
}

#[derive(Clone, Debug, clap::ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Set up logging based on verbosity
    if cli.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    }
    
    match cli.command {
        Commands::Init { path, template, quiet } => {
            commands::init::run(path, template, quiet, cli.format)
        }
        Commands::Validate { config, structure, fix } => {
            commands::validate::run(config, structure, fix, cli.format)
        }
        Commands::Info { verbose, config, stats } => {
            commands::info::run(verbose, config, stats, cli.format)
        }
        Commands::Scaffold { template, interactive, name } => {
            commands::scaffold::run(template, interactive, name, cli.format)
        }
        Commands::Config { action } => {
            commands::config::run(action, cli.format)
        }
        Commands::Templates { action } => {
            commands::templates::run(action, cli.format)
        }
        Commands::Doctor { fix, check } => {
            commands::doctor::run(fix, check, cli.format)
        }
    }
}
```

#### **Step 2: Workspace Initialization Command** (Day 2)
```rust
// src/commands/init.rs
use workspace_tools::{workspace, Workspace, TemplateType};
use anyhow::{Result, Context};
use console::style;
use dialoguer::{Confirm, Input, Select};
use std::path::PathBuf;

pub fn run(
    path: Option<PathBuf>, 
    template: Option<String>,
    quiet: bool,
    format: crate::OutputFormat,
) -> Result<()> {
    let target_path = path.unwrap_or_else(|| std::env::current_dir().unwrap());
    
    println!("{} Initializing workspace at {}", 
        style("🚀").cyan(), 
        style(target_path.display()).yellow()
    );
    
    // Check if directory is empty
    if target_path.exists() && target_path.read_dir()?.next().is_some() {
        if !quiet && !Confirm::new()
            .with_prompt("Directory is not empty. Continue?")
            .interact()? 
        {
            println!("Initialization cancelled.");
            return Ok(());
        }
    }
    
    // Set up workspace environment
    std::env::set_var("WORKSPACE_PATH", &target_path);
    let ws = Workspace::resolve().context("Failed to resolve workspace")?;
    
    // Determine template to use
    let template_type = if let Some(template_name) = template {
        parse_template_type(&template_name)?
    } else if quiet {
        TemplateType::Library // Default for quiet mode
    } else {
        prompt_for_template()?
    };
    
    // Create workspace structure
    create_workspace_structure(&ws, template_type, quiet)?;
    
    // Create cargo workspace config if not exists
    create_cargo_config(&ws)?;
    
    // Show success message
    match format {
        crate::OutputFormat::Text => {
            println!("\n{} Workspace initialized successfully!", style("✅").green());
            println!("  Template: {}", style(template_type.name()).yellow());
            println!("  Path: {}", style(target_path.display()).yellow());
            println!("\n{} Next steps:", style("💡").blue());
            println!("  cd {}", target_path.display());
            println!("  cargo workspace-tools info");
            println!("  cargo build");
        }
        crate::OutputFormat::Json => {
            let result = serde_json::json!({
                "status": "success",
                "path": target_path,
                "template": template_type.name(),
                "directories_created": template_type.directories().len(),
                "files_created": template_type.template_files().len(),
            });
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
    }
    
    Ok(())
}

fn prompt_for_template() -> Result<TemplateType> {
    let templates = vec![
        ("CLI Application", TemplateType::Cli),
        ("Web Service", TemplateType::WebService),
        ("Library", TemplateType::Library),
        ("Desktop Application", TemplateType::Desktop),
    ];
    
    let selection = Select::new()
        .with_prompt("Choose a project template")
        .items(&templates.iter().map(|(name, _)| *name).collect::<Vec<_>>())
        .default(0)
        .interact()?;
        
    Ok(templates[selection].1)
}

fn parse_template_type(name: &str) -> Result<TemplateType> {
    match name.to_lowercase().as_str() {
        "cli" | "command-line" => Ok(TemplateType::Cli),
        "web" | "web-service" | "server" => Ok(TemplateType::WebService),
        "lib" | "library" => Ok(TemplateType::Library),
        "desktop" | "gui" => Ok(TemplateType::Desktop),
        _ => anyhow::bail!("Unknown template type: {}. Available: cli, web, lib, desktop", name),
    }
}

fn create_workspace_structure(
    ws: &Workspace, 
    template_type: TemplateType,
    quiet: bool
) -> Result<()> {
    if !quiet {
        println!("{} Creating workspace structure...", style("📁").cyan());
    }
    
    // Use workspace_tools template system
    ws.scaffold_from_template(template_type)
        .context("Failed to scaffold workspace from template")?;
    
    if !quiet {
        println!("  {} Standard directories created", style("✓").green());
        println!("  {} Template files created", style("✓").green());
    }
    
    Ok(())
}

fn create_cargo_config(ws: &Workspace) -> Result<()> {
    let cargo_dir = ws.join(".cargo");
    let config_file = cargo_dir.join("config.toml");
    
    if !config_file.exists() {
        std::fs::create_dir_all(&cargo_dir)?;
        let cargo_config = r#"# Workspace configuration
[env]
WORKSPACE_PATH = { value = ".", relative = true }

[build]
# Uncomment to use a custom target directory
# target-dir = "target"
"#;
        std::fs::write(&config_file, cargo_config)?;
        println!("  {} Cargo workspace config created", style("✓").green());
    }
    
    Ok(())
}

impl TemplateType {
    fn name(&self) -> &'static str {
        match self {
            TemplateType::Cli => "CLI Application",
            TemplateType::WebService => "Web Service", 
            TemplateType::Library => "Library",
            TemplateType::Desktop => "Desktop Application",
        }
    }
}
```

#### **Step 3: Validation and Info Commands** (Day 3)
```rust
// src/commands/validate.rs
use workspace_tools::{workspace, WorkspaceError};
use anyhow::Result;
use console::style;
use std::collections::HashMap;

pub fn run(
    config: bool,
    structure: bool, 
    fix: bool,
    format: crate::OutputFormat,
) -> Result<()> {
    let ws = workspace()?;
    
    let mut results = ValidationResults::new();
    
    // If no specific validation requested, do all
    let check_all = !config && !structure;
    
    if check_all || structure {
        validate_structure(&ws, &mut results, fix)?;
    }
    
    if check_all || config {
        validate_configurations(&ws, &mut results, fix)?;
    }
    
    // Show results
    match format {
        crate::OutputFormat::Text => {
            display_validation_results(&results);
        }
        crate::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&results)?);
        }
    }
    
    if results.has_errors() {
        std::process::exit(1);
    }
    
    Ok(())
}

#[derive(Debug, serde::Serialize)]
struct ValidationResults {
    structure: StructureValidation,
    configurations: Vec<ConfigValidation>,
    summary: ValidationSummary,
}

#[derive(Debug, serde::Serialize)]
struct StructureValidation {
    required_directories: Vec<DirectoryCheck>,
    optional_directories: Vec<DirectoryCheck>,
    issues: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
struct DirectoryCheck {
    path: String,
    exists: bool,
    required: bool,
    permissions_ok: bool,
}

#[derive(Debug, serde::Serialize)]
struct ConfigValidation {
    name: String,
    path: String,
    valid: bool,
    format: String,
    issues: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
struct ValidationSummary {
    total_checks: usize,
    passed: usize,
    warnings: usize,
    errors: usize,
}

impl ValidationResults {
    fn new() -> Self {
        Self {
            structure: StructureValidation {
                required_directories: Vec::new(),
                optional_directories: Vec::new(),
                issues: Vec::new(),
            },
            configurations: Vec::new(),
            summary: ValidationSummary {
                total_checks: 0,
                passed: 0,
                warnings: 0,
                errors: 0,
            },
        }
    }
    
    fn has_errors(&self) -> bool {
        self.summary.errors > 0
    }
    
    fn add_structure_check(&mut self, check: DirectoryCheck) {
        if check.required {
            self.structure.required_directories.push(check);
        } else {
            self.structure.optional_directories.push(check);
        }
        self.summary.total_checks += 1;
        if check.exists && check.permissions_ok {
            self.summary.passed += 1;
        } else if check.required {
            self.summary.errors += 1;
        } else {
            self.summary.warnings += 1;
        }
    }
}

fn validate_structure(
    ws: &workspace_tools::Workspace, 
    results: &mut ValidationResults,
    fix: bool
) -> Result<()> {
    println!("{} Validating workspace structure...", style("🔍").cyan());
    
    let required_dirs = vec![
        ("config", ws.config_dir()),
        ("data", ws.data_dir()),
        ("logs", ws.logs_dir()),
    ];
    
    let optional_dirs = vec![
        ("docs", ws.docs_dir()),
        ("tests", ws.tests_dir()),
        (".workspace", ws.workspace_dir()),
    ];
    
    // Check required directories
    for (name, path) in required_dirs {
        let exists = path.exists();
        let permissions_ok = check_directory_permissions(&path);
        
        if !exists && fix {
            std::fs::create_dir_all(&path)?;
            println!("  {} Created missing directory: {}", style("🔧").yellow(), name);
        }
        
        results.add_structure_check(DirectoryCheck {
            path: path.display().to_string(),
            exists: path.exists(), // Re-check after potential fix
            required: true,
            permissions_ok,
        });
    }
    
    // Check optional directories
    for (name, path) in optional_dirs {
        let exists = path.exists();
        let permissions_ok = if exists { check_directory_permissions(&path) } else { true };
        
        results.add_structure_check(DirectoryCheck {
            path: path.display().to_string(),
            exists,
            required: false,
            permissions_ok,
        });
    }
    
    Ok(())
}

fn check_directory_permissions(path: &std::path::Path) -> bool {
    if !path.exists() {
        return false;
    }
    
    // Check if we can read and write to the directory
    path.metadata()
        .map(|metadata| !metadata.permissions().readonly())
        .unwrap_or(false)
}

fn validate_configurations(
    ws: &workspace_tools::Workspace,
    results: &mut ValidationResults, 
    _fix: bool
) -> Result<()> {
    println!("{} Validating configurations...", style("⚙️").cyan());
    
    let config_dir = ws.config_dir();
    if !config_dir.exists() {
        results.configurations.push(ConfigValidation {
            name: "config directory".to_string(),
            path: config_dir.display().to_string(),
            valid: false,
            format: "directory".to_string(),
            issues: vec!["Config directory does not exist".to_string()],
        });
        results.summary.errors += 1;
        return Ok(());
    }
    
    // Find all config files
    let config_files = find_config_files(&config_dir)?;
    
    for config_file in config_files {
        let validation = validate_single_config(&config_file)?;
        
        if validation.valid {
            results.summary.passed += 1;
        } else {
            results.summary.errors += 1;
        }
        results.summary.total_checks += 1;
        results.configurations.push(validation);
    }
    
    Ok(())
}

fn find_config_files(config_dir: &std::path::Path) -> Result<Vec<std::path::PathBuf>> {
    let mut config_files = Vec::new();
    
    for entry in std::fs::read_dir(config_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if matches!(ext.to_str(), Some("toml" | "yaml" | "yml" | "json")) {
                    config_files.push(path);
                }
            }
        }
    }
    
    Ok(config_files)
}

fn validate_single_config(path: &std::path::Path) -> Result<ConfigValidation> {
    let mut issues = Vec::new();
    let mut valid = true;
    
    // Determine format
    let format = path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    // Try to parse the file
    match std::fs::read_to_string(path) {
        Ok(content) => {
            match format.as_str() {
                "toml" => {
                    if let Err(e) = toml::from_str::<toml::Value>(&content) {
                        issues.push(format!("TOML parsing error: {}", e));
                        valid = false;
                    }
                }
                "json" => {
                    if let Err(e) = serde_json::from_str::<serde_json::Value>(&content) {
                        issues.push(format!("JSON parsing error: {}", e));
                        valid = false;
                    }
                }
                "yaml" | "yml" => {
                    if let Err(e) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
                        issues.push(format!("YAML parsing error: {}", e));
                        valid = false;
                    }
                }
                _ => {
                    issues.push("Unknown configuration format".to_string());
                    valid = false;
                }
            }
        }
        Err(e) => {
            issues.push(format!("Failed to read file: {}", e));
            valid = false;
        }
    }
    
    Ok(ConfigValidation {
        name: path.file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string(),
        path: path.display().to_string(),
        valid,
        format,
        issues,
    })
}

fn display_validation_results(results: &ValidationResults) {
    println!("\n{} Validation Results", style("📊").cyan());
    println!("{}", "=".repeat(50));
    
    // Structure validation
    println!("\n{} Directory Structure:", style("📁").blue());
    for dir in &results.structure.required_directories {
        let status = if dir.exists && dir.permissions_ok {
            style("✓").green()
        } else {
            style("✗").red()
        };
        println!("  {} {} (required)", status, dir.path);
    }
    
    for dir in &results.structure.optional_directories {
        let status = if dir.exists {
            style("✓").green()
        } else {
            style("-").yellow()
        };
        println!("  {} {} (optional)", status, dir.path);
    }
    
    // Configuration validation
    println!("\n{} Configuration Files:", style("⚙️").blue());
    for config in &results.configurations {
        let status = if config.valid {
            style("✓").green()
        } else {
            style("✗").red()
        };
        println!("  {} {} ({})", status, config.name, config.format);
        
        for issue in &config.issues {
            println!("    {} {}", style("!").red(), issue);
        }
    }
    
    // Summary
    println!("\n{} Summary:", style("📋").blue());
    println!("  Total checks: {}", results.summary.total_checks);
    println!("  {} Passed: {}", style("✓").green(), results.summary.passed);
    if results.summary.warnings > 0 {
        println!("  {} Warnings: {}", style("⚠").yellow(), results.summary.warnings);
    }
    if results.summary.errors > 0 {
        println!("  {} Errors: {}", style("✗").red(), results.summary.errors);
    }
    
    if results.has_errors() {
        println!("\n{} Run with --fix to attempt automatic repairs", style("💡").blue());
    } else {
        println!("\n{} Workspace validation passed!", style("🎉").green());
    }
}
```

#### **Step 4: Info and Configuration Commands** (Day 4)
```rust
// src/commands/info.rs
use workspace_tools::{workspace, Workspace};
use anyhow::Result;
use console::style;
use std::collections::HashMap;

pub fn run(
    verbose: bool,
    show_config: bool,
    show_stats: bool,
    format: crate::OutputFormat,
) -> Result<()> {
    let ws = workspace()?;
    let info = gather_workspace_info(&ws, verbose, show_config, show_stats)?;
    
    match format {
        crate::OutputFormat::Text => display_info_text(&info),
        crate::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&info)?);
        }
    }
    
    Ok(())
}

#[derive(Debug, serde::Serialize)]
struct WorkspaceInfo {
    workspace_root: String,
    is_cargo_workspace: bool,
    directories: HashMap<String, DirectoryInfo>,
    configurations: Vec<ConfigInfo>,
    statistics: Option<WorkspaceStats>,
    cargo_metadata: Option<CargoInfo>,
}

#[derive(Debug, serde::Serialize)]
struct DirectoryInfo {
    path: String,
    exists: bool,
    file_count: Option<usize>,
    size_bytes: Option<u64>,
}

#[derive(Debug, serde::Serialize)]
struct ConfigInfo {
    name: String,
    path: String,
    format: String,
    size_bytes: u64,
    valid: bool,
}

#[derive(Debug, serde::Serialize)]
struct WorkspaceStats {
    total_files: usize,
    total_size_bytes: u64,
    file_types: HashMap<String, usize>,
    largest_files: Vec<FileInfo>,
}

#[derive(Debug, serde::Serialize)]
struct FileInfo {
    path: String,
    size_bytes: u64,
}

#[derive(Debug, serde::Serialize)]
struct CargoInfo {
    workspace_members: Vec<String>,
    dependencies: HashMap<String, String>,
}

fn gather_workspace_info(
    ws: &Workspace,
    verbose: bool,
    show_config: bool,
    show_stats: bool,
) -> Result<WorkspaceInfo> {
    let mut info = WorkspaceInfo {
        workspace_root: ws.root().display().to_string(),
        is_cargo_workspace: ws.is_cargo_workspace(),
        directories: HashMap::new(),
        configurations: Vec::new(),
        statistics: None,
        cargo_metadata: None,
    };
    
    // Gather directory information
    let standard_dirs = vec![
        ("config", ws.config_dir()),
        ("data", ws.data_dir()),
        ("logs", ws.logs_dir()),
        ("docs", ws.docs_dir()),
        ("tests", ws.tests_dir()),
        ("workspace", ws.workspace_dir()),
    ];
    
    for (name, path) in standard_dirs {
        let dir_info = if verbose || path.exists() {
            DirectoryInfo {
                path: path.display().to_string(),
                exists: path.exists(),
                file_count: if path.exists() { count_files_in_directory(&path).ok() } else { None },
                size_bytes: if path.exists() { calculate_directory_size(&path).ok() } else { None },
            }
        } else {
            DirectoryInfo {
                path: path.display().to_string(),
                exists: false,
                file_count: None,
                size_bytes: None,
            }
        };
        
        info.directories.insert(name.to_string(), dir_info);
    }
    
    // Gather configuration information
    if show_config {
        info.configurations = gather_config_info(ws)?;
    }
    
    // Gather workspace statistics
    if show_stats {
        info.statistics = gather_workspace_stats(ws).ok();
    }
    
    // Gather Cargo metadata
    if info.is_cargo_workspace {
        info.cargo_metadata = gather_cargo_info(ws).ok();
    }
    
    Ok(info)
}

// Implementation of helper functions...
fn count_files_in_directory(path: &std::path::Path) -> Result<usize> {
    let mut count = 0;
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            count += 1;
        }
    }
    Ok(count)
}

fn calculate_directory_size(path: &std::path::Path) -> Result<u64> {
    let mut total_size = 0;
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_file() {
            total_size += metadata.len();
        } else if metadata.is_dir() {
            total_size += calculate_directory_size(&entry.path())?;
        }
    }
    Ok(total_size)
}

fn gather_config_info(ws: &Workspace) -> Result<Vec<ConfigInfo>> {
    let config_dir = ws.config_dir();
    let mut configs = Vec::new();
    
    if !config_dir.exists() {
        return Ok(configs);
    }
    
    for entry in std::fs::read_dir(config_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if matches!(ext, "toml" | "yaml" | "yml" | "json") {
                    let metadata = path.metadata()?;
                    let name = path.file_stem()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();
                    
                    // Quick validation check
                    let valid = match ext {
                        "toml" => {
                            std::fs::read_to_string(&path)
                                .and_then(|content| toml::from_str::<toml::Value>(&content).map_err(|e| e.into()))
                                .is_ok()
                        }
                        "json" => {
                            std::fs::read_to_string(&path)
                                .and_then(|content| serde_json::from_str::<serde_json::Value>(&content).map_err(|e| e.into()))
                                .is_ok()
                        }
                        "yaml" | "yml" => {
                            std::fs::read_to_string(&path)
                                .and_then(|content| serde_yaml::from_str::<serde_yaml::Value>(&content).map_err(|e| e.into()))
                                .is_ok()
                        }
                        _ => false,
                    };
                    
                    configs.push(ConfigInfo {
                        name,
                        path: path.display().to_string(),
                        format: ext.to_string(),
                        size_bytes: metadata.len(),
                        valid,
                    });
                }
            }
        }
    }
    
    Ok(configs)
}

fn display_info_text(info: &WorkspaceInfo) {
    println!("{} Workspace Information", style("📊").cyan());
    println!("{}", "=".repeat(60));
    
    println!("\n{} Basic Info:", style("🏠").blue());
    println!("  Root: {}", style(&info.workspace_root).yellow());
    println!("  Type: {}", 
        if info.is_cargo_workspace {
            style("Cargo Workspace").green()
        } else {
            style("Standard Workspace").yellow()
        }
    );
    
    println!("\n{} Directory Structure:", style("📁").blue());
    for (name, dir_info) in &info.directories {
        let status = if dir_info.exists {
            style("✓").green()
        } else {
            style("✗").red()
        };
        
        print!("  {} {}", status, style(name).bold());
        
        if dir_info.exists {
            if let Some(file_count) = dir_info.file_count {
                print!(" ({} files", file_count);
                if let Some(size) = dir_info.size_bytes {
                    print!(", {} bytes", format_bytes(size));
                }
                print!(")");
            }
        }
        println!();
    }
    
    if !info.configurations.is_empty() {
        println!("\n{} Configuration Files:", style("⚙️").blue());
        for config in &info.configurations {
            let status = if config.valid {
                style("✓").green()
            } else {
                style("✗").red()
            };
            println!("  {} {} ({}, {} bytes)", 
                status, 
                style(&config.name).bold(),
                config.format,
                format_bytes(config.size_bytes)
            );
        }
    }
    
    if let Some(stats) = &info.statistics {
        println!("\n{} Statistics:", style("📈").blue());
        println!("  Total files: {}", stats.total_files);
        println!("  Total size: {}", format_bytes(stats.total_size_bytes));
        
        if !stats.file_types.is_empty() {
            println!("  File types:");
            for (ext, count) in &stats.file_types {
                println!("    {}: {}", ext, count);
            }
        }
    }
    
    if let Some(cargo) = &info.cargo_metadata {
        println!("\n{} Cargo Information:", style("📦").blue());
        println!("  Workspace members: {}", cargo.workspace_members.len());
        for member in &cargo.workspace_members {
            println!("    • {}", member);
        }
    }
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}
```

#### **Step 5: Scaffolding and Doctor Commands** (Day 5)
```rust
// src/commands/scaffold.rs
use workspace_tools::{workspace, TemplateType};
use anyhow::Result;
use console::style;
use dialoguer::{Input, Confirm};

pub fn run(
    template: String,
    interactive: bool,
    name: Option<String>,
    format: crate::OutputFormat,
) -> Result<()> {
    let ws = workspace()?;
    
    let template_type = crate::utils::parse_template_type(&template)?;
    let component_name = if let Some(name) = name {
        name
    } else if interactive {
        prompt_for_component_name(&template_type)?
    } else {
        return Err(anyhow::anyhow!("Component name is required when not in interactive mode"));
    };
    
    println!("{} Scaffolding {} component: {}", 
        style("🏗️").cyan(),
        style(template_type.name()).yellow(),
        style(&component_name).green()
    );
    
    // Create component-specific directory structure
    create_component_structure(&ws, &template_type, &component_name, interactive)?;
    
    match format {
        crate::OutputFormat::Text => {
            println!("\n{} Component scaffolded successfully!", style("✅").green());
            println!("  Name: {}", style(&component_name).yellow());
            println!("  Type: {}", style(template_type.name()).yellow());
        }
        crate::OutputFormat::Json => {
            let result = serde_json::json!({
                "status": "success",
                "component_name": component_name,
                "template_type": template_type.name(),
            });
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
    }
    
    Ok(())
}

// src/commands/doctor.rs
use workspace_tools::{workspace, Workspace};
use anyhow::Result;
use console::style;
use std::collections::HashMap;

pub fn run(
    fix: bool,
    check: Vec<String>,
    format: crate::OutputFormat,
) -> Result<()> {
    let ws = workspace()?;
    
    println!("{} Running workspace health diagnostics...", style("🏥").cyan());
    
    let mut diagnostics = WorkspaceDiagnostics::new();
    
    // Run all checks or specific ones
    let checks_to_run = if check.is_empty() {
        vec!["structure", "config", "permissions", "cargo", "git"]
    } else {
        check.iter().map(|s| s.as_str()).collect()
    };
    
    for check_name in checks_to_run {
        match check_name {
            "structure" => check_structure(&ws, &mut diagnostics, fix)?,
            "config" => check_configurations(&ws, &mut diagnostics, fix)?,
            "permissions" => check_permissions(&ws, &mut diagnostics, fix)?,
            "cargo" => check_cargo_setup(&ws, &mut diagnostics, fix)?,
            "git" => check_git_setup(&ws, &mut diagnostics, fix)?,
            _ => eprintln!("Unknown check: {}", check_name),
        }
    }
    
    // Display results
    match format {
        crate::OutputFormat::Text => display_diagnostics(&diagnostics),
        crate::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&diagnostics)?);
        }
    }
    
    if diagnostics.has_critical_issues() {
        std::process::exit(1);
    }
    
    Ok(())
}

#[derive(Debug, serde::Serialize)]
struct WorkspaceDiagnostics {
    checks_run: Vec<String>,
    issues: Vec<DiagnosticIssue>,
    fixes_applied: Vec<String>,
    summary: DiagnosticSummary,
}

#[derive(Debug, serde::Serialize)]
struct DiagnosticIssue {
    category: String,
    severity: IssueSeverity,
    description: String,
    fix_available: bool,
    fix_description: Option<String>,
}

#[derive(Debug, serde::Serialize)]
enum IssueSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, serde::Serialize)]
struct DiagnosticSummary {
    total_checks: usize,
    issues_found: usize,
    fixes_applied: usize,
    health_score: f32, // 0.0 to 100.0
}

impl WorkspaceDiagnostics {
    fn new() -> Self {
        Self {
            checks_run: Vec::new(),
            issues: Vec::new(),
            fixes_applied: Vec::new(),
            summary: DiagnosticSummary {
                total_checks: 0,
                issues_found: 0,
                fixes_applied: 0,
                health_score: 100.0,
            },
        }
    }
    
    fn add_check(&mut self, check_name: &str) {
        self.checks_run.push(check_name.to_string());
        self.summary.total_checks += 1;
    }
    
    fn add_issue(&mut self, issue: DiagnosticIssue) {
        self.summary.issues_found += 1;
        
        // Adjust health score based on severity
        let score_impact = match issue.severity {
            IssueSeverity::Info => 1.0,
            IssueSeverity::Warning => 5.0,
            IssueSeverity::Error => 15.0,
            IssueSeverity::Critical => 30.0,
        };
        
        self.summary.health_score = (self.summary.health_score - score_impact).max(0.0);
        self.issues.push(issue);
    }
    
    fn add_fix(&mut self, description: &str) {
        self.fixes_applied.push(description.to_string());
        self.summary.fixes_applied += 1;
    }
    
    fn has_critical_issues(&self) -> bool {
        self.issues.iter().any(|issue| matches!(issue.severity, IssueSeverity::Critical))
    }
}

fn display_diagnostics(diagnostics: &WorkspaceDiagnostics) {
    println!("\n{} Workspace Health Report", style("📋").cyan());
    println!("{}", "=".repeat(50));
    
    // Health score
    let score_color = if diagnostics.summary.health_score >= 90.0 {
        style(format!("{:.1}%", diagnostics.summary.health_score)).green()
    } else if diagnostics.summary.health_score >= 70.0 {
        style(format!("{:.1}%", diagnostics.summary.health_score)).yellow()
    } else {
        style(format!("{:.1}%", diagnostics.summary.health_score)).red()
    };
    
    println!("\n{} Health Score: {}", style("🏥").blue(), score_color);
    
    // Issues by severity
    let mut issues_by_severity: HashMap<String, Vec<&DiagnosticIssue>> = HashMap::new();
    
    for issue in &diagnostics.issues {
        let severity_str = match issue.severity {
            IssueSeverity::Info => "Info",
            IssueSeverity::Warning => "Warning",
            IssueSeverity::Error => "Error", 
            IssueSeverity::Critical => "Critical",
        };
        issues_by_severity.entry(severity_str.to_string()).or_default().push(issue);
    }
    
    if !diagnostics.issues.is_empty() {
        println!("\n{} Issues Found:", style("⚠️").blue());
        
        for severity in &["Critical", "Error", "Warning", "Info"] {
            if let Some(issues) = issues_by_severity.get(*severity) {
                for issue in issues {
                    let icon = match issue.severity {
                        IssueSeverity::Critical => style("🔴").red(),
                        IssueSeverity::Error => style("🔴").red(),
                        IssueSeverity::Warning => style("🟡").yellow(),
                        IssueSeverity::Info => style("🔵").blue(),
                    };
                    
                    println!("  {} [{}] {}: {}", 
                        icon, 
                        issue.category, 
                        severity,
                        issue.description
                    );
                    
                    if issue.fix_available {
                        if let Some(fix_desc) = &issue.fix_description {
                            println!("    {} Fix: {}", style("🔧").cyan(), fix_desc);
                        }
                    }
                }
            }
        }
    }
    
    // Fixes applied
    if !diagnostics.fixes_applied.is_empty() {
        println!("\n{} Fixes Applied:", style("🔧").green());
        for fix in &diagnostics.fixes_applied {
            println!("  {} {}", style("✓").green(), fix);
        }
    }
    
    // Summary
    println!("\n{} Summary:", style("📊").blue());
    println!("  Checks run: {}", diagnostics.summary.total_checks);
    println!("  Issues found: {}", diagnostics.summary.issues_found);
    println!("  Fixes applied: {}", diagnostics.summary.fixes_applied);
    
    if diagnostics.has_critical_issues() {
        println!("\n{} Critical issues found! Please address them before continuing.", 
            style("🚨").red().bold()
        );
    } else if diagnostics.summary.health_score >= 90.0 {
        println!("\n{} Workspace health is excellent!", style("🎉").green());
    } else if diagnostics.summary.health_score >= 70.0 {
        println!("\n{} Workspace health is good with room for improvement.", style("👍").yellow());
    } else {
        println!("\n{} Workspace health needs attention.", style("⚠️").red());
    }
}
```

#### **Step 6: Testing and Packaging** (Day 6)
```rust
// tests/integration_tests.rs
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_init_command() {
    let temp_dir = TempDir::new().unwrap();
    
    let mut cmd = Command::cargo_bin("cargo-workspace-tools").unwrap();
    cmd.args(&["init", "--template", "lib", "--quiet"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("initialized successfully"));
    
    // Verify structure was created
    assert!(temp_dir.path().join("Cargo.toml").exists());
    assert!(temp_dir.path().join("src").exists());
    assert!(temp_dir.path().join(".cargo/config.toml").exists());
}

#[test]
fn test_validate_command() {
    let temp_dir = TempDir::new().unwrap();
    
    // Initialize workspace first
    Command::cargo_bin("cargo-workspace-tools").unwrap()
        .args(&["init", "--template", "lib", "--quiet"])
        .current_dir(&temp_dir)
        .assert()
        .success();
    
    // Validate the workspace
    let mut cmd = Command::cargo_bin("cargo-workspace-tools").unwrap();
    cmd.args(&["validate"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("validation passed"));
}

#[test]
fn test_info_command() {
    let temp_dir = TempDir::new().unwrap();
    
    Command::cargo_bin("cargo-workspace-tools").unwrap()
        .args(&["init", "--template", "cli", "--quiet"])
        .current_dir(&temp_dir)
        .assert()
        .success();
        
    let mut cmd = Command::cargo_bin("cargo-workspace-tools").unwrap();
    cmd.args(&["info"])
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Workspace Information"))
        .stdout(predicate::str::contains("Cargo Workspace"));
}

// Cargo.toml additions for testing
[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"
tempfile = "3.0"
```

### **Documentation and Distribution**

#### **Installation Instructions**
```bash
# Install from crates.io
cargo install workspace-tools-cli

# Verify installation
cargo workspace-tools --help

# Initialize a new CLI project
cargo workspace-tools init my-cli-app --template=cli

# Validate workspace health
cargo workspace-tools validate

# Show workspace info
cargo workspace-tools info --config --stats
```

### **Success Criteria**
- [ ] Complete CLI with all major commands implemented
- [ ] Interactive and non-interactive modes
- [ ] JSON and text output formats
- [ ] Comprehensive validation and diagnostics
- [ ] Template scaffolding integration
- [ ] Configuration management commands
- [ ] Health check and auto-fix capabilities
- [ ] Cargo integration and workspace detection
- [ ] Comprehensive test suite
- [ ] Professional help text and error messages
- [ ] Published to crates.io

### **Future Enhancements**
- Shell completion support (bash, zsh, fish)
- Configuration file generation wizards
- Integration with VS Code and other IDEs
- Plugin system for custom commands
- Remote template repositories
- Workspace analytics and reporting
- CI/CD integration helpers

This CLI tool will be the primary way developers discover and interact with workspace_tools, significantly increasing its visibility and adoption in the Rust ecosystem.