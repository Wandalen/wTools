# Task 013: Advanced Workspace Scaffolding

**Priority**: 🏗️ High Impact  
**Phase**: 1-2 (Enhanced Template System)  
**Estimated Effort**: 4-6 weeks  
**Dependencies**: Task 002 (Template System), Task 001 (Cargo Integration)  

## **Objective**
Extend the basic template system into a comprehensive workspace scaffolding solution that can generate complete, production-ready project structures with best practices built-in, making workspace_tools the go-to choice for new Rust project creation.

## **Technical Requirements**

### **Advanced Template Features**
1. **Hierarchical Template System**
   - Base templates with inheritance and composition
   - Plugin-based extensions for specialized use cases
   - Custom template repositories and sharing

2. **Interactive Scaffolding**
   - Wizard-style project creation with questionnaires
   - Conditional file generation based on user choices
   - Real-time preview of generated structure

3. **Best Practices Integration**
   - Security-focused configurations by default
   - Performance optimization patterns
   - Testing infrastructure setup
   - CI/CD pipeline generation

4. **Framework Integration**
   - Deep integration with popular Rust frameworks
   - Framework-specific optimizations and configurations
   - Plugin ecosystem for community extensions

### **New API Surface**
```rust
impl Workspace {
    /// Advanced scaffolding with interactive wizard
    pub fn scaffold_interactive(&self, template_name: &str) -> Result<ScaffoldingWizard>;
    
    /// Generate from template with parameters
    pub fn scaffold_from_template_with_params(
        &self,
        template: &str,
        params: ScaffoldingParams
    ) -> Result<GeneratedProject>;
    
    /// List available templates with metadata
    pub fn list_available_templates(&self) -> Result<Vec<TemplateInfo>>;
    
    /// Install template from repository
    pub fn install_template_from_repo(&self, repo_url: &str, name: &str) -> Result<()>;
    
    /// Validate existing project against template
    pub fn validate_against_template(&self, template_name: &str) -> Result<ValidationReport>;
    
    /// Update project structure to match template evolution
    pub fn update_from_template(&self, template_name: &str) -> Result<UpdateReport>;
}

/// Interactive scaffolding wizard
pub struct ScaffoldingWizard {
    template: Template,
    responses: HashMap<String, Value>,
    workspace: Workspace,
}

impl ScaffoldingWizard {
    pub fn ask_question(&mut self, question_id: &str) -> Result<Question>;
    pub fn answer_question(&mut self, question_id: &str, answer: Value) -> Result<()>;
    pub fn preview_structure(&self) -> Result<ProjectStructure>;
    pub fn generate(&self) -> Result<GeneratedProject>;
}

/// Advanced template definition
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Template {
    pub metadata: TemplateMetadata,
    pub inheritance: Option<String>,
    pub questions: Vec<Question>,
    pub files: Vec<TemplateFile>,
    pub dependencies: Vec<TemplateDependency>,
    pub post_generation: Vec<PostGenerationAction>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct TemplateMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub tags: Vec<String>,
    pub rust_version: String,
    pub frameworks: Vec<String>,
    pub complexity: TemplateComplexity,
    pub maturity: TemplateMaturity,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum TemplateComplexity {
    Beginner,
    Intermediate, 
    Advanced,
    Expert,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum TemplateMaturity {
    Experimental,
    Beta,
    Stable,
    Production,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Question {
    pub id: String,
    pub prompt: String,
    pub question_type: QuestionType,
    pub default: Option<Value>,
    pub validation: Option<ValidationRule>,
    pub conditions: Vec<ConditionalRule>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum QuestionType {
    Text { placeholder: Option<String> },
    Choice { options: Vec<String>, multiple: bool },
    Boolean { default: bool },
    Number { min: Option<i32>, max: Option<i32> },
    Path { must_exist: bool, is_directory: bool },
    Email,
    Url,
    SemVer,
}
```

## **Implementation Steps**

### **Phase 1: Advanced Template Engine** (Weeks 1-2)

