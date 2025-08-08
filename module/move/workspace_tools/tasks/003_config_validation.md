# Task 003: Config Validation

**Priority**: ⚙️ Medium-High Impact  
**Phase**: 1 (Immediate)  
**Estimated Effort**: 3-4 days  
**Dependencies**: None (can be standalone)  

## **Objective**
Implement schema-based configuration validation to prevent runtime configuration errors, provide type-safe configuration loading, and improve developer experience with clear validation messages.

## **Technical Requirements**

### **Core Features**
1. **Schema Validation**
   - JSON Schema support for configuration files
   - TOML, YAML, and JSON format support
   - Custom validation rules and constraints
   - Clear error messages with line numbers

2. **Type-Safe Loading**
   - Direct deserialization to Rust structs
   - Optional field handling
   - Default value support
   - Environment variable overrides

3. **Runtime Validation**
   - Configuration hot-reloading with validation
   - Validation caching for performance
   - Incremental validation

### **New API Surface**
```rust
impl Workspace {
    /// Load and validate configuration with schema
    pub fn load_config_with_schema<T>(
        &self, 
        config_name: &str, 
        schema: &str
    ) -> Result<T> 
    where 
        T: serde::de::DeserializeOwned;
    
    /// Load configuration with embedded schema
    pub fn load_config<T>(&self, config_name: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned + ConfigSchema;
    
    /// Validate configuration file against schema
    pub fn validate_config_file<P: AsRef<Path>>(
        &self,
        config_path: P,
        schema: &str
    ) -> Result<ConfigValidation>;
    
    /// Get configuration with environment overrides
    pub fn load_config_with_env<T>(
        &self,
        config_name: &str,
        env_prefix: &str
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned + ConfigSchema;
}

/// Trait for types that can provide their own validation schema
pub trait ConfigSchema {
    fn json_schema() -> &'static str;
    fn config_name() -> &'static str;
}

#[derive(Debug, Clone)]
pub struct ConfigValidation {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub path: String,
    pub message: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

#[derive(Debug, Clone)]  
pub struct ValidationWarning {
    pub path: String,
    pub message: String,
    pub suggestion: Option<String>,
}
```

### **Implementation Steps**

#### **Step 1: Dependencies and Foundation** (Day 1)
```rust
// Add to Cargo.toml
[features]
default = ["enabled", "config_validation"]
config_validation = [
    "dep:serde",
    "dep:serde_json", 
    "dep:toml",
    "dep:serde_yaml",
    "dep:jsonschema",
]

[dependencies]
serde = { version = "1.0", features = ["derive"], optional = true }
serde_json = { version = "1.0", optional = true }
toml = { version = "0.8", optional = true }
serde_yaml = { version = "0.9", optional = true }
jsonschema = { version = "0.17", optional = true }

// Config validation module
#[cfg(feature = "config_validation")]
mod config_validation {
    use serde_json::{Value, from_str as json_from_str};
    use jsonschema::{JSONSchema, ValidationError as JsonSchemaError};
    use std::path::Path;
    
    pub struct ConfigValidator {
        schemas: std::collections::HashMap<String, JSONSchema>,
    }
    
    impl ConfigValidator {
        pub fn new() -> Self {
            Self {
                schemas: std::collections::HashMap::new(),
            }
        }
        
        pub fn add_schema(&mut self, name: &str, schema: &str) -> Result<()> {
            let schema_value: Value = json_from_str(schema)
                .map_err(|e| WorkspaceError::ConfigurationError(
                    format!("Invalid JSON schema: {}", e)
                ))?;
                
            let compiled = JSONSchema::compile(&schema_value)
                .map_err(|e| WorkspaceError::ConfigurationError(
                    format!("Schema compilation error: {}", e)
                ))?;
                
            self.schemas.insert(name.to_string(), compiled);
            Ok(())
        }
        
        pub fn validate_json(&self, schema_name: &str, json: &Value) -> Result<ConfigValidation> {
            let schema = self.schemas.get(schema_name)
                .ok_or_else(|| WorkspaceError::ConfigurationError(
                    format!("Schema '{}' not found", schema_name)
                ))?;
                
            let validation_result = schema.validate(json);
            
            match validation_result {
                Ok(_) => Ok(ConfigValidation {
                    valid: true,
                    errors: vec![],
                    warnings: vec![],
                }),
                Err(errors) => {
                    let validation_errors: Vec<ValidationError> = errors
                        .map(|error| ValidationError {
                            path: error.instance_path.to_string(),
                            message: error.to_string(),
                            line: None, // TODO: Extract from parsing
                            column: None,
                        })
                        .collect();
                        
                    Ok(ConfigValidation {
                        valid: false,
                        errors: validation_errors,
                        warnings: vec![],
                    })
                }
            }
        }
    }
}
```

