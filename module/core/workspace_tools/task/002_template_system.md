# Task 002: Template System

**Priority**: 🏗️ High Impact  
**Phase**: 1 (Immediate)  
**Estimated Effort**: 4-5 days  
**Dependencies**: Task 001 (Cargo Integration) recommended  

## **Objective**
Implement a workspace scaffolding system that creates standard project structures, reducing time-to-productivity for new projects and establishing workspace_tools as a project creation tool.

## **Technical Requirements**

### **Core Features**
1. **Built-in Templates**
   - CLI application template
   - Web service template  
   - Library template
   - Desktop application template

2. **Template Engine**
   - Variable substitution (project name, author, etc.)
   - Conditional file generation
   - Directory structure creation
   - File content templating

3. **Extensibility**
   - Custom template support
   - Template validation
   - Template metadata

### **New API Surface**
```rust
impl Workspace 
{
    /// Create workspace structure from built-in template
    pub fn scaffold_from_template(&self, template: TemplateType) -> Result<()>;
    
    /// Create workspace structure from custom template
    pub fn scaffold_from_path<P: AsRef<Path>>(&self, template_path: P) -> Result<()>;
    
    /// List available built-in templates
    pub fn available_templates() -> Vec<TemplateInfo>;
    
    /// Validate template before scaffolding
    pub fn validate_template<P: AsRef<Path>>(&self, template_path: P) -> Result<TemplateValidation>;
}

#[derive(Debug, Clone)]
pub enum TemplateType 
{
    Cli,
    WebService,
    Library, 
    Desktop,
}

#[derive(Debug, Clone)]
pub struct TemplateInfo 
{
    pub name: String,
    pub description: String,
    pub files_created: usize,
    pub directories_created: usize,
}

#[derive(Debug, Clone)]
pub struct TemplateValidation 
{
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TemplateContext 
{
    pub project_name: String,
    pub author_name: String,
    pub author_email: String,
    pub license: String,
    pub variables: HashMap<String, String>,
}
```

### **Implementation Steps**

#### **Step 1: Template Engine Foundation** (Day 1)
```rust
// Add to Cargo.toml dependencies
[features]
default = ["enabled", "templates"]
templates = ["dep:handlebars", "dep:serde_json"]

[dependencies]
handlebars = { version = "4.0", optional = true }
serde_json = { version = "1.0", optional = true }

// Template engine implementation
#[cfg(feature = "templates")]
mod templating {
    use handlebars::Handlebars;
    use serde_json::{json, Value};
    use std::collections::HashMap;
    
    pub struct TemplateEngine 
{
        handlebars: Handlebars<'static>,
    }
    
    impl TemplateEngine 
{
        pub fn new() -> Self 
{
            let mut handlebars = Handlebars::new();
            handlebars.set_strict_mode(true);
            Self { handlebars }
        }
        
        pub fn render_string(&self, template: &str, context: &TemplateContext) -> Result<String> 
{
            let json_context = json!({
                "project_name": context.project_name,
                "author_name": context.author_name,
                "author_email": context.author_email,
                "license": context.license,
                "variables": context.variables,
            });
            
            self.handlebars.render_template(template, &json_context)
                .map_err(|e| WorkspaceError::ConfigurationError(e.to_string()))
        }
        
        pub fn render_file<P: AsRef<Path>>(
            &self, 
            template_path: P, 
            context: &TemplateContext
        ) -> Result<String> {
            let template_content = std::fs::read_to_string(template_path)
                .map_err(|e| WorkspaceError::IoError(e.to_string()))?;
            self.render_string(&template_content, context)
        }
    }
}
```

#### **Step 2: Built-in Templates** (Day 2)
```rust
// Embedded templates using include_str!
const CLI_TEMPLATE: &[(&str, &str)] = &[
    ("Cargo.toml", include_str!("../templates/cli/Cargo.toml.hbs")),
    ("src/main.rs", include_str!("../templates/cli/src/main.rs.hbs")),
    ("src/cli.rs", include_str!("../templates/cli/src/cli.rs.hbs")),
    ("config/app.toml", include_str!("../templates/cli/config/app.toml.hbs")),
    ("README.md", include_str!("../templates/cli/README.md.hbs")),
    (".gitignore", include_str!("../templates/cli/.gitignore")),
];

const WEB_SERVICE_TEMPLATE: &[(&str, &str)] = &[
    ("Cargo.toml", include_str!("../templates/web/Cargo.toml.hbs")),
    ("src/main.rs", include_str!("../templates/web/src/main.rs.hbs")),
    ("src/handlers.rs", include_str!("../templates/web/src/handlers.rs.hbs")),
    ("src/config.rs", include_str!("../templates/web/src/config.rs.hbs")),
    ("config/development.toml", include_str!("../templates/web/config/development.toml.hbs")),
    ("config/production.toml", include_str!("../templates/web/config/production.toml.hbs")),
    ("static/css/main.css", include_str!("../templates/web/static/css/main.css")),
    ("templates/base.html", include_str!("../templates/web/templates/base.html.hbs")),
    ("docker-compose.yml", include_str!("../templates/web/docker-compose.yml.hbs")),
    ("Dockerfile", include_str!("../templates/web/Dockerfile.hbs")),
];

impl TemplateType 
{
    fn template_files(&self) -> &'static [(&'static str, &'static str)] 
{
        match self {
            TemplateType::Cli => CLI_TEMPLATE,
            TemplateType::WebService => WEB_SERVICE_TEMPLATE,
            TemplateType::Library => LIBRARY_TEMPLATE,
            TemplateType::Desktop => DESKTOP_TEMPLATE,
        }
    }
    
    fn directories(&self) -> &'static [&'static str] 
{
        match self {
            TemplateType::Cli => &["src", "config", "data", "logs", "tests"],
            TemplateType::WebService => &[
                "src", "config", "data", "logs", "static/css", "static/js", 
                "templates", "uploads", "tests"
            ],
            TemplateType::Library => &["src", "examples", "tests", "benches"],
            TemplateType::Desktop => &[
                "src", "assets", "resources", "config", "data", "plugins"
            ],
        }
    }
}
```