#### **Week 1: Template Inheritance System**
```rust
// Template inheritance and composition
#[derive(Debug, Clone)]
pub struct TemplateEngine {
    template_registry: TemplateRegistry,
    template_cache: HashMap<String, CompiledTemplate>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        Self {
            template_registry: TemplateRegistry::new(),
            template_cache: HashMap::new(),
        }
    }
    
    pub fn compile_template(&mut self, template_name: &str) -> Result<CompiledTemplate> {
        if let Some(cached) = self.template_cache.get(template_name) {
            return Ok(cached.clone());
        }
        
        let template = self.template_registry.load_template(template_name)?;
        let compiled = self.resolve_inheritance(template)?;
        
        self.template_cache.insert(template_name.to_string(), compiled.clone());
        Ok(compiled)
    }
    
    fn resolve_inheritance(&self, template: Template) -> Result<CompiledTemplate> {
        let mut resolved_files = Vec::new();
        let mut resolved_dependencies = Vec::new();
        let mut resolved_questions = Vec::new();
        
        // Handle inheritance chain
        if let Some(parent_name) = &template.inheritance {
            let parent = self.template_registry.load_template(parent_name)?;
            let parent_compiled = self.resolve_inheritance(parent)?;
            
            // Inherit and merge
            resolved_files.extend(parent_compiled.files);
            resolved_dependencies.extend(parent_compiled.dependencies);
            resolved_questions.extend(parent_compiled.questions);
        }
        
        // Add/override with current template
        resolved_files.extend(template.files);
        resolved_dependencies.extend(template.dependencies);
        resolved_questions.extend(template.questions);
        
        Ok(CompiledTemplate {
            metadata: template.metadata,
            files: resolved_files,
            dependencies: resolved_dependencies,
            questions: resolved_questions,
            post_generation: template.post_generation,
        })
    }
}

// Template file with advanced features
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct TemplateFile {
    pub path: String,
    pub content: TemplateContent,
    pub conditions: Vec<ConditionalRule>,
    pub permissions: Option<u32>,
    pub binary: bool,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum TemplateContent {
    Inline(String),
    FromFile(String),
    Generated { generator: String, params: HashMap<String, Value> },
    Composite(Vec<TemplateContent>),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ConditionalRule {
    pub condition: String,  // JavaScript-like expression
    pub operator: ConditionalOperator,
    pub value: Value,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum ConditionalOperator {
    Equals,
    NotEquals,
    Contains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    And(Vec<ConditionalRule>),
    Or(Vec<ConditionalRule>),
}
```

