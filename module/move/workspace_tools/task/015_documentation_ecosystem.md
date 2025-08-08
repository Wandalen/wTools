# Task 015: Documentation Ecosystem

**Priority**: 📚 High Impact  
**Phase**: 3-4 (Content & Community)  
**Estimated Effort**: 5-6 weeks  
**Dependencies**: Core features stable, Task 010 (CLI Tool)  

## **Objective**
Create a comprehensive documentation ecosystem that transforms workspace_tools from a useful library into a widely adopted standard by providing exceptional learning resources, best practices, and community-driven content that makes workspace management accessible to all Rust developers.

## **Strategic Documentation Goals**

### **Educational Impact**
- **Rust Book Integration**: Get workspace_tools patterns included as recommended practices
- **Learning Path**: From beginner to expert workspace management
- **Best Practices**: Establish industry standards for Rust workspace organization
- **Community Authority**: Become the definitive resource for workspace management

### **Adoption Acceleration**
- **Zero Barrier to Entry**: Anyone can understand and implement in 5 minutes
- **Progressive Disclosure**: Simple start, advanced features available when needed
- **Framework Integration**: Clear guides for every popular Rust framework
- **Enterprise Ready**: Documentation that satisfies corporate evaluation criteria

## **Technical Requirements**

### **Documentation Infrastructure**
1. **Multi-Platform Publishing**
   - docs.rs integration with custom styling
   - Standalone documentation website with search
   - PDF/ePub generation for offline reading
   - Mobile-optimized responsive design

2. **Interactive Learning**
   - Executable code examples in documentation
   - Interactive playground for testing concepts
   - Step-by-step tutorials with validation
   - Video content integration

3. **Community Contributions**
   - Easy contribution workflow for community examples
   - Translation support for non-English speakers
   - Versioned documentation with migration guides
   - Community-driven cookbook and patterns

## **Implementation Steps**

### **Phase 1: Foundation Documentation** (Weeks 1-2)

#### **Week 1: Core Documentation Structure**
```markdown
# Documentation Site Architecture

docs/
├── README.md                    # Main landing page
├── SUMMARY.md                   # mdBook table of contents
├── book/                        # Main documentation book
│   ├── introduction.md
│   ├── quickstart/
│   │   ├── installation.md
│   │   ├── first-workspace.md
│   │   └── basic-usage.md
│   ├── concepts/
│   │   ├── workspace-structure.md
│   │   ├── path-resolution.md
│   │   └── standard-directories.md
│   ├── guides/
│   │   ├── cli-applications.md
│   │   ├── web-services.md
│   │   ├── desktop-apps.md
│   │   └── libraries.md
│   ├── features/
│   │   ├── configuration.md
│   │   ├── templates.md
│   │   ├── secrets.md
│   │   └── async-operations.md
│   ├── integrations/
│   │   ├── frameworks/
│   │   │   ├── axum.md
│   │   │   ├── bevy.md
│   │   │   ├── tauri.md
│   │   │   └── leptos.md
│   │   ├── tools/
│   │   │   ├── docker.md
│   │   │   ├── ci-cd.md
│   │   │   └── ide-setup.md
│   │   └── deployment/
│   │       ├── cloud-platforms.md
│   │       └── containers.md
│   ├── cookbook/
│   │   ├── common-patterns.md
│   │   ├── testing-strategies.md
│   │   └── troubleshooting.md
│   ├── api/
│   │   ├── workspace.md
│   │   ├── configuration.md
│   │   └── utilities.md
│   └── contributing/
│       ├── development.md
│       ├── documentation.md
│       └── community.md
├── examples/                    # Comprehensive example projects
│   ├── hello-world/
│   ├── web-api-complete/
│   ├── desktop-app/
│   ├── cli-tool-advanced/
│   └── monorepo-enterprise/
└── assets/                      # Images, diagrams, videos
    ├── images/
    ├── diagrams/
    └── videos/
```

#### **Core Documentation Content**
```markdown
<!-- book/introduction.md -->
# Introduction to workspace_tools

Welcome to **workspace_tools** — the definitive solution for workspace-relative path resolution in Rust. 

## What is workspace_tools?

workspace_tools solves a fundamental problem that every Rust developer encounters: **reliable path resolution that works regardless of where your code runs**.

### The Problem

```rust
// ❌ These approaches are fragile and break easily:

// Relative paths break when execution context changes
let config = std::fs::read_to_string("../config/app.toml")?;

// Hardcoded paths aren't portable
let data = std::fs::read_to_string("/home/user/project/data/cache.db")?;

// Environment-dependent solutions require manual setup
let base = std::env::var("PROJECT_ROOT")?;
let config = std::fs::read_to_string(format!("{}/config/app.toml", base))?;
```

### The Solution

```rust
// ✅ workspace_tools provides reliable, context-independent paths:

use workspace_tools::workspace;

let ws = workspace()?;
let config = std::fs::read_to_string(ws.join("config/app.toml"))?;
let data = std::fs::read_to_string(ws.data_dir().join("cache.db"))?;

// Works perfectly whether called from:
// - Project root: cargo run
// - Subdirectory: cd src && cargo run  
// - IDE debug session
// - CI/CD pipeline
// - Container deployment
```

## Why workspace_tools?

### 🎯 **Zero Configuration** 
Works immediately with Cargo workspaces. No setup files needed.

### 🏗️ **Standard Layout**
Promotes consistent, predictable project structures across the Rust ecosystem.

### 🔒 **Security First**
Built-in secrets management with environment fallbacks.

### ⚡ **High Performance**
Optimized for minimal overhead, scales to large monorepos.

### 🧪 **Testing Ready**
Isolated workspace utilities make testing straightforward.

### 🌍 **Cross-Platform**
Handles Windows/macOS/Linux path differences automatically.

### 📦 **Framework Agnostic**
Works seamlessly with any Rust framework or architecture.

## Who Should Use This?

- **Application Developers**: CLI tools, web services, desktop apps
- **Library Authors**: Need reliable resource loading
- **DevOps Engineers**: Container and CI/CD deployments  
- **Team Leads**: Standardizing project structure across teams
- **Students & Educators**: Learning Rust best practices

## Quick Preview

Here's what a typical workspace_tools project looks like:

```
my-project/
├── Cargo.toml
├── src/
│   └── main.rs
├── config/                 # ← ws.config_dir()
│   ├── app.toml
│   └── database.yaml
├── data/                   # ← ws.data_dir()
│   └── cache.db
├── logs/                   # ← ws.logs_dir()
└── tests/                  # ← ws.tests_dir()
    └── integration_tests.rs
```

```rust
// src/main.rs
use workspace_tools::workspace;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws = workspace()?;
    
    // Load configuration
    let config_content = std::fs::read_to_string(
        ws.config_dir().join("app.toml")
    )?;
    
    // Initialize logging
    let log_path = ws.logs_dir().join("app.log");
    
    // Access data directory
    let cache_path = ws.data_dir().join("cache.db");
    
    println!("✅ Workspace initialized at: {}", ws.root().display());
    Ok(())
}
```

## What's Next?

Ready to get started? The [Quick Start Guide](./quickstart/installation.md) will have you up and running in 5 minutes.

Want to understand the concepts first? Check out [Core Concepts](./concepts/workspace-structure.md).

Looking for specific use cases? Browse our [Integration Guides](./integrations/frameworks/).

---

*💡 **Pro Tip**: workspace_tools follows the principle of "Convention over Configuration" — it works great with zero setup, but provides extensive customization when you need it.*
```