#### **Step 3: Scaffolding Implementation** (Day 3)
```rust
#[cfg(feature = "templates")]
impl Workspace 
{
    pub fn scaffold_from_template(&self, template: TemplateType) -> Result<()> 
{
        // Create default context
        let context = self.create_default_context()?;
        self.scaffold_with_context(template, &context)
    }
    
    pub fn scaffold_with_context(
        &self, 
        template: TemplateType, 
        context: &TemplateContext
    ) -> Result<()> {
        let engine = TemplateEngine::new();
        
        // Create directories
        for dir in template.directories() {
            let dir_path = self.join(dir);
            std::fs::create_dir_all(&dir_path)
                .map_err(|e| WorkspaceError::IoError(e.to_string()))?;
        }
        
        // Create files from templates
        for (file_path, template_content) in template.template_files() {
            let rendered_content = engine.render_string(template_content, context)?;
            let full_path = self.join(file_path);
            
            // Ensure parent directory exists
            if let Some(parent) = full_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| WorkspaceError::IoError(e.to_string()))?;
            }
            
            std::fs::write(&full_path, rendered_content)
                .map_err(|e| WorkspaceError::IoError(e.to_string()))?;
        }
        
        Ok(())
    }
    
    fn create_default_context(&self) -> Result<TemplateContext> 
{
        Ok(TemplateContext {
            project_name: self.root()
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("my_project")
                .to_string(),
            author_name: std::env::var("USER")
                .or_else(|_| std::env::var("USERNAME"))
                .unwrap_or_else(|_| "Author".to_string()),
            author_email: format!("{}@example.com", 
                std::env::var("USER").unwrap_or_else(|_| "author".to_string())
            ),
            license: "MIT".to_string(),
            variables: HashMap::new(),
        })
    }
}
```

#### **Step 4: Template Files Creation** (Day 4)
Create actual template files in `templates/` directory:

**templates/cli/Cargo.toml.hbs**:
```toml
[package]
name = "{{project_name}}"
version = "0.1.0"
edition = "2021"
authors = ["{{author_name}} <{{author_email}}>"]
license = "{{license}}"
description = "A CLI application built with workspace_tools"

[dependencies]
workspace_tools = "0.2"
clap = { version = "4.0", features = ["derive"] }
anyhow = "1.0"
```

**templates/cli/src/main.rs.hbs**:
```rust
//! {{project_name}} - CLI application

use workspace_tools::workspace;
use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "{{project_name}}")]
#[command(about = "A CLI application with workspace_tools")]
struct Cli 
{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands 
{
    /// Initialize the application
    Init,
    /// Show configuration information
    Info,
}

fn main() -> Result<()> 
{
    let cli = Cli::parse();
    let ws = workspace()?;
    
    match cli.command {
        Commands::Init => {
            println!("Initializing {{project_name}}...");
            // Create necessary directories
            std::fs::create_dir_all(ws.config_dir())?;
            std::fs::create_dir_all(ws.data_dir())?;
            std::fs::create_dir_all(ws.logs_dir())?;
            println!("✅ Initialization complete!");
        }
        Commands::Info => {
            println!("{{project_name}} Information:");
            println!("Workspace root: {}", ws.root().display());
            println!("Config dir: {}", ws.config_dir().display());
            println!("Data dir: {}", ws.data_dir().display());
        }
    }
    
    Ok(())
}
```

**templates/web/src/main.rs.hbs**:
```rust
//! {{project_name}} - Web service

use workspace_tools::workspace;
use std::net::SocketAddr;

mod handlers;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    let ws = workspace()?;
    let config = config::load_config(&ws).await?;
    
    println!("🚀 Starting {{project_name}}");
    println!("Workspace: {}", ws.root().display());
    
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    println!("🌐 Server running on http://{}", addr);
    
    // Your web framework setup here
    // axum::Server::bind(&addr)...
    
    Ok(())
}
```