#### **Week 2: Interactive Wizard System**
```rust
// Interactive scaffolding wizard implementation
use std::io::{self, Write};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{self, Color, Stylize},
    terminal::{self, ClearType},
};

pub struct ScaffoldingWizard {
    template: CompiledTemplate,
    responses: HashMap<String, Value>,
    current_question: usize,
    workspace: Workspace,
}

impl ScaffoldingWizard {
    pub fn new(template: CompiledTemplate, workspace: Workspace) -> Self {
        Self {
            template,
            responses: HashMap::new(),
            current_question: 0,
            workspace,
        }
    }
    
    pub async fn run_interactive(&mut self) -> Result<GeneratedProject> {
        println!("{}", "🚀 Workspace Scaffolding Wizard".bold().cyan());
        println!("{}", format!("Template: {}", self.template.metadata.name).dim());
        println!("{}", format!("Description: {}", self.template.metadata.description).dim());
        println!();
        
        // Run through all questions
        for (index, question) in self.template.questions.iter().enumerate() {
            self.current_question = index;
            
            if self.should_ask_question(question)? {
                let answer = self.ask_question_interactive(question).await?;
                self.responses.insert(question.id.clone(), answer);
            }
        }
        
        // Show preview
        self.show_preview()?;
        
        // Confirm generation
        if self.confirm_generation().await? {
            self.generate_project()
        } else {
            Err(WorkspaceError::ConfigurationError("Generation cancelled".to_string()))
        }
    }
    
    async fn ask_question_interactive(&self, question: &Question) -> Result<Value> {
        loop {
            // Clear screen and show progress
            execute!(io::stdout(), terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;
            
            self.show_progress_header()?;
            self.show_question(question)?;
            
            let answer = match &question.question_type {
                QuestionType::Text { placeholder } => {
                    self.get_text_input(placeholder.as_deref()).await?
                },
                QuestionType::Choice { options, multiple } => {
                    self.get_choice_input(options, *multiple).await?
                },
                QuestionType::Boolean { default } => {
                    self.get_boolean_input(*default).await?
                },
                QuestionType::Number { min, max } => {
                    self.get_number_input(*min, *max).await?
                },
                QuestionType::Path { must_exist, is_directory } => {
                    self.get_path_input(*must_exist, *is_directory).await?
                },
                QuestionType::Email => {
                    self.get_email_input().await?
                },
                QuestionType::Url => {
                    self.get_url_input().await?
                },
                QuestionType::SemVer => {
                    self.get_semver_input().await?
                },
            };
            
            // Validate answer
            if let Some(validation) = &question.validation {
                if let Err(error) = self.validate_answer(&answer, validation) {
                    println!("{} {}", "❌".red(), error.to_string().red());
                    println!("Press any key to try again...");
                    self.wait_for_key().await?;
                    continue;
                }
            }
            
            return Ok(answer);
        }
    }
    
    fn show_progress_header(&self) -> Result<()> {
        let total = self.template.questions.len();
        let current = self.current_question + 1;
        let progress = (current as f32 / total as f32 * 100.0) as usize;
        
        println!("{}", "🏗️  Workspace Scaffolding".bold().cyan());
        println!("{}", format!("Template: {}", self.template.metadata.name).dim());
        println!();
        
        // Progress bar
        let bar_width = 50;
        let filled = (progress * bar_width / 100).min(bar_width);
        let empty = bar_width - filled;
        
        print!("Progress: [");
        print!("{}", "█".repeat(filled).green());
        print!("{}", "░".repeat(empty).dim());
        println!("] {}/{}  ({}%)", current, total, progress);
        println!();
        
        Ok(())
    }
    
    fn show_question(&self, question: &Question) -> Result<()> {
        println!("{} {}", "?".bold().blue(), question.prompt.bold());
        
        if let Some(default) = &question.default {
            println!("  {} {}", "Default:".dim(), format!("{}", default).dim());
        }
        
        println!();
        Ok(())
    }
    
    async fn get_choice_input(&self, options: &[String], multiple: bool) -> Result<Value> {
        let mut selected = vec![false; options.len()];
        let mut current = 0;
        
        loop {
            // Clear and redraw options
            execute!(io::stdout(), cursor::MoveUp(options.len() as u16 + 2))?;
            execute!(io::stdout(), terminal::Clear(ClearType::FromCursorDown))?;
            
            for (i, option) in options.iter().enumerate() {
                let marker = if i == current { ">" } else { " " };
                let checkbox = if selected[i] { "☑" } else { "☐" };
                let style = if i == current { 
                    format!("{} {} {}", marker.cyan(), checkbox, option).bold() 
                } else {
                    format!("{} {} {}", marker, checkbox, option)
                };
                println!("  {}", style);
            }
            
            println!();
            if multiple {
                println!("  {} Use ↑↓ to navigate, SPACE to select, ENTER to confirm", "💡".dim());
            } else {
                println!("  {} Use ↑↓ to navigate, ENTER to select", "💡".dim());
            }
            
            // Handle input
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Up => {
                        current = if current > 0 { current - 1 } else { options.len() - 1 };
                    }
                    KeyCode::Down => {
                        current = (current + 1) % options.len();
                    }
                    KeyCode::Char(' ') if multiple => {
                        selected[current] = !selected[current];
                    }
                    KeyCode::Enter => {
                        if multiple {
                            let choices: Vec<String> = options.iter()
                                .enumerate()
                                .filter(|(i, _)| selected[*i])
                                .map(|(_, option)| option.clone())
                                .collect();
                            return Ok(Value::Array(choices.into_iter().map(Value::String).collect()));
                        } else {
                            return Ok(Value::String(options[current].clone()));
                        }
                    }
                    KeyCode::Esc => {
                        return Err(WorkspaceError::ConfigurationError("Cancelled".to_string()));
                    }
                    _ => {}
                }
            }
        }
    }
    
    fn show_preview(&self) -> Result<()> {
        println!();
        println!("{}", "📋 Project Structure Preview".bold().yellow());
        println!("{}", "═".repeat(50).dim());
        
        let structure = self.preview_structure()?;
        self.print_structure(&structure, 0)?;
        
        println!();
        Ok(())
    }
    
    fn preview_structure(&self) -> Result<ProjectStructure> {
        let mut structure = ProjectStructure::new();
        
        for template_file in &self.template.files {
            if self.should_generate_file(template_file)? {
                let resolved_path = self.resolve_template_string(&template_file.path)?;
                structure.add_file(resolved_path);
            }
        }
        
        Ok(structure)
    }
    
    fn print_structure(&self, structure: &ProjectStructure, indent: usize) -> Result<()> {
        let indent_str = "  ".repeat(indent);
        
        for item in &structure.items {
            match item {
                StructureItem::Directory { name, children } => {
                    println!("{}📁 {}/", indent_str, name.blue());
                    for child in children {
                        self.print_structure_item(child, indent + 1)?;
                    }
                }
                StructureItem::File { name, size } => {
                    let size_str = if let Some(s) = size {
                        format!(" ({} bytes)", s).dim()
                    } else {
                        String::new()
                    };
                    println!("{}📄 {}{}", indent_str, name, size_str);
                }
            }
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ProjectStructure {
    items: Vec<StructureItem>,
}

impl ProjectStructure {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
    
    fn add_file(&mut self, path: String) {
        // Implementation for building nested structure
        // This would parse the path and create the directory hierarchy
    }
}

#[derive(Debug, Clone)]
enum StructureItem {
    Directory { 
        name: String, 
        children: Vec<StructureItem> 
    },
    File { 
        name: String, 
        size: Option<usize> 
    },
}
```