#### **Step 2: Configuration Format Detection and Parsing** (Day 1-2)
```rust
#[cfg(feature = "config_validation")]
impl Workspace {
    /// Detect configuration file format from extension
    fn detect_config_format<P: AsRef<Path>>(path: P) -> Result<ConfigFormat> {
        let path = path.as_ref();
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("toml") => Ok(ConfigFormat::Toml),
            Some("yaml") | Some("yml") => Ok(ConfigFormat::Yaml),
            Some("json") => Ok(ConfigFormat::Json),
            _ => Err(WorkspaceError::ConfigurationError(
                format!("Unsupported config format: {}", path.display())
            ))
        }
    }
    
    /// Parse configuration file to JSON value for validation
    fn parse_config_to_json<P: AsRef<Path>>(
        &self, 
        config_path: P
    ) -> Result<serde_json::Value> {
        let path = config_path.as_ref();
        let content = std::fs::read_to_string(path)
            .map_err(|e| WorkspaceError::IoError(e.to_string()))?;
            
        let format = self.detect_config_format(path)?;
        
        match format {
            ConfigFormat::Json => {
                serde_json::from_str(&content)
                    .map_err(|e| WorkspaceError::ConfigurationError(
                        format!("JSON parsing error in {}: {}", path.display(), e)
                    ))
            }
            ConfigFormat::Toml => {
                let toml_value: toml::Value = toml::from_str(&content)
                    .map_err(|e| WorkspaceError::ConfigurationError(
                        format!("TOML parsing error in {}: {}", path.display(), e)
                    ))?;
                    
                // Convert TOML to JSON for validation
                let json_string = serde_json::to_string(&toml_value)
                    .map_err(|e| WorkspaceError::ConfigurationError(e.to_string()))?;
                serde_json::from_str(&json_string)
                    .map_err(|e| WorkspaceError::ConfigurationError(e.to_string()))
            }
            ConfigFormat::Yaml => {
                let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)
                    .map_err(|e| WorkspaceError::ConfigurationError(
                        format!("YAML parsing error in {}: {}", path.display(), e)
                    ))?;
                    
                // Convert YAML to JSON for validation
                serde_json::to_value(yaml_value)
                    .map_err(|e| WorkspaceError::ConfigurationError(e.to_string()))
            }
        }
    }
}

#[derive(Debug, Clone)]
enum ConfigFormat {
    Json,
    Toml, 
    Yaml,
}
```