#### **Week 2: Interactive Examples System**
```rust
// docs/interactive_examples.rs - System for runnable documentation examples

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

pub struct InteractiveExample {
    pub id: String,
    pub title: String,
    pub description: String,
    pub setup_files: Vec<(PathBuf, String)>,
    pub main_code: String,
    pub expected_output: String,
    pub cleanup: bool,
}

impl InteractiveExample {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: String::new(),
            setup_files: Vec::new(),
            main_code: String::new(),
            expected_output: String::new(),
            cleanup: true,
        }
    }
    
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
    
    pub fn with_file(mut self, path: impl Into<PathBuf>, content: impl Into<String>) -> Self {
        self.setup_files.push((path.into(), content.into()));
        self
    }
    
    pub fn with_main_code(mut self, code: impl Into<String>) -> Self {
        self.main_code = code.into();
        self
    }
    
    pub fn with_expected_output(mut self, output: impl Into<String>) -> Self {
        self.expected_output = output.into();
        self
    }
    
    /// Execute the example in an isolated environment
    pub fn execute(&self) -> Result<ExecutionResult, Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let workspace_root = temp_dir.path();
        
        // Set up workspace structure
        self.setup_workspace(&workspace_root)?;
        
        // Create main.rs with the example code
        let main_rs = workspace_root.join("src/main.rs");
        std::fs::create_dir_all(main_rs.parent().unwrap())?;
        std::fs::write(&main_rs, &self.main_code)?;
        
        // Run the example
        let output = Command::new("cargo")
            .args(&["run", "--quiet"])
            .current_dir(&workspace_root)
            .output()?;
        
        let result = ExecutionResult {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            expected_output: self.expected_output.clone(),
        };
        
        Ok(result)
    }
    
    fn setup_workspace(&self, root: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // Create Cargo.toml
        let cargo_toml = r#"[package]
name = "workspace-tools-example"
version = "0.1.0"
edition = "2021"

[dependencies]
workspace_tools = { path = "../../../../" }
"#;
        std::fs::write(root.join("Cargo.toml"), cargo_toml)?;
        
        // Create setup files
        for (file_path, content) in &self.setup_files {
            let full_path = root.join(file_path);
            if let Some(parent) = full_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(full_path, content)?;
        }
        
        Ok(())
    }
}

#[derive(Debug)]
pub struct ExecutionResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub expected_output: String,
}

impl ExecutionResult {
    pub fn matches_expected(&self) -> bool {
        if self.expected_output.is_empty() {
            self.success
        } else {
            self.success && self.stdout.trim() == self.expected_output.trim()
        }
    }
}

// Example definitions for documentation
pub fn create_basic_examples() -> Vec<InteractiveExample> {
    vec![
        InteractiveExample::new("hello_workspace", "Hello Workspace")
            .with_description("Basic workspace_tools usage - your first workspace-aware application")
            .with_file("config/greeting.toml", r#"message = "Hello from workspace_tools!"
name = "Developer""#)
            .with_main_code(r#"use workspace_tools::workspace;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws = workspace()?;
    
    println!("🚀 Workspace root: {}", ws.root().display());
    println!("📁 Config directory: {}", ws.config_dir().display());
    
    // Read configuration
    let config_path = ws.config_dir().join("greeting.toml");
    if config_path.exists() {
        let config = std::fs::read_to_string(config_path)?;
        println!("📄 Config content:\n{}", config);
    }
    
    println!("✅ Successfully accessed workspace!");
    Ok(())
}"#)
            .with_expected_output("✅ Successfully accessed workspace!"),
            
        InteractiveExample::new("standard_directories", "Standard Directories")
            .with_description("Using workspace_tools standard directory layout")
            .with_file("data/users.json", r#"{"users": [{"name": "Alice"}, {"name": "Bob"}]}"#)
            .with_file("logs/.gitkeep", "")
            .with_main_code(r#"use workspace_tools::workspace;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws = workspace()?;
    
    // Demonstrate all standard directories
    println!("📂 Standard Directories:");
    println!("  Config: {}", ws.config_dir().display());
    println!("  Data: {}", ws.data_dir().display());
    println!("  Logs: {}", ws.logs_dir().display());
    println!("  Docs: {}", ws.docs_dir().display());
    println!("  Tests: {}", ws.tests_dir().display());
    
    // Check which directories exist
    let directories = [
        ("config", ws.config_dir()),
        ("data", ws.data_dir()),
        ("logs", ws.logs_dir()),
        ("docs", ws.docs_dir()),
        ("tests", ws.tests_dir()),
    ];
    
    println!("\n📊 Directory Status:");
    for (name, path) in directories {
        let exists = path.exists();
        let status = if exists { "✅" } else { "❌" };
        println!("  {} {}: {}", status, name, path.display());
    }
    
    // Read data file
    let data_file = ws.data_dir().join("users.json");
    if data_file.exists() {
        let users = std::fs::read_to_string(data_file)?;
        println!("\n📄 Data file content:\n{}", users);
    }
    
    Ok(())
}"#),
            
        InteractiveExample::new("configuration_loading", "Configuration Loading")
            .with_description("Loading and validating configuration files")
            .with_file("config/app.toml", r#"[application]
name = "MyApp"
version = "1.0.0"
debug = true

[database]
host = "localhost"
port = 5432
name = "myapp_db"

[server]
port = 8080
workers = 4"#)
            .with_main_code(r#"use workspace_tools::workspace;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws = workspace()?;
    
    // Find configuration file (supports .toml, .yaml, .json)
    match ws.find_config("app") {
        Ok(config_path) => {
            println!("📄 Found config: {}", config_path.display());
            
            let content = std::fs::read_to_string(config_path)?;
            println!("\n📋 Configuration content:");
            println!("{}", content);
            
            // In a real application, you'd deserialize this with serde
            println!("✅ Configuration loaded successfully!");
        }
        Err(e) => {
            println!("❌ No configuration found: {}", e);
            println!("💡 Expected files: config/app.{{toml,yaml,json}} or .app.toml");
        }
    }
    
    Ok(())
}"#),
    ]
}

// Test runner for all examples
pub fn test_all_examples() -> Result<(), Box<dyn std::error::Error>> {
    let examples = create_basic_examples();
    let mut passed = 0;
    let mut failed = 0;
    
    println!("🧪 Running interactive examples...\n");
    
    for example in &examples {
        print!("Testing '{}': ", example.title);
        
        match example.execute() {
            Ok(result) => {
                if result.matches_expected() {
                    println!("✅ PASSED");
                    passed += 1;
                } else {
                    println!("❌ FAILED");
                    println!("  Expected: {}", result.expected_output);
                    println!("  Got: {}", result.stdout);
                    if !result.stderr.is_empty() {
                        println!("  Error: {}", result.stderr);
                    }
                    failed += 1;
                }
            }
            Err(e) => {
                println!("❌ ERROR: {}", e);
                failed += 1;
            }
        }
    }
    
    println!("\n📊 Results: {} passed, {} failed", passed, failed);
    
    if failed > 0 {
        Err("Some examples failed".into())
    } else {
        Ok(())
    }
}
```

### **Phase 2: Comprehensive Guides** (Weeks 3-4)

#### **Week 3: Framework Integration Guides**
```markdown
<!-- book/integrations/frameworks/axum.md -->
# Axum Web Service Integration

This guide shows you how to build a production-ready web service using [Axum](https://github.com/tokio-rs/axum) and workspace_tools for reliable configuration and asset management.

## Overview

By the end of this guide, you'll have a complete web service that:
- ✅ Uses workspace_tools for all path operations
- ✅ Loads configuration from multiple environments
- ✅ Serves static assets reliably
- ✅ Implements structured logging
- ✅ Handles secrets securely
- ✅ Works consistently across development, testing, and production

## Project Setup

Let's create a new Axum project with workspace_tools:

```bash
cargo new --bin my-web-service
cd my-web-service
```

Add dependencies to `Cargo.toml`:

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
tower = "0.4"
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
workspace_tools = { version = "0.2", features = ["serde_integration"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json"] }
```

## Workspace Structure

Create the standard workspace structure:

```bash
mkdir -p config data logs assets/static
```

Your project should now look like:

```
my-web-service/
├── Cargo.toml
├── src/
│   └── main.rs
├── config/              # Configuration files
├── data/               # Application data
├── logs/               # Application logs  
├── assets/
│   └── static/         # Static web assets
└── tests/              # Integration tests
```

## Configuration Management

Create configuration files for different environments:

**`config/app.toml`** (base configuration):
```toml
[server]
host = "127.0.0.1"
port = 3000
workers = 4

[database]
url = "postgresql://localhost/myapp_dev"
max_connections = 10
timeout_seconds = 30

[logging]
level = "info"
format = "json"

[assets]
static_dir = "assets/static"
```

**`config/app.production.toml`** (production overrides):
```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 8

[database]
url = "${DATABASE_URL}"
max_connections = 20

[logging]
level = "warn"
```

## Application Code

Here's the complete application implementation:

**`src/config.rs`**:
```rust
use serde::{Deserialize, Serialize};
use workspace_tools::Workspace;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub assets: AssetsConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub timeout_seconds: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AssetsConfig {
    pub static_dir: String,
}

impl AppConfig {
    pub fn load(workspace: &Workspace) -> Result<Self, Box<dyn std::error::Error>> {
        // Determine environment
        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
        
        // Load base config
        let base_config_path = workspace.find_config("app")?;
        let mut config: AppConfig = {
            let content = std::fs::read_to_string(&base_config_path)?;
            toml::from_str(&content)?
        };
        
        // Load environment-specific overrides
        let env_config_path = workspace.join(format!("config/app.{}.toml", env));
        if env_config_path.exists() {
            let env_content = std::fs::read_to_string(&env_config_path)?;
            let env_config: AppConfig = toml::from_str(&env_content)?;
            
            // Simple merge (in production, you'd want more sophisticated merging)
            config.server = env_config.server;
            if !env_config.database.url.is_empty() {
                config.database = env_config.database;
            }
            config.logging = env_config.logging;
        }
        
        // Substitute environment variables
        config.database.url = substitute_env_vars(&config.database.url);
        
        Ok(config)
    }
}

fn substitute_env_vars(input: &str) -> String {
    let mut result = input.to_string();
    
    // Simple ${VAR} substitution
    while let Some(start) = result.find("${") {
        if let Some(end) = result[start..].find('}') {
            let var_name = &result[start + 2..start + end];
            if let Ok(var_value) = std::env::var(var_name) {
                result.replace_range(start..start + end + 1, &var_value);
            } else {
                break; // Avoid infinite loop on missing vars
            }
        } else {
            break;
        }
    }
    
    result
}
```

**`src/main.rs`**:
```rust
mod config;

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tracing::{info, instrument};
use workspace_tools::workspace;

use config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    config: Arc<AppConfig>,
    workspace: Arc<workspace_tools::Workspace>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize workspace
    let ws = workspace()?;
    info!("🚀 Initializing web service at: {}", ws.root().display());
    
    // Load configuration
    let config = Arc::new(AppConfig::load(&ws)?);
    info!("📄 Configuration loaded for environment: {}", 
          std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()));
    
    // Initialize logging
    initialize_logging(&ws, &config)?;
    
    // Create application state
    let state = AppState {
        config: config.clone(),
        workspace: Arc::new(ws),
    };
    
    // Create static file service
    let static_assets = ServeDir::new(state.workspace.join(&config.assets.static_dir));
    
    // Build router
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .route("/config", get(config_handler))
        .nest_service("/static", static_assets)
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(tower_http::trace::TraceLayer::new_for_http())
        );
    
    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    info!("🌐 Starting server on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

#[instrument(skip(state))]
async fn root_handler(State(state): State<AppState>) -> Json<Value> {
    Json(json!({
        "message": "Hello from workspace_tools + Axum!",
        "workspace_root": state.workspace.root().display().to_string(),
        "config_dir": state.workspace.config_dir().display().to_string(),
        "status": "ok"
    }))
}

#[instrument(skip(state))]
async fn health_handler(State(state): State<AppState>) -> (StatusCode, Json<Value>) {
    // Check workspace accessibility
    if !state.workspace.root().exists() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"status": "error", "message": "Workspace not accessible"}))
        );
    }
    
    // Check config directory
    if !state.workspace.config_dir().exists() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"status": "error", "message": "Config directory missing"}))
        );
    }
    
    (
        StatusCode::OK,
        Json(json!({
            "status": "healthy",
            "workspace": {
                "root": state.workspace.root().display().to_string(),
                "config_accessible": state.workspace.config_dir().exists(),
                "data_accessible": state.workspace.data_dir().exists(),
                "logs_accessible": state.workspace.logs_dir().exists(),
            }
        }))
    )
}

#[instrument(skip(state))]
async fn config_handler(State(state): State<AppState>) -> Json<Value> {
    Json(json!({
        "server": {
            "host": state.config.server.host,
            "port": state.config.server.port,
            "workers": state.config.server.workers
        },
        "logging": {
            "level": state.config.logging.level,
            "format": state.config.logging.format
        },
        "workspace": {
            "root": state.workspace.root().display().to_string(),
            "directories": {
                "config": state.workspace.config_dir().display().to_string(),
                "data": state.workspace.data_dir().display().to_string(),
                "logs": state.workspace.logs_dir().display().to_string(),
            }
        }
    }))
}

fn initialize_logging(ws: &workspace_tools::Workspace, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure logs directory exists
    std::fs::create_dir_all(ws.logs_dir())?;
    
    // Configure tracing based on config
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(match config.logging.level.as_str() {
            "trace" => tracing::Level::TRACE,
            "debug" => tracing::Level::DEBUG,
            "info" => tracing::Level::INFO,
            "warn" => tracing::Level::WARN,
            "error" => tracing::Level::ERROR,
            _ => tracing::Level::INFO,
        })
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)?;
    
    Ok(())
}
```

## Running the Application

### Development
```bash
cargo run
```

Visit:
- http://localhost:3000/ - Main endpoint
- http://localhost:3000/health - Health check
- http://localhost:3000/config - Configuration info

### Production
```bash
APP_ENV=production DATABASE_URL=postgresql://prod-server/myapp cargo run
```

## Testing

Create integration tests using workspace_tools:

**`tests/integration_test.rs`**:
```rust
use workspace_tools::testing::create_test_workspace_with_structure;

#[tokio::test]
async fn test_web_service_startup() {
    let (_temp_dir, ws) = create_test_workspace_with_structure();
    
    // Create test configuration
    let config_content = r#"
[server]
host = "127.0.0.1"
port = 0

[database]
url = "sqlite::memory:"
max_connections = 1
timeout_seconds = 5

[logging]
level = "debug"
format = "json"

[assets]
static_dir = "assets/static"
    "#;
    
    std::fs::write(ws.config_dir().join("app.toml"), config_content).unwrap();
    
    // Test configuration loading
    let config = my_web_service::config::AppConfig::load(&ws).unwrap();
    assert_eq!(config.server.host, "127.0.0.1");
    assert_eq!(config.database.max_connections, 1);
}
```

## Deployment with Docker

**`Dockerfile`**:
```dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary
COPY --from=builder /app/target/release/my-web-service /app/

# Copy workspace structure
COPY config/ ./config/
COPY assets/ ./assets/
RUN mkdir -p data logs

# Set environment
ENV WORKSPACE_PATH=/app
ENV APP_ENV=production

EXPOSE 8080
CMD ["./my-web-service"]
```

## Best Practices Summary

✅ **Configuration Management**
- Use layered configuration (base + environment)
- Environment variable substitution for secrets
- Validate configuration on startup

✅ **Static Assets**
- Use workspace-relative paths for assets
- Leverage Axum's `ServeDir` for static files
- Version assets in production

✅ **Logging**
- Initialize logs directory with workspace_tools
- Use structured logging (JSON in production)
- Configure log levels per environment

✅ **Health Checks**
- Verify workspace accessibility
- Check critical directories exist
- Return meaningful error messages

✅ **Testing**
- Use workspace_tools test utilities
- Test with isolated workspace environments
- Validate configuration loading

This integration shows how workspace_tools eliminates path-related issues in web services while promoting clean, maintainable architecture patterns.
```

#### **Week 4: Advanced Use Cases and Patterns**
```markdown
<!-- book/cookbook/common-patterns.md -->
# Common Patterns and Recipes

This cookbook contains battle-tested patterns for using workspace_tools in real-world scenarios. Each pattern includes complete code examples, explanations, and variations.

## Pattern 1: Configuration Hierarchies

**Problem**: You need different configurations for development, testing, staging, and production environments, with shared base settings and environment-specific overrides.

**Solution**: Use layered configuration files with workspace_tools:

```rust
use workspace_tools::Workspace;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub app: AppSettings,
    pub database: DatabaseSettings,
    pub cache: CacheSettings,
    pub features: FeatureFlags,
}

impl Config {
    pub fn load_for_environment(ws: &Workspace, env: &str) -> Result<Self, ConfigError> {
        let mut config_layers = Vec::new();
        
        // 1. Base configuration (always loaded)
        config_layers.push("base");
        
        // 2. Environment-specific configuration
        config_layers.push(env);
        
        // 3. Local overrides (for development)
        if env == "development" {
            config_layers.push("local");
        }
        
        // 4. Secret configuration (if exists)
        config_layers.push("secrets");
        
        Self::load_layered(ws, &config_layers)
    }
    
    fn load_layered(ws: &Workspace, layers: &[&str]) -> Result<Self, ConfigError> {
        let mut final_config: Option<Config> = None;
        
        for layer in layers {
            let config_name = if *layer == "base" { "config" } else { &format!("config.{}", layer) };
            
            match Self::load_single_config(ws, config_name) {
                Ok(layer_config) => {
                    final_config = Some(match final_config {
                        None => layer_config,
                        Some(base) => base.merge_with(layer_config)?,
                    });
                }
                Err(ConfigError::NotFound(_)) if *layer != "base" => {
                    // Optional layers can be missing
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
        
        final_config.ok_or(ConfigError::NotFound("base configuration".to_string()))
    }
    
    fn load_single_config(ws: &Workspace, name: &str) -> Result<Self, ConfigError> {
        let config_path = ws.find_config(name)
            .map_err(|_| ConfigError::NotFound(name.to_string()))?;
        
        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| ConfigError::ReadError(e.to_string()))?;
        
        // Support multiple formats
        let config = if config_path.extension().map_or(false, |ext| ext == "toml") {
            toml::from_str(&content)
        } else if config_path.extension().map_or(false, |ext| ext == "yaml" || ext == "yml") {
            serde_yaml::from_str(&content)
        } else {
            serde_json::from_str(&content)
        }.map_err(|e| ConfigError::ParseError(e.to_string()))?;
        
        Ok(config)
    }
    
    fn merge_with(mut self, other: Config) -> Result<Self, ConfigError> {
        // Merge strategies for different fields
        self.app = other.app; // Replace
        self.database = self.database.merge_with(other.database); // Selective merge
        self.cache = other.cache; // Replace
        self.features.merge_with(&other.features); // Additive merge
        
        Ok(self)
    }
}

// Usage example
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws = workspace_tools::workspace()?;
    let env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    
    let config = Config::load_for_environment(&ws, &env)?;
    println!("Loaded configuration for environment: {}", env);
    
    Ok(())
}
```

**File Structure**:
```
config/
├── config.toml              # Base configuration
├── config.development.toml  # Development overrides  
├── config.testing.toml      # Testing overrides
├── config.staging.toml      # Staging overrides
├── config.production.toml   # Production overrides
├── config.local.toml        # Local developer overrides (git-ignored)
└── config.secrets.toml      # Secrets (git-ignored)
```

## Pattern 2: Plugin Architecture

**Problem**: You want to build an extensible application where plugins can be loaded dynamically and have access to workspace resources.

**Solution**: Create a plugin system that provides workspace context:

```rust
use workspace_tools::Workspace;
use std::collections::HashMap;
use std::sync::Arc;

pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self, workspace: Arc<Workspace>) -> Result<(), PluginError>;
    fn execute(&self, context: &PluginContext) -> Result<PluginResult, PluginError>;
    fn shutdown(&mut self) -> Result<(), PluginError>;
}

pub struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
    workspace: Arc<Workspace>,
}

impl PluginManager {
    pub fn new(workspace: Workspace) -> Self {
        Self {
            plugins: HashMap::new(),
            workspace: Arc::new(workspace),
        }
    }
    
    pub fn load_plugins_from_directory(&mut self, plugin_dir: &str) -> Result<usize, PluginError> {
        let plugins_path = self.workspace.join(plugin_dir);
        
        if !plugins_path.exists() {
            std::fs::create_dir_all(&plugins_path)
                .map_err(|e| PluginError::IoError(e.to_string()))?;
            return Ok(0);
        }
        
        let mut loaded_count = 0;
        
        // Scan for plugin configuration files
        for entry in std::fs::read_dir(&plugins_path)
            .map_err(|e| PluginError::IoError(e.to_string()))? {
            
            let entry = entry.map_err(|e| PluginError::IoError(e.to_string()))?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "toml") {
                if let Ok(plugin) = self.load_plugin_from_config(&path) {
                    self.register_plugin(plugin)?;
                    loaded_count += 1;
                }
            }
        }
        
        Ok(loaded_count)
    }
    
    fn load_plugin_from_config(&self, config_path: &std::path::Path) -> Result<Box<dyn Plugin>, PluginError> {
        let config_content = std::fs::read_to_string(config_path)
            .map_err(|e| PluginError::IoError(e.to_string()))?;
        
        let plugin_config: PluginConfig = toml::from_str(&config_content)
            .map_err(|e| PluginError::ConfigError(e.to_string()))?;
        
        // Create plugin based on type
        match plugin_config.plugin_type.as_str() {
            "data_processor" => Ok(Box::new(DataProcessorPlugin::new(plugin_config)?)),
            "notification" => Ok(Box::new(NotificationPlugin::new(plugin_config)?)),
            "backup" => Ok(Box::new(BackupPlugin::new(plugin_config)?)),
            _ => Err(PluginError::UnknownPluginType(plugin_config.plugin_type))
        }
    }
    
    pub fn register_plugin(&mut self, mut plugin: Box<dyn Plugin>) -> Result<(), PluginError> {
        let name = plugin.name().to_string();
        
        // Initialize plugin with workspace context
        plugin.initialize(self.workspace.clone())?;
        
        self.plugins.insert(name, plugin);
        Ok(())
    }
    
    pub fn execute_plugin(&self, name: &str, context: &PluginContext) -> Result<PluginResult, PluginError> {
        let plugin = self.plugins.get(name)
            .ok_or_else(|| PluginError::PluginNotFound(name.to_string()))?;
        
        plugin.execute(context)
    }
    
    pub fn shutdown_all(&mut self) -> Result<(), PluginError> {
        for (name, plugin) in &mut self.plugins {
            if let Err(e) = plugin.shutdown() {
                eprintln!("Warning: Failed to shutdown plugin '{}': {}", name, e);
            }
        }
        self.plugins.clear();
        Ok(())
    }
}

// Example plugin implementation
pub struct DataProcessorPlugin {
    name: String,
    version: String,
    config: PluginConfig,
    workspace: Option<Arc<Workspace>>,
    input_dir: Option<std::path::PathBuf>,
    output_dir: Option<std::path::PathBuf>,
}

impl DataProcessorPlugin {
    fn new(config: PluginConfig) -> Result<Self, PluginError> {
        Ok(Self {
            name: config.name.clone(),
            version: config.version.clone(),
            config,
            workspace: None,
            input_dir: None,
            output_dir: None,
        })
    }
}

impl Plugin for DataProcessorPlugin {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    fn initialize(&mut self, workspace: Arc<Workspace>) -> Result<(), PluginError> {
        // Set up plugin-specific directories using workspace
        self.input_dir = Some(workspace.data_dir().join("input"));
        self.output_dir = Some(workspace.data_dir().join("output"));
        
        // Create directories if they don't exist
        if let Some(input_dir) = &self.input_dir {
            std::fs::create_dir_all(input_dir)
                .map_err(|e| PluginError::IoError(e.to_string()))?;
        }
        
        if let Some(output_dir) = &self.output_dir {
            std::fs::create_dir_all(output_dir)
                .map_err(|e| PluginError::IoError(e.to_string()))?;
        }
        
        self.workspace = Some(workspace);
        Ok(())
    }
    
    fn execute(&self, context: &PluginContext) -> Result<PluginResult, PluginError> {
        let workspace = self.workspace.as_ref()
            .ok_or(PluginError::NotInitialized)?;
        
        let input_dir = self.input_dir.as_ref().unwrap();
        let output_dir = self.output_dir.as_ref().unwrap();
        
        // Process files from input directory
        let mut processed_files = Vec::new();
        
        for entry in std::fs::read_dir(input_dir)
            .map_err(|e| PluginError::IoError(e.to_string()))? {
            
            let entry = entry.map_err(|e| PluginError::IoError(e.to_string()))?;
            let input_path = entry.path();
            
            if input_path.is_file() {
                let file_name = input_path.file_name().unwrap().to_string_lossy();
                let output_path = output_dir.join(format!("processed_{}", file_name));
                
                // Simple processing: read, transform, write
                let content = std::fs::read_to_string(&input_path)
                    .map_err(|e| PluginError::IoError(e.to_string()))?;
                
                let processed_content = self.process_content(&content);
                
                std::fs::write(&output_path, processed_content)
                    .map_err(|e| PluginError::IoError(e.to_string()))?;
                
                processed_files.push(output_path.to_string_lossy().to_string());
            }
        }
        
        Ok(PluginResult {
            success: true,
            message: format!("Processed {} files", processed_files.len()),
            data: Some(processed_files.into()),
        })
    }
    
    fn shutdown(&mut self) -> Result<(), PluginError> {
        // Cleanup plugin resources
        self.workspace = None;
        Ok(())
    }
}

impl DataProcessorPlugin {
    fn process_content(&self, content: &str) -> String {
        // Example processing: convert to uppercase and add timestamp
        format!("Processed at {}: {}", 
               chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
               content.to_uppercase())
    }
}

// Usage example
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws = workspace_tools::workspace()?;
    let mut plugin_manager = PluginManager::new(ws);
    
    // Load plugins from workspace
    let loaded_count = plugin_manager.load_plugins_from_directory("plugins")?;
    println!("Loaded {} plugins", loaded_count);
    
    // Execute a plugin
    let context = PluginContext::new();
    if let Ok(result) = plugin_manager.execute_plugin("data_processor", &context) {
        println!("Plugin result: {}", result.message);
    }
    
    // Cleanup
    plugin_manager.shutdown_all()?;
    
    Ok(())
}
```