### **Phase 2: Production-Ready Templates** (Weeks 3-4)

#### **Week 3: Framework-Specific Templates**
```toml
# templates/web-service-axum/template.toml
[metadata]
name = "web-service-axum"
version = "1.0.0"
description = "Production-ready web service using Axum framework"
author = "workspace_tools"
tags = ["web", "api", "axum", "production"]
rust_version = "1.70.0"
frameworks = ["axum", "tower", "tokio"]
complexity = "Intermediate"
maturity = "Production"

[inheritance]
base = "rust-base"

[[questions]]
id = "service_name"
prompt = "What's the name of your web service?"
type = { Text = { placeholder = "my-api-service" } }
validation = { regex = "^[a-z][a-z0-9-]+$" }

[[questions]]
id = "api_version"
prompt = "API version?"
type = { Text = { placeholder = "v1" } }
default = "v1"

[[questions]]
id = "database"
prompt = "Which database do you want to use?"
type = { Choice = { options = ["PostgreSQL", "MySQL", "SQLite", "None"], multiple = false } }
default = "PostgreSQL"

[[questions]]
id = "authentication"
prompt = "Do you need authentication?"
type = { Boolean = { default = true } }

[[questions]]
id = "openapi"
prompt = "Generate OpenAPI documentation?"
type = { Boolean = { default = true } }

[[questions]]
id = "docker"
prompt = "Include Docker configuration?"
type = { Boolean = { default = true } }

[[questions]]
id = "ci_cd"
prompt = "Which CI/CD platform?"
type = { Choice = { options = ["GitHub Actions", "GitLab CI", "None"], multiple = false } }
default = "GitHub Actions"

# Conditional file generation
[[files]]
path = "src/main.rs"
content = { FromFile = "templates/main.rs" }

[[files]]
path = "src/routes/mod.rs"
content = { FromFile = "templates/routes/mod.rs" }

[[files]]
path = "src/routes/{{api_version}}/mod.rs"
content = { FromFile = "templates/routes/versioned.rs" }

[[files]]
path = "src/models/mod.rs" 
content = { FromFile = "templates/models/mod.rs" }
conditions = [
    { condition = "database", operator = "NotEquals", value = "None" }
]

[[files]]
path = "src/auth/mod.rs"
content = { FromFile = "templates/auth/mod.rs" }
conditions = [
    { condition = "authentication", operator = "Equals", value = true }
]

[[files]]
path = "migrations/001_initial.sql"
content = { Generated = { generator = "database_migration", params = { database = "{{database}}" } } }
conditions = [
    { condition = "database", operator = "NotEquals", value = "None" }
]

[[files]]
path = "Dockerfile"
content = { FromFile = "templates/docker/Dockerfile" }
conditions = [
    { condition = "docker", operator = "Equals", value = true }
]

[[files]]
path = ".github/workflows/ci.yml"
content = { FromFile = "templates/github-actions/ci.yml" }
conditions = [
    { condition = "ci_cd", operator = "Equals", value = "GitHub Actions" }
]

# Dependencies configuration
[[dependencies]]
crate = "axum"
version = "0.7"
features = ["macros"]

[[dependencies]]
crate = "tokio"
version = "1.0"
features = ["full"]

[[dependencies]]
crate = "tower"
version = "0.4"

[[dependencies]]
crate = "sqlx"
version = "0.7"
features = ["runtime-tokio-rustls", "{{database | lower}}"]
conditions = [
    { condition = "database", operator = "NotEquals", value = "None" }
]

[[dependencies]]
crate = "jsonwebtoken"
version = "9.0"
conditions = [
    { condition = "authentication", operator = "Equals", value = true }
]

[[dependencies]]
crate = "utoipa"
version = "4.0"
features = ["axum_extras"]
conditions = [
    { condition = "openapi", operator = "Equals", value = true }
]

# Post-generation actions
[[post_generation]]
action = "RunCommand"
command = "cargo fmt"
description = "Format generated code"

[[post_generation]]
action = "RunCommand" 
command = "cargo clippy -- -D warnings"
description = "Check code quality"

[[post_generation]]
action = "CreateGitRepo"
description = "Initialize git repository"

[[post_generation]]
action = "ShowMessage"
message = """
🎉 Web service scaffolding complete!

Next steps:
1. Review the generated configuration files
2. Update database connection settings in config/
3. Run `cargo run` to start the development server
4. Check the API documentation at http://localhost:3000/swagger-ui/

Happy coding! 🦀
"""
```