#### **Step 3: Main Configuration Loading API** (Day 2-3)
```rust
#[cfg(feature = "config_validation")]
impl Workspace {
    pub fn load_config_with_schema<T>(
        &self,
        config_name: &str,
        schema: &str
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned
    {
        // Find configuration file
        let config_path = self.find_config(config_name)?;
        
        // Parse to JSON for validation
        let json_value = self.parse_config_to_json(&config_path)?;
        
        // Validate against schema
        let mut validator = ConfigValidator::new();
        validator.add_schema("config", schema)?;
        let validation = validator.validate_json("config", &json_value)?;
        
        if !validation.valid {
            let errors: Vec<String> = validation.errors.iter()
                .map(|e| format!("{}: {}", e.path, e.message))
                .collect();
            return Err(WorkspaceError::ConfigurationError(
                format!("Configuration validation failed:\n{}", errors.join("\n"))
            ));
        }
        
        // Deserialize to target type
        serde_json::from_value(json_value)
            .map_err(|e| WorkspaceError::ConfigurationError(e.to_string()))
    }
    
    pub fn load_config<T>(&self, config_name: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned + ConfigSchema
    {
        self.load_config_with_schema(config_name, T::json_schema())
    }
    
    pub fn validate_config_file<P: AsRef<Path>>(
        &self,
        config_path: P,
        schema: &str
    ) -> Result<ConfigValidation> {
        let json_value = self.parse_config_to_json(config_path)?;
        
        let mut validator = ConfigValidator::new();
        validator.add_schema("validation", schema)?;
        validator.validate_json("validation", &json_value)
    }
    
    pub fn load_config_with_env<T>(
        &self,
        config_name: &str,
        env_prefix: &str
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned + ConfigSchema
    {
        // Load base configuration
        let mut config = self.load_config::<T>(config_name)?;
        
        // Override with environment variables
        self.apply_env_overrides(&mut config, env_prefix)?;
        
        Ok(config)
    }
    
    fn apply_env_overrides<T>(&self, config: &mut T, env_prefix: &str) -> Result<()>
    where 
        T: serde::Serialize + serde::de::DeserializeOwned
    {
        // Convert to JSON for manipulation
        let mut json_value = serde_json::to_value(&config)
            .map_err(|e| WorkspaceError::ConfigurationError(e.to_string()))?;
            
        // Apply environment variable overrides
        for (key, value) in std::env::vars() {
            if key.starts_with(env_prefix) {
                let config_key = key.strip_prefix(env_prefix)
                    .unwrap()
                    .to_lowercase()
                    .replace('_', ".");
                    
                self.set_json_value(&mut json_value, &config_key, value)?;
            }
        }
        
        // Convert back to target type
        *config = serde_json::from_value(json_value)
            .map_err(|e| WorkspaceError::ConfigurationError(e.to_string()))?;
            
        Ok(())
    }
    
    fn set_json_value(
        &self, 
        json: &mut serde_json::Value, 
        path: &str, 
        value: String
    ) -> Result<()> {
        // Simple nested key setting (e.g., "database.host" -> json["database"]["host"])
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = json;
        
        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                // Last part - set the value
                current[part] = serde_json::Value::String(value.clone());
            } else {
                // Ensure the path exists
                if !current.is_object() {
                    current[part] = serde_json::json!({});
                }
                current = &mut current[part];
            }
        }
        
        Ok(())
    }
}
```