**Plugin Configuration Example** (`plugins/data_processor.toml`):
```toml
name = "data_processor"
version = "1.0.0"
plugin_type = "data_processor"
description = "Processes data files in the workspace"

[settings]
batch_size = 100
timeout_seconds = 30

[permissions]
read_data = true
write_data = true
read_config = false
write_config = false
```

## Pattern 3: Multi-Workspace Monorepo

**Problem**: You have a large monorepo with multiple related projects that need to share resources and configuration while maintaining independence.

**Solution**: Create a workspace hierarchy with shared utilities:

```rust
use workspace_tools::Workspace;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct MonorepoManager {
    root_workspace: Workspace,
    sub_workspaces: HashMap<String, Workspace>,
    shared_config: SharedConfig,
}

impl MonorepoManager {
    pub fn new() -> Result<Self, MonorepoError> {
        let root_workspace = workspace_tools::workspace()?;
        
        // Verify this is a monorepo structure
        if !Self::is_monorepo_root(&root_workspace) {
            return Err(MonorepoError::NotMonorepo);
        }
        
        let shared_config = SharedConfig::load(&root_workspace)?;
        
        Ok(Self {
            root_workspace,
            sub_workspaces: HashMap::new(),
            shared_config,
        })
    }
    
    fn is_monorepo_root(ws: &Workspace) -> bool {
        // Check for monorepo indicators
        ws.join("workspace.toml").exists() || 
        ws.join("monorepo.json").exists() ||
        ws.join("projects").is_dir()
    }
    
    pub fn discover_sub_workspaces(&mut self) -> Result<Vec<String>, MonorepoError> {
        let projects_dir = self.root_workspace.join("projects");
        let mut discovered = Vec::new();
        
        if projects_dir.exists() {
            for entry in std::fs::read_dir(&projects_dir)
                .map_err(|e| MonorepoError::IoError(e.to_string()))? {
                
                let entry = entry.map_err(|e| MonorepoError::IoError(e.to_string()))?;
                let project_path = entry.path();
                
                if project_path.is_dir() {
                    let project_name = project_path.file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string();
                    
                    // Create workspace for this project
                    std::env::set_var("WORKSPACE_PATH", &project_path);
                    let sub_workspace = Workspace::resolve()
                        .map_err(|_| MonorepoError::InvalidSubWorkspace(project_name.clone()))?;
                    
                    self.sub_workspaces.insert(project_name.clone(), sub_workspace);
                    discovered.push(project_name);
                }
            }
        }
        
        // Restore original workspace path
        std::env::set_var("WORKSPACE_PATH", self.root_workspace.root());
        
        Ok(discovered)
    }
    
    pub fn get_sub_workspace(&self, name: &str) -> Option<&Workspace> {
        self.sub_workspaces.get(name)
    }
    
    pub fn execute_in_all_workspaces<F, R>(&self, mut operation: F) -> Vec<(String, Result<R, MonorepoError>)>
    where
        F: FnMut(&str, &Workspace) -> Result<R, MonorepoError>,
    {
        let mut results = Vec::new();
        
        // Execute in root workspace
        let root_result = operation("root", &self.root_workspace);
        results.push(("root".to_string(), root_result));
        
        // Execute in each sub-workspace
        for (name, workspace) in &self.sub_workspaces {
            let result = operation(name, workspace);
            results.push((name.clone(), result));
        }
        
        results
    }
    
    pub fn sync_shared_configuration(&self) -> Result<(), MonorepoError> {
        let shared_config_content = toml::to_string_pretty(&self.shared_config)
            .map_err(|e| MonorepoError::ConfigError(e.to_string()))?;
        
        // Write shared config to each sub-workspace
        for (name, workspace) in &self.sub_workspaces {
            let shared_config_path = workspace.config_dir().join("shared.toml");
            
            // Ensure config directory exists
            std::fs::create_dir_all(workspace.config_dir())
                .map_err(|e| MonorepoError::IoError(e.to_string()))?;
            
            std::fs::write(&shared_config_path, &shared_config_content)
                .map_err(|e| MonorepoError::IoError(e.to_string()))?;
            
            println!("Synced shared configuration to project: {}", name);
        }
        
        Ok(())
    }
    
    pub fn build_dependency_graph(&self) -> Result<DependencyGraph, MonorepoError> {
        let mut graph = DependencyGraph::new();
        
        // Add root workspace
        graph.add_node("root", &self.root_workspace);
        
        // Add sub-workspaces and their dependencies
        for (name, workspace) in &self.sub_workspaces {
            graph.add_node(name, workspace);
            
            // Parse Cargo.toml to find workspace dependencies
            let cargo_toml_path = workspace.join("Cargo.toml");
            if cargo_toml_path.exists() {
                let dependencies = self.parse_workspace_dependencies(&cargo_toml_path)?;
                for dep in dependencies {
                    if self.sub_workspaces.contains_key(&dep) {
                        graph.add_edge(name, &dep);
                    }
                }
            }
        }
        
        Ok(graph)
    }
    
    fn parse_workspace_dependencies(&self, cargo_toml_path: &Path) -> Result<Vec<String>, MonorepoError> {
        let content = std::fs::read_to_string(cargo_toml_path)
            .map_err(|e| MonorepoError::IoError(e.to_string()))?;
        
        let parsed: toml::Value = toml::from_str(&content)
            .map_err(|e| MonorepoError::ConfigError(e.to_string()))?;
        
        let mut workspace_deps = Vec::new();
        
        if let Some(dependencies) = parsed.get("dependencies").and_then(|d| d.as_table()) {
            for (dep_name, dep_config) in dependencies {
                if let Some(dep_table) = dep_config.as_table() {
                    if dep_table.get("path").is_some() {
                        // This is a local workspace dependency
                        workspace_deps.push(dep_name.clone());
                    }
                }
            }
        }
        
        Ok(workspace_deps)
    }
}

// Usage example for monorepo operations
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut monorepo = MonorepoManager::new()?;
    
    // Discover all sub-workspaces
    let projects = monorepo.discover_sub_workspaces()?;
    println!("Discovered projects: {:?}", projects);
    
    // Sync shared configuration
    monorepo.sync_shared_configuration()?;
    
    // Execute operation across all workspaces
    let results = monorepo.execute_in_all_workspaces(|name, workspace| {
        // Example: Check if tests directory exists
        let tests_exist = workspace.tests_dir().exists();
        Ok(format!("Tests directory exists: {}", tests_exist))
    });
    
    for (name, result) in results {
        match result {
            Ok(message) => println!("{}: {}", name, message),
            Err(e) => eprintln!("{}: Error - {}", name, e),
        }
    }
    
    // Build dependency graph
    let dep_graph = monorepo.build_dependency_graph()?;
    println!("Dependency graph: {:#?}", dep_graph);
    
    Ok(())
}
```

**Monorepo Structure**:
```
my-monorepo/
├── workspace.toml           # Monorepo configuration
├── config/                  # Shared configuration
│   ├── shared.toml
│   └── ci.yaml
├── scripts/                 # Shared build/deployment scripts
├── docs/                    # Monorepo-wide documentation
└── projects/                # Individual project workspaces
    ├── web-api/             # Project A
    │   ├── Cargo.toml
    │   ├── src/
    │   ├── config/
    │   └── tests/
    ├── mobile-client/       # Project B
    │   ├── Cargo.toml
    │   ├── src/
    │   ├── config/
    │   └── tests/
    └── shared-lib/          # Shared library
        ├── Cargo.toml
        ├── src/
        └── tests/
```