#### **Week 4: Advanced Code Generators**
```rust
// Code generation system
pub trait CodeGenerator {
    fn generate(&self, params: &HashMap<String, Value>) -> Result<String>;
    fn name(&self) -> &str;
}

pub struct DatabaseMigrationGenerator;

impl CodeGenerator for DatabaseMigrationGenerator {
    fn generate(&self, params: &HashMap<String, Value>) -> Result<String> {
        let database = params.get("database")
            .and_then(|v| v.as_str())
            .ok_or_else(|| WorkspaceError::ConfigurationError("Missing database parameter".to_string()))?;
            
        match database {
            "PostgreSQL" => Ok(self.generate_postgresql_migration()),
            "MySQL" => Ok(self.generate_mysql_migration()),
            "SQLite" => Ok(self.generate_sqlite_migration()),
            _ => Err(WorkspaceError::ConfigurationError(format!("Unsupported database: {}", database)))
        }
    }
    
    fn name(&self) -> &str {
        "database_migration"
    }
}

impl DatabaseMigrationGenerator {
    fn generate_postgresql_migration(&self) -> String {
        r#"-- Initial database schema for PostgreSQL

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);

-- Add triggers for updated_at
CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at 
    BEFORE UPDATE ON users 
    FOR EACH ROW 
    EXECUTE FUNCTION update_modified_column();
"#.to_string()
    }
    
    fn generate_mysql_migration(&self) -> String {
        r#"-- Initial database schema for MySQL

CREATE TABLE users (
    id CHAR(36) PRIMARY KEY DEFAULT (UUID()),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_email ON users(email);
"#.to_string()
    }
    
    fn generate_sqlite_migration(&self) -> String {
        r#"-- Initial database schema for SQLite

CREATE TABLE users (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_email ON users(email);

-- Trigger for updated_at
CREATE TRIGGER update_users_updated_at 
    AFTER UPDATE ON users
    FOR EACH ROW
    BEGIN
        UPDATE users SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
    END;
"#.to_string()
    }
}

pub struct RestApiGenerator;

impl CodeGenerator for RestApiGenerator {
    fn generate(&self, params: &HashMap<String, Value>) -> Result<String> {
        let resource = params.get("resource")
            .and_then(|v| v.as_str())
            .ok_or_else(|| WorkspaceError::ConfigurationError("Missing resource parameter".to_string()))?;
        
        let has_auth = params.get("authentication")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
            
        self.generate_rest_routes(resource, has_auth)
    }
    
    fn name(&self) -> &str {
        "rest_api"
    }
}

impl RestApiGenerator {
    fn generate_rest_routes(&self, resource: &str, has_auth: bool) -> Result<String> {
        let auth_middleware = if has_auth {
            "use crate::auth::require_auth;\n"
        } else {
            ""
        };
        
        let auth_layer = if has_auth {
            ".route_layer(middleware::from_fn(require_auth))"
        } else {
            ""
        };
        
        Ok(format!(r#"use axum::{{
    extract::{{Path, Query, State}},
    http::StatusCode,
    response::Json,
    routing::{{get, post, put, delete}},
    Router,
    middleware,
}};
use serde::{{Deserialize, Serialize}};
use uuid::Uuid;
{}
use crate::models::{};
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Create{}Request {{
    // Add fields here
    pub name: String,
}}

#[derive(Debug, Serialize, Deserialize)]
pub struct Update{}Request {{
    // Add fields here  
    pub name: Option<String>,
}}

#[derive(Debug, Deserialize)]
pub struct {}Query {{
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub search: Option<String>,
}}

pub fn routes() -> Router<AppState> {{
    Router::new()
        .route("/{}", get(list_{}))
        .route("/{}", post(create_{}))
        .route("/{}/:id", get(get_{}))
        .route("/{}/:id", put(update_{}))
        .route("/{}/:id", delete(delete_{}))
        {}
}}

async fn list_{}(
    Query(query): Query<{}Query>,
    State(state): State<AppState>,
) -> Result<Json<Vec<{}>>, StatusCode> {{
    // TODO: Implement listing with pagination and search
    todo!("Implement {} listing")
}}

async fn create_{}(
    State(state): State<AppState>,
    Json(request): Json<Create{}Request>,
) -> Result<Json<{}>, StatusCode> {{
    // TODO: Implement creation
    todo!("Implement {} creation")
}}

async fn get_{}(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<{}>, StatusCode> {{
    // TODO: Implement getting by ID
    todo!("Implement {} retrieval")
}}

async fn update_{}(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(request): Json<Update{}Request>,
) -> Result<Json<{}>, StatusCode> {{
    // TODO: Implement updating
    todo!("Implement {} updating")
}}

async fn delete_{}(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<StatusCode, StatusCode> {{
    // TODO: Implement deletion
    todo!("Implement {} deletion")
}}
"#,
            auth_middleware,
            resource,
            resource,
            resource,
            resource,
            resource, resource,
            resource, resource,
            resource, resource,
            resource, resource,
            resource, resource,
            auth_layer,
            resource,
            resource,
            resource,
            resource,
            resource,
            resource,
            resource,
            resource,
            resource,
            resource,
            resource,
            resource,
            resource,
            resource,
            resource,
            resource,
        ))
    }
}
```