#### **Step 4: Schema Definition Helpers and Macros** (Day 3-4)
```rust
// Procedural macro for automatic schema generation (future enhancement)
// For now, manual schema definition helper

#[cfg(feature = "config_validation")]
pub mod schema {
    /// Helper to create common JSON schemas
    pub struct SchemaBuilder {
        schema: serde_json::Value,
    }
    
    impl SchemaBuilder {
        pub fn new() -> Self {
            Self {
                schema: serde_json::json!({
                    "$schema": "http://json-schema.org/draft-07/schema#",
                    "type": "object",
                    "properties": {},
                    "required": []
                })
            }
        }
        
        pub fn add_string_field(mut self, name: &str, required: bool) -> Self {
            self.schema["properties"][name] = serde_json::json!({
                "type": "string"
            });
            
            if required {
                self.schema["required"].as_array_mut().unwrap()
                    .push(serde_json::Value::String(name.to_string()));
            }
            
            self
        }
        
        pub fn add_integer_field(mut self, name: &str, min: Option<i64>, max: Option<i64>) -> Self {
            let mut field_schema = serde_json::json!({
                "type": "integer"
            });
            
            if let Some(min_val) = min {
                field_schema["minimum"] = serde_json::Value::Number(min_val.into());
            }
            if let Some(max_val) = max {
                field_schema["maximum"] = serde_json::Value::Number(max_val.into());
            }
            
            self.schema["properties"][name] = field_schema;
            self
        }
        
        pub fn build(self) -> String {
            serde_json::to_string_pretty(&self.schema).unwrap()
        }
    }
}

// Example usage in application configs
use workspace_tools::{ConfigSchema, schema::SchemaBuilder};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AppConfig {
    pub name: String,
    pub port: u16,
    pub database_url: String,
    pub log_level: String,
    pub max_connections: Option<u32>,
}

impl ConfigSchema for AppConfig {
    fn json_schema() -> &'static str {
        r#"{
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "properties": {
                "name": {"type": "string", "minLength": 1},
                "port": {"type": "integer", "minimum": 1, "maximum": 65535},
                "database_url": {"type": "string", "format": "uri"},
                "log_level": {
                    "type": "string",
                    "enum": ["error", "warn", "info", "debug", "trace"]
                },
                "max_connections": {"type": "integer", "minimum": 1}
            },
            "required": ["name", "port", "database_url", "log_level"],
            "additionalProperties": false
        }"#
    }
    
    fn config_name() -> &'static str {
        "app"
    }
}
```

#### **Step 5: Testing and Examples** (Day 4)
```rust
#[cfg(test)]
#[cfg(feature = "config_validation")]
mod config_validation_tests {
    use super::*;
    use crate::testing::create_test_workspace_with_structure;
    
    #[derive(serde::Deserialize, serde::Serialize)]
    struct TestConfig {
        name: String,
        port: u16,
        enabled: bool,
    }
    
    impl ConfigSchema for TestConfig {
        fn json_schema() -> &'static str {
            r#"{
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "port": {"type": "integer", "minimum": 1, "maximum": 65535},
                    "enabled": {"type": "boolean"}
                },
                "required": ["name", "port"],
                "additionalProperties": false
            }"#
        }
        
        fn config_name() -> &'static str { "test" }
    }
    
    #[test]
    fn test_valid_config_loading() {
        let (_temp_dir, ws) = create_test_workspace_with_structure();
        
        let config_content = r#"
name = "test_app"
port = 8080
enabled = true
"#;
        
        std::fs::write(ws.config_dir().join("test.toml"), config_content).unwrap();
        
        let config: TestConfig = ws.load_config("test").unwrap();
        assert_eq!(config.name, "test_app");
        assert_eq!(config.port, 8080);
        assert_eq!(config.enabled, true);
    }
    
    #[test] 
    fn test_invalid_config_validation() {
        let (_temp_dir, ws) = create_test_workspace_with_structure();
        
        let invalid_config = r#"
name = "test_app"
port = 99999  # Invalid port number
enabled = "not_a_boolean"
"#;
        
        std::fs::write(ws.config_dir().join("test.toml"), invalid_config).unwrap();
        
        let result = ws.load_config::<TestConfig>("test");
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        match error {
            WorkspaceError::ConfigurationError(msg) => {
                assert!(msg.contains("validation failed"));
                assert!(msg.contains("port"));
            }
            _ => panic!("Expected configuration error"),
        }
    }
    
    #[test]
    fn test_environment_overrides() {
        let (_temp_dir, ws) = create_test_workspace_with_structure();
        
        let config_content = r#"
name = "test_app" 
port = 8080
enabled = false
"#;
        
        std::fs::write(ws.config_dir().join("test.toml"), config_content).unwrap();
        
        // Set environment overrides
        std::env::set_var("APP_PORT", "9000");
        std::env::set_var("APP_ENABLED", "true");
        
        let config: TestConfig = ws.load_config_with_env("test", "APP_").unwrap();
        
        assert_eq!(config.name, "test_app"); // Not overridden
        assert_eq!(config.port, 9000); // Overridden
        assert_eq!(config.enabled, true); // Overridden
        
        // Cleanup
        std::env::remove_var("APP_PORT");
        std::env::remove_var("APP_ENABLED");
    }
}
```