#### **Step 5: Testing & Documentation** (Day 5)
```rust
#[cfg(test)]
#[cfg(feature = "templates")]
mod template_tests {
    use super::*;
    use crate::testing::create_test_workspace;
    
    #[test]
    fn test_cli_template_scaffolding() 
{
        let (_temp_dir, ws) = create_test_workspace();
        
        ws.scaffold_from_template(TemplateType::Cli).unwrap();
        
        // Verify files were created
        assert!(ws.join("Cargo.toml").exists());
        assert!(ws.join("src/main.rs").exists());
        assert!(ws.join("src/cli.rs").exists());
        assert!(ws.config_dir().join("app.toml").exists());
        
        // Verify content was templated
        let cargo_toml = std::fs::read_to_string(ws.join("Cargo.toml")).unwrap();
        assert!(cargo_toml.contains("workspace_tools"));
        assert!(!cargo_toml.contains("{{project_name}}"));
    }
    
    #[test]
    fn test_web_service_template_scaffolding() 
{
        let (_temp_dir, ws) = create_test_workspace();
        
        ws.scaffold_from_template(TemplateType::WebService).unwrap();
        
        // Verify web-specific structure
        assert!(ws.join("static/css").exists());
        assert!(ws.join("templates").exists());
        assert!(ws.join("docker-compose.yml").exists());
    }
    
    #[test]
    fn test_custom_template_context() 
{
        let (_temp_dir, ws) = create_test_workspace();
        
        let mut context = TemplateContext {
            project_name: "my_awesome_cli".to_string(),
            author_name: "Test Author".to_string(),
            author_email: "test@example.com".to_string(),
            license: "Apache-2.0".to_string(),
            variables: HashMap::new(),
        };
        
        ws.scaffold_with_context(TemplateType::Cli, &context).unwrap();
        
        let cargo_toml = std::fs::read_to_string(ws.join("Cargo.toml")).unwrap();
        assert!(cargo_toml.contains("my_awesome_cli"));
        assert!(cargo_toml.contains("Test Author"));
        assert!(cargo_toml.contains("Apache-2.0"));
    }
}
```

### **CLI Integration**
```rust
// Future: CLI command for scaffolding
// cargo workspace-tools init --template=web-service
// cargo workspace-tools scaffold --template=cli MyApp
```

### **Documentation Updates**

#### **README.md Addition**
```markdown
## 🏗️ project scaffolding

workspace_tools includes project templates for common Rust project types:

```rust
use workspace_tools::{workspace, TemplateType};

let ws = workspace()?;

// Create a CLI application structure
ws.scaffold_from_template(TemplateType::Cli)?;

// Create a web service structure  
ws.scaffold_from_template(TemplateType::WebService)?;
```

### Available templates:
- **CLI**: Command-line applications with argument parsing
- **Web Service**: Web applications with static assets and templates  
- **Library**: Rust libraries with examples and benchmarks
- **Desktop**: GUI applications with assets and resources
```

#### **New Example: templates.rs**
```rust
//! Project scaffolding example

use workspace_tools::{workspace, TemplateType, TemplateContext};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    let ws = workspace()?;
    
    println!("🏗️  Project Scaffolding Demo");
    println!("Available templates:");
    
    for template in Workspace::available_templates() {
        println!("  📋 {}: {}", template.name, template.description);
        println!("     Creates {} files, {} directories", 
                 template.files_created, template.directories_created);
    }
    
    // Scaffold with custom context
    let mut custom_vars = HashMap::new();
    custom_vars.insert("database".to_string(), "postgresql".to_string());
    
    let context = TemplateContext {
        project_name: "my_web_app".to_string(),
        author_name: "Developer".to_string(),
        author_email: "dev@example.com".to_string(),  
        license: "MIT".to_string(),
        variables: custom_vars,
    };
    
    println!("\n🔨 Scaffolding web service template...");
    ws.scaffold_with_context(TemplateType::WebService, &context)?;
    println!("✅ Project structure created!");
    
    Ok(())
}
```

### **Success Criteria**
- [ ] Four built-in templates (CLI, Web, Library, Desktop)
- [ ] Template engine with variable substitution
- [ ] Custom context support for personalization
- [ ] Comprehensive test coverage for all templates
- [ ] Generated projects compile and run successfully  
- [ ] Documentation with examples
- [ ] Performance: Scaffolding completes in <1 second

### **Future Enhancements**
- External template repository support
- Interactive template selection
- Template validation and linting
- Integration with cargo-generate
- Custom template creation tools

### **Breaking Changes**
None - this is purely additive functionality with a feature flag.

This task establishes workspace_tools as not just a path resolution library, but a comprehensive project creation and management tool.