### **Phase 3: Template Repository System** (Weeks 5-6)

#### **Week 5: Template Distribution**
```rust
// Template repository management
pub struct TemplateRepository {
    url: String,
    cache_dir: PathBuf,
    metadata: RepositoryMetadata,
}

impl TemplateRepository {
    pub fn new(url: String, cache_dir: PathBuf) -> Self {
        Self {
            url,
            cache_dir,
            metadata: RepositoryMetadata::default(),
        }
    }
    
    pub async fn sync(&mut self) -> Result<()> {
        // Download repository metadata
        let metadata_url = format!("{}/index.json", self.url);
        let response = reqwest::get(&metadata_url).await
            .map_err(|e| WorkspaceError::IoError(e.to_string()))?;
            
        self.metadata = response.json().await
            .map_err(|e| WorkspaceError::ConfigurationError(e.to_string()))?;
            
        // Download templates that have been updated
        for template_info in &self.metadata.templates {
            let local_path = self.cache_dir.join(&template_info.name);
            
            if !local_path.exists() || template_info.version != self.get_cached_version(&template_info.name)? {
                self.download_template(template_info).await?;
            }
        }
        
        Ok(())
    }
    
    pub async fn install_template(&self, name: &str) -> Result<PathBuf> {
        let template_info = self.metadata.templates.iter()
            .find(|t| t.name == name)
            .ok_or_else(|| WorkspaceError::PathNotFound(PathBuf::from(name)))?;
            
        let template_dir = self.cache_dir.join(name);
        
        if !template_dir.exists() {
            self.download_template(template_info).await?;
        }
        
        Ok(template_dir)
    }
    
    async fn download_template(&self, template_info: &TemplateInfo) -> Result<()> {
        let template_url = format!("{}/templates/{}.tar.gz", self.url, template_info.name);
        let response = reqwest::get(&template_url).await
            .map_err(|e| WorkspaceError::IoError(e.to_string()))?;
            
        let bytes = response.bytes().await
            .map_err(|e| WorkspaceError::IoError(e.to_string()))?;
            
        // Extract tar.gz
        let template_dir = self.cache_dir.join(&template_info.name);
        std::fs::create_dir_all(&template_dir)
            .map_err(|e| WorkspaceError::IoError(e.to_string()))?;
            
        // TODO: Extract tar.gz to template_dir
        self.extract_template(&bytes, &template_dir)?;
        
        Ok(())
    }
    
    fn extract_template(&self, bytes: &[u8], dest: &Path) -> Result<()> {
        // Implementation for extracting tar.gz archive
        // This would use a crate like flate2 + tar
        todo!("Implement tar.gz extraction")
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct RepositoryMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub templates: Vec<TemplateInfo>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl Default for RepositoryMetadata {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: String::new(),
            description: String::new(),
            templates: Vec::new(),
            last_updated: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct TemplateInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub tags: Vec<String>,
    pub complexity: TemplateComplexity,
    pub maturity: TemplateMaturity,
    pub download_count: u64,
    pub rating: f32,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}
```