### **Documentation Updates**

#### **README.md Addition**
```markdown
## ⚙️ configuration validation

workspace_tools provides schema-based configuration validation:

```rust
use workspace_tools::{workspace, ConfigSchema};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct AppConfig {
    name: String,
    port: u16,
    database_url: String,
}

impl ConfigSchema for AppConfig {
    fn json_schema() -> &'static str {
        r#"{"type": "object", "properties": {...}}"#
    }
    
    fn config_name() -> &'static str { "app" }
}

let ws = workspace()?;
let config: AppConfig = ws.load_config("app")?; // Validates automatically
```

**Features:**
- Type-safe configuration loading
- JSON Schema validation  
- Environment variable overrides
- Support for TOML, YAML, and JSON formats
```

#### **New Example: config_validation.rs**
```rust
//! Configuration validation example

use workspace_tools::{workspace, ConfigSchema, schema::SchemaBuilder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
    database: String,
    ssl: bool,
    max_connections: Option<u32>,
}

impl ConfigSchema for DatabaseConfig {
    fn json_schema() -> &'static str {
        r#"{
            "type": "object",
            "properties": {
                "host": {"type": "string"},
                "port": {"type": "integer", "minimum": 1, "maximum": 65535},
                "username": {"type": "string", "minLength": 1},
                "database": {"type": "string", "minLength": 1},
                "ssl": {"type": "boolean"},
                "max_connections": {"type": "integer", "minimum": 1, "maximum": 1000}
            },
            "required": ["host", "port", "username", "database"],
            "additionalProperties": false
        }"#
    }
    
    fn config_name() -> &'static str { "database" }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws = workspace()?;
    
    println!("⚙️  Configuration Validation Demo");
    
    // Load and validate configuration
    match ws.load_config::<DatabaseConfig>("database") {
        Ok(config) => {
            println!("✅ Configuration loaded successfully:");
            println!("   Database: {}@{}:{}/{}", 
                config.username, config.host, config.port, config.database);
            println!("   SSL: {}", config.ssl);
            if let Some(max_conn) = config.max_connections {
                println!("   Max connections: {}", max_conn);
            }
        }
        Err(e) => {
            println!("❌ Configuration validation failed:");
            println!("   {}", e);
        }
    }
    
    // Example with environment overrides
    println!("\n🌍 Testing environment overrides...");
    std::env::set_var("DB_HOST", "production-db.example.com");
    std::env::set_var("DB_SSL", "true");
    
    match ws.load_config_with_env::<DatabaseConfig>("database", "DB_") {
        Ok(config) => {
            println!("✅ Configuration with env overrides:");
            println!("   Host: {} (from env)", config.host);
            println!("   SSL: {} (from env)", config.ssl);
        }
        Err(e) => {
            println!("❌ Failed: {}", e);
        }
    }
    
    Ok(())
}
```

### **Success Criteria**
- [ ] JSON Schema validation for all config formats
- [ ] Type-safe configuration loading with serde
- [ ] Environment variable override support
- [ ] Clear validation error messages with paths
- [ ] Support for TOML, YAML, and JSON formats
- [ ] Schema builder helper utilities
- [ ] Comprehensive test coverage
- [ ] Performance: Validation completes in <50ms

### **Future Enhancements**
- Procedural macro for automatic schema generation
- Configuration hot-reloading with validation
- IDE integration for configuration IntelliSense
- Configuration documentation generation from schemas
- Advanced validation rules (custom validators)

### **Breaking Changes**
None - this is purely additive functionality with feature flag.