These patterns demonstrate how workspace_tools scales from simple applications to complex enterprise scenarios while maintaining clean, maintainable code organization.
```

### **Phase 3: Community Content Platform** (Weeks 5-6)

#### **Week 5: Interactive Documentation Platform**
```rust
// docs-platform/src/lib.rs - Interactive documentation platform

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, Json},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentationSite {
    pub title: String,
    pub description: String,
    pub sections: Vec<DocumentationSection>,
    pub examples: HashMap<String, InteractiveExample>,
    pub search_index: SearchIndex,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentationSection {
    pub id: String,
    pub title: String,
    pub content: String,
    pub subsections: Vec<DocumentationSection>,
    pub examples: Vec<String>, // Example IDs
    pub code_snippets: Vec<CodeSnippet>,
    pub metadata: SectionMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeSnippet {
    pub language: String,
    pub code: String,
    pub executable: bool,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SectionMetadata {
    pub difficulty: DifficultyLevel,
    pub estimated_reading_time: u32, // minutes
    pub prerequisites: Vec<String>,
    pub related_sections: Vec<String>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InteractiveExample {
    pub id: String,
    pub title: String,
    pub description: String,
    pub code: String,
    pub setup_files: Vec<(String, String)>,
    pub expected_output: Option<String>,
    pub explanation: String,
    pub difficulty: DifficultyLevel,
    pub tags: Vec<String>,
    pub run_count: u64,
    pub rating: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchIndex {
    pub sections: HashMap<String, SearchableSection>,
    pub examples: HashMap<String, SearchableExample>,
    pub keywords: HashMap<String, Vec<String>>, // keyword -> [section_ids]
}

// Web application state
#[derive(Clone)]
pub struct AppState {
    pub docs: Arc<RwLock<DocumentationSite>>,
    pub workspace: Arc<workspace_tools::Workspace>,
    pub example_runner: Arc<ExampleRunner>,
}

pub struct ExampleRunner {
    temp_dir: tempfile::TempDir,
}

impl ExampleRunner {
    pub fn new() -> Result<Self, std::io::Error> {
        Ok(Self {
            temp_dir: tempfile::TempDir::new()?,
        })
    }
    
    pub async fn run_example(&self, example: &InteractiveExample) -> Result<ExampleResult, String> {
        let example_dir = self.temp_dir.path().join(&example.id);
        tokio::fs::create_dir_all(&example_dir).await
            .map_err(|e| e.to_string())?;
        
        // Set up Cargo.toml
        let cargo_toml = r#"[package]
name = "interactive-example"
version = "0.1.0"
edition = "2021"

[dependencies]
workspace_tools = { path = "../../../../" }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
"#;
        
        tokio::fs::write(example_dir.join("Cargo.toml"), cargo_toml).await
            .map_err(|e| e.to_string())?;
        
        // Create src directory and main.rs
        tokio::fs::create_dir_all(example_dir.join("src")).await
            .map_err(|e| e.to_string())?;
        tokio::fs::write(example_dir.join("src/main.rs"), &example.code).await
            .map_err(|e| e.to_string())?;
        
        // Create setup files
        for (file_path, content) in &example.setup_files {
            let full_path = example_dir.join(file_path);
            if let Some(parent) = full_path.parent() {
                tokio::fs::create_dir_all(parent).await
                    .map_err(|e| e.to_string())?;
            }
            tokio::fs::write(full_path, content).await
                .map_err(|e| e.to_string())?;
        }
        
        // Execute the example
        let output = tokio::process::Command::new("cargo")
            .args(&["run", "--quiet"])
            .current_dir(&example_dir)
            .output()
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(ExampleResult {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            execution_time: std::time::Duration::from_secs(1), // TODO: measure actual time
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ExampleResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub execution_time: std::time::Duration,
}

// API handlers
pub async fn serve_documentation(
    Path(section_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, StatusCode> {
    let docs = state.docs.read().await;
    
    if let Some(section) = find_section(&docs.sections, &section_id) {
        let html = render_section_html(section, &docs.examples);
        Ok(Html(html))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn run_interactive_example(
    Path(example_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ExampleResult>, StatusCode> {
    let docs = state.docs.read().await;
    
    if let Some(example) = docs.examples.get(&example_id) {
        match state.example_runner.run_example(example).await {
            Ok(result) => Ok(Json(result)),
            Err(error) => {
                let error_result = ExampleResult {
                    success: false,
                    stdout: String::new(),
                    stderr: error,
                    execution_time: std::time::Duration::from_secs(0),
                };
                Ok(Json(error_result))
            }
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Deserialize)]
pub struct SearchQuery {
    q: String,
    filter: Option<String>,
    difficulty: Option<DifficultyLevel>,
}

pub async fn search_documentation(
    Query(query): Query<SearchQuery>,
    State(state): State<AppState>,
) -> Result<Json<SearchResults>, StatusCode> {
    let docs = state.docs.read().await;
    let results = search_content(&docs, &query.q, query.difficulty.as_ref());
    Ok(Json(results))
}

fn search_content(
    docs: &DocumentationSite,
    query: &str,
    difficulty_filter: Option<&DifficultyLevel>,
) -> SearchResults {
    let mut section_results = Vec::new();
    let mut example_results = Vec::new();
    
    let query_lower = query.to_lowercase();
    
    // Search sections
    search_sections_recursive(&docs.sections, &query_lower, &mut section_results);
    
    // Search examples
    for (id, example) in &docs.examples {
        if difficulty_filter.map_or(true, |filter| std::mem::discriminant(filter) == std::mem::discriminant(&example.difficulty)) {
            let relevance = calculate_example_relevance(example, &query_lower);
            if relevance > 0.0 {
                example_results.push(SearchResultItem {
                    id: id.clone(),
                    title: example.title.clone(),
                    excerpt: truncate_text(&example.description, 150),
                    relevance,
                    item_type: "example".to_string(),
                });
            }
        }
    }
    
    // Sort by relevance
    section_results.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());
    example_results.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());
    
    SearchResults {
        query: query.to_string(),
        total_results: section_results.len() + example_results.len(),
        sections: section_results,
        examples: example_results,
    }
}

#[derive(Debug, Serialize)]
pub struct SearchResults {
    pub query: String,
    pub total_results: usize,
    pub sections: Vec<SearchResultItem>,
    pub examples: Vec<SearchResultItem>,
}

#[derive(Debug, Serialize)]
pub struct SearchResultItem {
    pub id: String,
    pub title: String,
    pub excerpt: String,
    pub relevance: f32,
    pub item_type: String,
}

// HTML rendering functions
fn render_section_html(section: &DocumentationSection, examples: &HashMap<String, InteractiveExample>) -> String {
    format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - workspace_tools Documentation</title>
    <link href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.24.1/themes/prism.min.css" rel="stylesheet">
    <link href="/static/docs.css" rel="stylesheet">
</head>
<body>
    <nav class="sidebar">
        <div class="sidebar-header">
            <h2>workspace_tools</h2>
            <span class="version">v0.2.0</span>
        </div>
        <div class="search-box">
            <input type="text" id="search" placeholder="Search documentation...">
        </div>
        <!-- Navigation will be populated by JavaScript -->
    </nav>
    
    <main class="content">
        <article>
            <header>
                <h1>{}</h1>
                <div class="article-meta">
                    <span class="difficulty difficulty-{}">{:?}</span>
                    <span class="reading-time">{} min read</span>
                    <span class="last-updated">Updated {}</span>
                </div>
            </header>
            
            <div class="article-content">
                {}
            </div>
            
            {}
            
            {}
        </article>
    </main>
    
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.24.1/components/prism-core.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.24.1/plugins/autoloader/prism-autoloader.min.js"></script>
    <script src="/static/docs.js"></script>
</body>
</html>"#,
        section.title,
        section.title,
        format!("{:?}", section.metadata.difficulty).to_lowercase(),
        section.metadata.difficulty,
        section.metadata.estimated_reading_time,
        section.metadata.last_updated.format("%B %d, %Y"),
        markdown_to_html(&section.content),
        render_code_snippets(&section.code_snippets),
        render_interactive_examples(&section.examples, examples)
    )
}

fn render_code_snippets(snippets: &[CodeSnippet]) -> String {
    if snippets.is_empty() {
        return String::new();
    }
    
    let mut html = String::from(r#"<section class="code-examples">
        <h2>Code Examples</h2>"#);
    
    for (i, snippet) in snippets.iter().enumerate() {
        html.push_str(&format!(r#"
        <div class="code-example" data-index="{}">
            {}
            <pre><code class="language-{}">{}</code></pre>
            {}
        </div>"#,
            i,
            snippet.description.as_ref().map_or(String::new(), |desc| format!(r#"<p class="code-description">{}</p>"#, desc)),
            snippet.language,
            html_escape(&snippet.code),
            if snippet.executable {
                r#"<button class="run-code-btn" onclick="runCodeSnippet(this)">Run Code</button>"#
            } else {
                ""
            }
        ));
    }
    
    html.push_str("</section>");
    html
}

fn render_interactive_examples(example_ids: &[String], examples: &HashMap<String, InteractiveExample>) -> String {
    if example_ids.is_empty() {
        return String::new();
    }
    
    let mut html = String::from(r#"<section class="interactive-examples">
        <h2>Interactive Examples</h2>
        <div class="examples-grid">"#);
    
    for example_id in example_ids {
        if let Some(example) = examples.get(example_id) {
            html.push_str(&format!(r#"
            <div class="example-card" data-example-id="{}">
                <h3>{}</h3>
                <p>{}</p>
                <div class="example-meta">
                    <span class="difficulty difficulty-{}">{:?}</span>
                    <span class="tags">{}</span>
                </div>
                <button class="run-example-btn" onclick="runInteractiveExample('{}')">
                    Try It Out
                </button>
                <div class="example-result" style="display: none;"></div>
            </div>"#,
                example.id,
                example.title,
                truncate_text(&example.description, 120),
                format!("{:?}", example.difficulty).to_lowercase(),
                example.difficulty,
                example.tags.join(", "),
                example.id
            ));
        }
    }
    
    html.push_str("</div></section>");
    html
}

// Utility functions
fn find_section(sections: &[DocumentationSection], id: &str) -> Option<&DocumentationSection> {
    for section in sections {
        if section.id == id {
            return Some(section);
        }
        if let Some(found) = find_section(&section.subsections, id) {
            return Some(found);
        }
    }
    None
}

fn search_sections_recursive(
    sections: &[DocumentationSection],
    query: &str,
    results: &mut Vec<SearchResultItem>,
) {
    for section in sections {
        let relevance = calculate_section_relevance(section, query);
        if relevance > 0.0 {
            results.push(SearchResultItem {
                id: section.id.clone(),
                title: section.title.clone(),
                excerpt: truncate_text(&section.content, 150),
                relevance,
                item_type: "section".to_string(),
            });
        }
        search_sections_recursive(&section.subsections, query, results);
    }
}

fn calculate_section_relevance(section: &DocumentationSection, query: &str) -> f32 {
    let title_matches = section.title.to_lowercase().matches(query).count() as f32 * 3.0;
    let content_matches = section.content.to_lowercase().matches(query).count() as f32;
    
    title_matches + content_matches
}

fn calculate_example_relevance(example: &InteractiveExample, query: &str) -> f32 {
    let title_matches = example.title.to_lowercase().matches(query).count() as f32 * 3.0;
    let description_matches = example.description.to_lowercase().matches(query).count() as f32 * 2.0;
    let code_matches = example.code.to_lowercase().matches(query).count() as f32;
    let tag_matches = example.tags.iter()
        .map(|tag| tag.to_lowercase().matches(query).count() as f32)
        .sum::<f32>() * 2.0;
    
    title_matches + description_matches + code_matches + tag_matches
}

fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        format!("{}...", &text[..max_length.min(text.len())])
    }
}

fn markdown_to_html(markdown: &str) -> String {
    // TODO: Implement markdown to HTML conversion
    // For now, just return the markdown wrapped in <pre>
    format!("<pre>{}</pre>", html_escape(markdown))
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

// Create the documentation router
pub fn create_docs_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { Html(include_str!("../templates/index.html")) }))
        .route("/docs/:section_id", get(serve_documentation))
        .route("/api/examples/:example_id/run", get(run_interactive_example))
        .route("/api/search", get(search_documentation))
        .with_state(state)
}
```

#### **Week 6: Community Contribution System**
```rust
// community/src/lib.rs - Community contribution and feedback system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommunityContribution {
    pub id: Uuid,
    pub author: ContributionAuthor,
    pub contribution_type: ContributionType,
    pub title: String,
    pub description: String,
    pub content: ContributionContent,
    pub tags: Vec<String>,
    pub status: ContributionStatus,
    pub votes: VoteCount,
    pub reviews: Vec<CommunityReview>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContributionAuthor {
    pub username: String,
    pub display_name: String,
    pub email: Option<String>,
    pub github_handle: Option<String>,
    pub reputation: u32,
    pub contribution_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ContributionType {
    Documentation,
    Example,
    Tutorial,
    Pattern,
    Integration,
    BestPractice,
    Translation,
    BugReport,
    FeatureRequest,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ContributionContent {
    Markdown { content: String },
    Code { language: String, code: String, description: String },
    Example { code: String, setup_files: Vec<(String, String)>, explanation: String },
    Integration { framework: String, guide: String, code_samples: Vec<CodeSample> },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodeSample {
    pub filename: String,
    pub language: String,
    pub code: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ContributionStatus {
    Draft,
    Submitted,
    UnderReview,
    Approved,
    Published,
    NeedsRevision,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VoteCount {
    pub upvotes: u32,
    pub downvotes: u32,
}

impl VoteCount {
    pub fn score(&self) -> i32 {
        self.upvotes as i32 - self.downvotes as i32
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommunityReview {
    pub id: Uuid,
    pub reviewer: String,
    pub rating: ReviewRating,
    pub feedback: String,
    pub suggestions: Vec<ReviewSuggestion>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReviewRating {
    Excellent,
    Good,
    NeedsImprovement,
    Poor,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReviewSuggestion {
    pub suggestion_type: SuggestionType,
    pub description: String,
    pub code_change: Option<CodeChange>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SuggestionType {
    CodeImprovement,
    ClarificationNeeded,
    AddExample,
    FixTypo,
    UpdateDocumentation,
    SecurityConcern,
    PerformanceIssue,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodeChange {
    pub file_path: String,
    pub original: String,
    pub suggested: String,
    pub reason: String,
}

pub struct CommunityManager {
    contributions: HashMap<Uuid, CommunityContribution>,
    authors: HashMap<String, ContributionAuthor>,
    workspace: workspace_tools::Workspace,
}

impl CommunityManager {
    pub fn new(workspace: workspace_tools::Workspace) -> Self {
        Self {
            contributions: HashMap::new(),
            authors: HashMap::new(),
            workspace,
        }
    }
    
    pub fn load_from_workspace(&mut self) -> Result<(), CommunityError> {
        let community_dir = self.workspace.join("community");
        
        if !community_dir.exists() {
            std::fs::create_dir_all(&community_dir)
                .map_err(|e| CommunityError::IoError(e.to_string()))?;
            return Ok(());
        }
        
        // Load contributions
        let contributions_dir = community_dir.join("contributions");
        if contributions_dir.exists() {
            for entry in std::fs::read_dir(&contributions_dir)
                .map_err(|e| CommunityError::IoError(e.to_string()))? {
                
                let entry = entry.map_err(|e| CommunityError::IoError(e.to_string()))?;
                if entry.path().extension().map_or(false, |ext| ext == "json") {
                    let contribution = self.load_contribution(&entry.path())?;
                    self.contributions.insert(contribution.id, contribution);
                }
            }
        }
        
        // Load authors
        let authors_file = community_dir.join("authors.json");
        if authors_file.exists() {
            let content = std::fs::read_to_string(&authors_file)
                .map_err(|e| CommunityError::IoError(e.to_string()))?;
            self.authors = serde_json::from_str(&content)
                .map_err(|e| CommunityError::ParseError(e.to_string()))?;
        }
        
        Ok(())
    }
    
    pub fn submit_contribution(&mut self, mut contribution: CommunityContribution) -> Result<Uuid, CommunityError> {
        // Assign ID and set timestamps
        contribution.id = Uuid::new_v4();
        contribution.created_at = chrono::Utc::now();
        contribution.updated_at = contribution.created_at;
        contribution.status = ContributionStatus::Submitted;
        
        // Update author statistics
        if let Some(author) = self.authors.get_mut(&contribution.author.username) {
            author.contribution_count += 1;
        } else {
            self.authors.insert(contribution.author.username.clone(), contribution.author.clone());
        }
        
        // Save to workspace
        self.save_contribution(&contribution)?;
        
        let id = contribution.id;
        self.contributions.insert(id, contribution);
        
        Ok(id)
    }
    
    pub fn add_review(&mut self, contribution_id: Uuid, review: CommunityReview) -> Result<(), CommunityError> {
        let contribution = self.contributions.get_mut(&contribution_id)
            .ok_or(CommunityError::ContributionNotFound(contribution_id))?;
        
        contribution.reviews.push(review);
        contribution.updated_at = chrono::Utc::now();
        
        // Update status based on reviews
        self.update_contribution_status(contribution_id)?;
        
        // Save updated contribution
        self.save_contribution(contribution)?;
        
        Ok(())
    }
    
    pub fn vote_on_contribution(&mut self, contribution_id: Uuid, is_upvote: bool) -> Result<(), CommunityError> {
        let contribution = self.contributions.get_mut(&contribution_id)
            .ok_or(CommunityError::ContributionNotFound(contribution_id))?;
        
        if is_upvote {
            contribution.votes.upvotes += 1;
        } else {
            contribution.votes.downvotes += 1;
        }
        
        contribution.updated_at = chrono::Utc::now();
        
        // Update author reputation
        if let Some(author) = self.authors.get_mut(&contribution.author.username) {
            if is_upvote {
                author.reputation += 5;
            } else if author.reputation >= 2 {
                author.reputation -= 2;
            }
        }
        
        self.save_contribution(contribution)?;
        
        Ok(())
    }
    
    pub fn get_contributions_by_type(&self, contribution_type: &ContributionType) -> Vec<&CommunityContribution> {
        self.contributions.values()
            .filter(|c| std::mem::discriminant(&c.contribution_type) == std::mem::discriminant(contribution_type))
            .collect()
    }
    
    pub fn get_top_contributors(&self, limit: usize) -> Vec<&ContributionAuthor> {
        let mut authors: Vec<_> = self.authors.values().collect();
        authors.sort_by(|a, b| b.reputation.cmp(&a.reputation));
        authors.into_iter().take(limit).collect()
    }
    
    pub fn generate_community_report(&self) -> CommunityReport {
        let total_contributions = self.contributions.len();
        let total_authors = self.authors.len();
        
        let mut contributions_by_type = HashMap::new();
        let mut contributions_by_status = HashMap::new();
        
        for contribution in self.contributions.values() {
            let type_count = contributions_by_type.entry(contribution.contribution_type.clone()).or_insert(0);
            *type_count += 1;
            
            let status_count = contributions_by_status.entry(contribution.status.clone()).or_insert(0);
            *status_count += 1;
        }
        
        let top_contributors = self.get_top_contributors(10)
            .into_iter()
            .map(|author| TopContributor {
                username: author.username.clone(),
                display_name: author.display_name.clone(),
                reputation: author.reputation,
                contribution_count: author.contribution_count,
            })
            .collect();
        
        let recent_contributions = {
            let mut recent: Vec<_> = self.contributions.values()
                .filter(|c| matches!(c.status, ContributionStatus::Published))
                .collect();
            recent.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            recent.into_iter()
                .take(20)
                .map(|c| RecentContribution {
                    id: c.id,
                    title: c.title.clone(),
                    author: c.author.display_name.clone(),
                    contribution_type: c.contribution_type.clone(),
                    created_at: c.created_at,
                    votes: c.votes.clone(),
                })
                .collect()
        };
        
        CommunityReport {
            total_contributions,
            total_authors,
            contributions_by_type,
            contributions_by_status,
            top_contributors,
            recent_contributions,
            generated_at: chrono::Utc::now(),
        }
    }
    
    fn load_contribution(&self, path: &std::path::Path) -> Result<CommunityContribution, CommunityError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| CommunityError::IoError(e.to_string()))?;
        
        serde_json::from_str(&content)
            .map_err(|e| CommunityError::ParseError(e.to_string()))
    }
    
    fn save_contribution(&self, contribution: &CommunityContribution) -> Result<(), CommunityError> {
        let contributions_dir = self.workspace.join("community/contributions");
        std::fs::create_dir_all(&contributions_dir)
            .map_err(|e| CommunityError::IoError(e.to_string()))?;
        
        let filename = format!("{}.json", contribution.id);
        let file_path = contributions_dir.join(filename);
        
        let content = serde_json::to_string_pretty(contribution)
            .map_err(|e| CommunityError::ParseError(e.to_string()))?;
        
        std::fs::write(&file_path, content)
            .map_err(|e| CommunityError::IoError(e.to_string()))?;
        
        Ok(())
    }
    
    fn update_contribution_status(&mut self, contribution_id: Uuid) -> Result<(), CommunityError> {
        let contribution = self.contributions.get_mut(&contribution_id)
            .ok_or(CommunityError::ContributionNotFound(contribution_id))?;
        
        if contribution.reviews.len() >= 3 {
            let excellent_count = contribution.reviews.iter()
                .filter(|r| matches!(r.rating, ReviewRating::Excellent))
                .count();
            let good_count = contribution.reviews.iter()
                .filter(|r| matches!(r.rating, ReviewRating::Good))
                .count();
            let poor_count = contribution.reviews.iter()
                .filter(|r| matches!(r.rating, ReviewRating::Poor))
                .count();
            
            contribution.status = if excellent_count >= 2 || (excellent_count + good_count) >= 3 {
                ContributionStatus::Approved
            } else if poor_count >= 2 {
                ContributionStatus::NeedsRevision
            } else {
                ContributionStatus::UnderReview
            };
        }
        
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommunityReport {
    pub total_contributions: usize,
    pub total_authors: usize,
    pub contributions_by_type: HashMap<ContributionType, u32>,
    pub contributions_by_status: HashMap<ContributionStatus, u32>,
    pub top_contributors: Vec<TopContributor>,
    pub recent_contributions: Vec<RecentContribution>,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopContributor {
    pub username: String,
    pub display_name: String,
    pub reputation: u32,
    pub contribution_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentContribution {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub contribution_type: ContributionType,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub votes: VoteCount,
}

#[derive(Debug)]
pub enum CommunityError {
    IoError(String),
    ParseError(String),
    ContributionNotFound(Uuid),
    InvalidContribution(String),
}

impl std::fmt::Display for CommunityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommunityError::IoError(msg) => write!(f, "IO error: {}", msg),
            CommunityError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CommunityError::ContributionNotFound(id) => write!(f, "Contribution not found: {}", id),
            CommunityError::InvalidContribution(msg) => write!(f, "Invalid contribution: {}", msg),
        }
    }
}

impl std::error::Error for CommunityError {}
```

## **Success Criteria**
- [ ] Comprehensive documentation covering all features and use cases
- [ ] Interactive examples that run successfully in documentation
- [ ] Multi-language support for global adoption
- [ ] Community contribution system with review process
- [ ] Search functionality across all documentation
- [ ] Mobile-responsive documentation website
- [ ] Integration with popular learning platforms
- [ ] Video content and tutorials
- [ ] Documentation analytics showing user engagement
- [ ] Regular content updates and maintenance workflow

## **Metrics to Track**
- Documentation page views and time spent
- Interactive example execution count and success rate
- Community contribution submission and approval rates
- Search query analysis and content gaps
- User feedback and satisfaction scores
- Integration guide usage and framework adoption

## **Future Enhancements**
- AI-powered documentation assistance and Q&A
- Real-time collaborative editing for community contributions
- Automated documentation generation from code
- Interactive tutorials with guided exercises
- Integration with popular code editors for inline help
- Multilingual documentation with community translations

This comprehensive documentation ecosystem transforms workspace_tools from a technical library into an accessible, community-driven standard that educates and empowers the entire Rust ecosystem.

<function_calls>
<invoke name="TodoWrite">
<parameter name="todos">[{"id": "t1", "content": "Create task 011_ide_integration.md", "status": "completed"}, {"id": "t2", "content": "Create task 012_cargo_team_integration.md", "status": "completed"}, {"id": "t3", "content": "Create task 013_workspace_scaffolding.md (enhanced template system)", "status": "completed"}, {"id": "t4", "content": "Create task 014_performance_optimization.md", "status": "completed"}, {"id": "t5", "content": "Create task 015_documentation_ecosystem.md", "status": "completed"}, {"id": "t6", "content": "Create task 016_community_building.md", "status": "in_progress"}]