#### **Week 6: CLI Integration and Testing**
```rust
// CLI commands for advanced scaffolding
impl WorkspaceToolsCli {
    pub async fn scaffold_interactive(&self, template_name: Option<String>) -> Result<()> {
        let workspace = workspace()?;
        
        let template_name = match template_name {
            Some(name) => name,
            None => self.select_template_interactive().await?,
        };
        
        let template_engine = TemplateEngine::new();
        let compiled_template = template_engine.compile_template(&template_name)?;
        
        let mut wizard = ScaffoldingWizard::new(compiled_template, workspace);
        let generated_project = wizard.run_interactive().await?;
        
        println!("🎉 Project scaffolding complete!");
        println!("Generated {} files in {}", 
                generated_project.files_created.len(),
                generated_project.root_path.display());
        
        Ok(())
    }
    
    async fn select_template_interactive(&self) -> Result<String> {
        let template_registry = TemplateRegistry::new();
        let templates = template_registry.list_templates()?;
        
        if templates.is_empty() {
            return Err(WorkspaceError::ConfigurationError(
                "No templates available. Try running 'workspace-tools template install-repo https://github.com/workspace-tools/templates'"
                    .to_string()
            ));
        }
        
        println!("📚 Available Templates:");
        println!();
        
        for (i, template) in templates.iter().enumerate() {
            let complexity_color = match template.complexity {
                TemplateComplexity::Beginner => "green",
                TemplateComplexity::Intermediate => "yellow", 
                TemplateComplexity::Advanced => "orange",
                TemplateComplexity::Expert => "red",
            };
            
            println!("{}. {} {} {}", 
                     i + 1, 
                     template.name.bold(),
                     format!("({})", template.complexity).color(complexity_color),
                     template.description.dim());
            
            if !template.tags.is_empty() {
                println!("   Tags: {}", template.tags.join(", ").dim());
            }
            println!();
        }
        
        print!("Select template (1-{}): ", templates.len());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let selection: usize = input.trim().parse()
            .map_err(|_| WorkspaceError::ConfigurationError("Invalid selection".to_string()))?;
            
        if selection == 0 || selection > templates.len() {
            return Err(WorkspaceError::ConfigurationError("Selection out of range".to_string()));
        }
        
        Ok(templates[selection - 1].name.clone())
    }
    
    pub async fn template_install_repo(&self, repo_url: &str, name: Option<String>) -> Result<()> {
        let repo_name = name.unwrap_or_else(|| {
            repo_url.split('/').last().unwrap_or("unknown").to_string()
        });
        
        let template_registry = TemplateRegistry::new();
        let mut repo = TemplateRepository::new(repo_url.to_string(), template_registry.cache_dir());
        
        println!("📦 Installing template repository: {}", repo_url);
        repo.sync().await?;
        
        template_registry.add_repository(repo_name, repo)?;
        
        println!("✅ Template repository installed successfully");
        Ok(())
    }
    
    pub fn template_list(&self) -> Result<()> {
        let template_registry = TemplateRegistry::new();
        let templates = template_registry.list_templates()?;
        
        if templates.is_empty() {
            println!("No templates available.");
            println!("Install templates with: workspace-tools template install-repo <url>");
            return Ok(());
        }
        
        println!("📚 Available Templates:\n");
        
        let mut table = Vec::new();
        table.push(vec!["Name", "Version", "Complexity", "Maturity", "Description"]);
        table.push(vec!["----", "-------", "----------", "--------", "-----------"]);
        
        for template in templates {
            table.push(vec![
                &template.name,
                &template.version,
                &format!("{:?}", template.complexity),
                &format!("{:?}", template.maturity),
                &template.description,
            ]);
        }
        
        // Print formatted table
        self.print_table(&table);
        
        Ok(())
    }
}
```

## **Success Criteria**
- [ ] Interactive scaffolding wizard working smoothly
- [ ] Template inheritance and composition system functional
- [ ] Framework-specific templates (minimum 5 production-ready templates)
- [ ] Template repository system with sync capabilities
- [ ] Code generators producing high-quality, customized code
- [ ] CLI integration providing excellent user experience  
- [ ] Template validation and update mechanisms
- [ ] Comprehensive documentation and examples

## **Metrics to Track**
- Number of available templates in ecosystem
- Template usage statistics and popularity
- User satisfaction with generated project quality
- Time-to-productivity improvements for new projects
- Community contributions of custom templates

## **Future Enhancements**
- Visual template designer with drag-and-drop interface
- AI-powered template recommendations based on project requirements
- Integration with popular project management tools (Jira, Trello)
- Template versioning and automatic migration tools
- Community marketplace for sharing custom templates
- Integration with cloud deployment platforms (AWS, GCP, Azure)

This advanced scaffolding system transforms workspace_tools from a simple path resolution library into a comprehensive project generation and management platform, making it indispensable for Rust developers starting new projects.