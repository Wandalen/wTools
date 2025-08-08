# Task 005: Serde Integration

**Status**: ✅ **COMPLETED**  
**Priority**: 📄 High Impact  
**Phase**: 2 (Ecosystem Integration)  
**Estimated Effort**: 3-4 days  
**Dependencies**: Task 003 (Config Validation) recommended  
**Completion Date**: 2024-08-08  

## **Implementation Summary**
✅ **All core features implemented and fully tested:**
- Auto-format detection configuration loading via `load_config()`
- Multi-format support: TOML, JSON, YAML with `load_config_from()`
- Configuration serialization via `save_config()` and `save_config_to()`
- Layered configuration merging with `load_config_layered()`
- Partial configuration updates via `update_config()`
- 10 comprehensive tests covering all serde integration scenarios
- Feature flag: `serde_integration` with optional dependencies

## **Objective**
Provide first-class serde integration for seamless configuration management, eliminating boilerplate code and making workspace_tools the standard choice for configuration loading in Rust applications.

## **Technical Requirements**

### **Core Features**
1. **Direct Serde Deserialization**
   - Auto-detect format (TOML/YAML/JSON) from file extension
   - Zero-copy deserialization where possible
   - Custom deserializers for workspace-specific types

2. **Configuration Serialization**
   - Save configurations back to files
   - Format preservation and pretty-printing
   - Atomic writes to prevent corruption

3. **Advanced Features**
   - Partial configuration updates
   - Configuration merging and overlays
   - Custom field processing (e.g., path resolution)

### **New API Surface**
```rust
impl Workspace {
    /// Load configuration with automatic format detection
    pub fn load_config<T>(&self, name: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned;
    
    /// Load configuration from specific file
    pub fn load_config_from<T, P>(&self, path: P) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        P: AsRef<Path>;
    
    /// Save configuration with format matching the original
    pub fn save_config<T>(&self, name: &str, config: &T) -> Result<()>
    where
        T: serde::Serialize;
    
    /// Save configuration to specific file with format detection
    pub fn save_config_to<T, P>(&self, path: P, config: &T) -> Result<()>
    where
        T: serde::Serialize,
        P: AsRef<Path>;
    
    /// Load and merge multiple configuration layers
    pub fn load_config_layered<T>(&self, names: &[&str]) -> Result<T>
    where
        T: serde::de::DeserializeOwned + ConfigMerge;
    
    /// Update configuration partially
    pub fn update_config<T, U>(&self, name: &str, updates: U) -> Result<T>
    where
        T: serde::de::DeserializeOwned + serde::Serialize,
        U: serde::Serialize;
}

/// Trait for configuration types that can be merged
pub trait ConfigMerge: Sized {
    fn merge(self, other: Self) -> Self;
}

/// Workspace-aware serde deserializer
#[derive(Debug)]
pub struct WorkspaceDeserializer<'ws> {
    workspace: &'ws Workspace,
}

/// Custom serde field for workspace-relative paths
#[derive(Debug, Clone, PartialEq)]
pub struct WorkspacePath(PathBuf);
```

### **Implementation Steps**

#### **Step 1: Core Serde Integration** (Day 1)
```rust
// Add to Cargo.toml
[features]
default = ["enabled", "serde_integration"]
serde_integration = [
    "dep:serde",
    "dep:serde_json",
    "dep:toml", 
    "dep:serde_yaml",
]

[dependencies]
serde = { version = "1.0", features = ["derive"], optional = true }
serde_json = { version = "1.0", optional = true }
toml = { version = "0.8", features = ["preserve_order"], optional = true }
serde_yaml = { version = "0.9", optional = true }

// Core implementation
#[cfg(feature = "serde_integration")]
impl Workspace {
    pub fn load_config<T>(&self, name: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let config_path = self.find_config(name)?;
        self.load_config_from(config_path)
    }
    
    pub fn load_config_from<T, P>(&self, path: P) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let full_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.join(path)
        };
        
        let content = std::fs::read_to_string(&full_path)
            .map_err(|e| WorkspaceError::IoError(format!(
                "Failed to read config file {}: {}", full_path.display(), e
            )))?;
        
        self.deserialize_config(&content, &full_path)
    }
    
    fn deserialize_config<T>(&self, content: &str, path: &Path) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let format = self.detect_config_format(path)?;
        
        match format {
            ConfigFormat::Json => {
                serde_json::from_str(content)
                    .map_err(|e| WorkspaceError::ConfigurationError(
                        format!("JSON parsing error in {}: {}", path.display(), e)
                    ))
            }
            ConfigFormat::Toml => {
                toml::from_str(content)
                    .map_err(|e| WorkspaceError::ConfigurationError(
                        format!("TOML parsing error in {}: {}", path.display(), e)
                    ))
            }
            ConfigFormat::Yaml => {
                serde_yaml::from_str(content)
                    .map_err(|e| WorkspaceError::ConfigurationError(
                        format!("YAML parsing error in {}: {}", path.display(), e)
                    ))
            }
        }
    }
    
    fn detect_config_format(&self, path: &Path) -> Result<ConfigFormat> {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => Ok(ConfigFormat::Json),
            Some("toml") => Ok(ConfigFormat::Toml),
            Some("yaml") | Some("yml") => Ok(ConfigFormat::Yaml),
            _ => Err(WorkspaceError::ConfigurationError(
                format!("Unknown config format for file: {}", path.display())
            )),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ConfigFormat {
    Json,
    Toml,
    Yaml,
}
```

#### **Step 2: Configuration Serialization** (Day 2)
```rust
#[cfg(feature = "serde_integration")]
impl Workspace {
    pub fn save_config<T>(&self, name: &str, config: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let config_path = self.find_config(name)
            .or_else(|_| {
                // If config doesn't exist, create default path with .toml extension
                Ok(self.config_dir().join(format!("{}.toml", name)))
            })?;
        
        self.save_config_to(config_path, config)
    }
    
    pub fn save_config_to<T, P>(&self, path: P, config: &T) -> Result<()>
    where
        T: serde::Serialize,
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let full_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.join(path)
        };
        
        // Ensure parent directory exists
        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| WorkspaceError::IoError(e.to_string()))?;
        }
        
        let content = self.serialize_config(config, &full_path)?;
        
        // Atomic write: write to temp file, then rename
        let temp_path = full_path.with_extension("tmp");
        std::fs::write(&temp_path, content)
            .map_err(|e| WorkspaceError::IoError(e.to_string()))?;
        std::fs::rename(&temp_path, &full_path)
            .map_err(|e| WorkspaceError::IoError(e.to_string()))?;
        
        Ok(())
    }
    
    fn serialize_config<T>(&self, config: &T, path: &Path) -> Result<String>
    where
        T: serde::Serialize,
    {
        let format = self.detect_config_format(path)?;
        
        match format {
            ConfigFormat::Json => {
                serde_json::to_string_pretty(config)
                    .map_err(|e| WorkspaceError::ConfigurationError(e.to_string()))
            }
            ConfigFormat::Toml => {
                toml::to_string_pretty(config)
                    .map_err(|e| WorkspaceError::ConfigurationError(e.to_string()))
            }
            ConfigFormat::Yaml => {
                serde_yaml::to_string(config)
                    .map_err(|e| WorkspaceError::ConfigurationError(e.to_string()))
            }
        }
    }
    
    /// Update existing configuration with partial data
    pub fn update_config<T, U>(&self, name: &str, updates: U) -> Result<T>
    where
        T: serde::de::DeserializeOwned + serde::Serialize,
        U: serde::Serialize,
    {
        // Load existing config
        let mut existing: T = self.load_config(name)?;
        
        // Convert to JSON values for merging
        let mut existing_value = serde_json::to_value(&existing)
            .map_err(|e| WorkspaceError::ConfigurationError(e.to_string()))?;
        let updates_value = serde_json::to_value(updates)
            .map_err(|e| WorkspaceError::ConfigurationError(e.to_string()))?;
        
        // Merge updates into existing config
        merge_json_values(&mut existing_value, updates_value);
        
        // Convert back to target type
        let updated_config: T = serde_json::from_value(existing_value)
            .map_err(|e| WorkspaceError::ConfigurationError(e.to_string()))?;
        
        // Save updated config
        self.save_config(name, &updated_config)?;
        
        Ok(updated_config)
    }
}

fn merge_json_values(target: &mut serde_json::Value, source: serde_json::Value) {
    use serde_json::Value;
    
    match (target, source) {
        (Value::Object(target_map), Value::Object(source_map)) => {
            for (key, value) in source_map {
                match target_map.get_mut(&key) {
                    Some(target_value) => merge_json_values(target_value, value),
                    None => { target_map.insert(key, value); }
                }
            }
        }
        (target_value, source_value) => *target_value = source_value,
    }
}
```

#### **Step 3: Configuration Layering and Merging** (Day 3)
```rust
/// Trait for configuration types that support merging
pub trait ConfigMerge: Sized {
    fn merge(self, other: Self) -> Self;
}

#[cfg(feature = "serde_integration")]
impl Workspace {
    pub fn load_config_layered<T>(&self, names: &[&str]) -> Result<T>
    where
        T: serde::de::DeserializeOwned + ConfigMerge,
    {
        let mut configs = Vec::new();
        
        for name in names {
            match self.load_config::<T>(name) {
                Ok(config) => configs.push(config),
                Err(WorkspaceError::PathNotFound(_)) => {
                    // Skip missing optional configs
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
        
        if configs.is_empty() {
            return Err(WorkspaceError::PathNotFound(
                self.config_dir().join("no_configs_found")
            ));
        }
        
        // Merge all configs together
        let mut result = configs.into_iter().next().unwrap();
        for config in configs {
            result = result.merge(config);
        }
        
        Ok(result)
    }
    
    /// Load configuration with environment-specific overlays
    pub fn load_config_with_environment<T>(&self, base_name: &str, env: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned + ConfigMerge,
    {
        let configs_to_try = vec![
            base_name.to_string(),
            format!("{}.{}", base_name, env),
            format!("{}.local", base_name),
        ];
        
        let config_names: Vec<&str> = configs_to_try.iter().map(|s| s.as_str()).collect();
        self.load_config_layered(&config_names)
    }
}

// Example implementation of ConfigMerge for common patterns
impl ConfigMerge for serde_json::Value {
    fn merge(mut self, other: Self) -> Self {
        merge_json_values(&mut self, other);
        self
    }
}

// Derive macro helper (future enhancement)
/*
#[derive(serde::Deserialize, serde::Serialize, ConfigMerge)]
struct AppConfig {
    #[merge(strategy = "replace")]
    name: String,
    
    #[merge(strategy = "merge")]
    database: DatabaseConfig,
    
    #[merge(strategy = "append")]
    plugins: Vec<String>,
}
*/
```

#### **Step 4: Workspace-Aware Custom Types** (Day 3-4)
```rust
/// Custom serde type for workspace-relative paths
#[derive(Debug, Clone, PartialEq)]
pub struct WorkspacePath(PathBuf);

impl WorkspacePath {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self(path.as_ref().to_path_buf())
    }
    
    pub fn as_path(&self) -> &Path {
        &self.0
    }
    
    pub fn resolve(&self, workspace: &Workspace) -> PathBuf {
        if self.0.is_absolute() {
            self.0.clone()
        } else {
            workspace.join(&self.0)
        }
    }
}

impl<'de> serde::Deserialize<'de> for WorkspacePath {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path_str = String::deserialize(deserializer)?;
        Ok(WorkspacePath::new(path_str))
    }
}

impl serde::Serialize for WorkspacePath {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.to_string_lossy().serialize(serializer)
    }
}

/// Workspace context for custom deserialization
#[cfg(feature = "serde_integration")]
pub struct WorkspaceDeserializer<'ws> {
    workspace: &'ws Workspace,
}

impl<'ws> WorkspaceDeserializer<'ws> {
    pub fn new(workspace: &'ws Workspace) -> Self {
        Self { workspace }
    }
    
    pub fn deserialize_with_workspace<T>(&self, content: &str, path: &Path) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        // TODO: Implement workspace-aware deserialization
        // This would allow configurations to reference workspace paths
        // and have them automatically resolved during deserialization
        self.workspace.deserialize_config(content, path)
    }
}

// Environment variable substitution in configs
#[derive(Debug, Clone)]
pub struct EnvVar(String);

impl<'de> serde::Deserialize<'de> for EnvVar {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let var_name = String::deserialize(deserializer)?;
        Ok(EnvVar(var_name))
    }
}

impl serde::Serialize for EnvVar {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match std::env::var(&self.0) {
            Ok(value) => value.serialize(serializer),
            Err(_) => format!("${{{}}}", self.0).serialize(serializer),
        }
    }
}
```

#### **Step 5: Testing and Examples** (Day 4)
```rust
#[cfg(test)]
#[cfg(feature = "serde_integration")]
mod serde_integration_tests {
    use super::*;
    use crate::testing::create_test_workspace_with_structure;
    use serde::{Deserialize, Serialize};
    
    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct TestConfig {
        name: String,
        port: u16,
        features: Vec<String>,
        database: DatabaseConfig,
    }
    
    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct DatabaseConfig {
        host: String,
        port: u16,
        ssl: bool,
    }
    
    impl ConfigMerge for TestConfig {
        fn merge(mut self, other: Self) -> Self {
            // Simple merge strategy - other values override self
            Self {
                name: other.name,
                port: other.port,
                features: {
                    let mut combined = self.features;
                    combined.extend(other.features);
                    combined.sort();
                    combined.dedup();
                    combined
                },
                database: other.database,
            }
        }
    }
    
    #[test]
    fn test_config_loading_toml() {
        let (_temp_dir, ws) = create_test_workspace_with_structure();
        
        let config_content = r#"
name = "test_app"
port = 8080
features = ["logging", "metrics"]

[database]
host = "localhost"
port = 5432
ssl = false
"#;
        
        std::fs::write(ws.config_dir().join("app.toml"), config_content).unwrap();
        
        let config: TestConfig = ws.load_config("app").unwrap();
        assert_eq!(config.name, "test_app");
        assert_eq!(config.port, 8080);
        assert_eq!(config.features, vec!["logging", "metrics"]);
        assert_eq!(config.database.host, "localhost");
    }
    
    #[test]
    fn test_config_loading_yaml() {
        let (_temp_dir, ws) = create_test_workspace_with_structure();
        
        let config_content = r#"
name: yaml_app
port: 9000
features:
  - security
  - caching
database:
  host: db.example.com
  port: 3306
  ssl: true
"#;
        
        std::fs::write(ws.config_dir().join("app.yaml"), config_content).unwrap();
        
        let config: TestConfig = ws.load_config("app").unwrap();
        assert_eq!(config.name, "yaml_app");
        assert_eq!(config.database.ssl, true);
    }
    
    #[test]
    fn test_config_saving() {
        let (_temp_dir, ws) = create_test_workspace_with_structure();
        
        let config = TestConfig {
            name: "saved_app".to_string(),
            port: 7000,
            features: vec!["auth".to_string()],
            database: DatabaseConfig {
                host: "saved.db".to_string(),
                port: 5433,
                ssl: true,
            },
        };
        
        ws.save_config("saved", &config).unwrap();
        
        // Verify file was created and can be loaded back
        let loaded_config: TestConfig = ws.load_config("saved").unwrap();
        assert_eq!(loaded_config, config);
    }
    
    #[test]
    fn test_config_updating() {
        let (_temp_dir, ws) = create_test_workspace_with_structure();
        
        // Create initial config
        let initial_config = TestConfig {
            name: "initial".to_string(),
            port: 8000,
            features: vec!["basic".to_string()],
            database: DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                ssl: false,
            },
        };
        
        ws.save_config("updatetest", &initial_config).unwrap();
        
        // Update with partial data
        #[derive(Serialize)]
        struct PartialUpdate {
            port: u16,
            features: Vec<String>,
        }
        
        let updates = PartialUpdate {
            port: 8080,
            features: vec!["basic".to_string(), "advanced".to_string()],
        };
        
        let updated_config: TestConfig = ws.update_config("updatetest", updates).unwrap();
        
        // Verify updates were applied
        assert_eq!(updated_config.name, "initial"); // Unchanged
        assert_eq!(updated_config.port, 8080); // Updated
        assert_eq!(updated_config.features, vec!["basic", "advanced"]); // Updated
    }
    
    #[test]
    fn test_layered_config_loading() {
        let (_temp_dir, ws) = create_test_workspace_with_structure();
        
        // Base config
        let base_config = r#"
name = "layered_app"
port = 8080
features = ["base"]

[database]
host = "localhost"
port = 5432
ssl = false
"#;
        std::fs::write(ws.config_dir().join("base.toml"), base_config).unwrap();
        
        // Environment-specific config
        let env_config = r#"
port = 9000
features = ["env_specific"]

[database]
ssl = true
"#;
        std::fs::write(ws.config_dir().join("production.toml"), env_config).unwrap();
        
        let merged_config: TestConfig = ws.load_config_layered(&["base", "production"]).unwrap();
        
        assert_eq!(merged_config.name, "layered_app");
        assert_eq!(merged_config.port, 9000); // Overridden
        assert_eq!(merged_config.database.ssl, true); // Overridden
        assert!(merged_config.features.contains(&"base".to_string()));
        assert!(merged_config.features.contains(&"env_specific".to_string()));
    }
    
    #[test]
    fn test_workspace_path_type() {
        let workspace_path = WorkspacePath::new("config/app.toml");
        let json = serde_json::to_string(&workspace_path).unwrap();
        assert_eq!(json, r#""config/app.toml""#);
        
        let deserialized: WorkspacePath = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, workspace_path);
    }
}
```

### **Documentation Updates**

#### **README.md Addition**
```markdown
## 📄 serde integration

workspace_tools provides seamless serde integration for configuration management:

```rust
use workspace_tools::workspace;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct AppConfig {
    name: String,
    port: u16,
    database_url: String,
}

let ws = workspace()?;

// Load with automatic format detection (TOML/YAML/JSON)
let config: AppConfig = ws.load_config("app")?;

// Save configuration back
ws.save_config("app", &config)?;

// Update configuration partially
#[derive(Serialize)]
struct Update { port: u16 }
let updated: AppConfig = ws.update_config("app", Update { port: 9000 })?;
```

**Features:**
- Automatic format detection and conversion
- Configuration layering and merging
- Workspace-relative path types
- Environment variable substitution
```

### **Success Criteria**
- [ ] Zero-boilerplate configuration loading/saving
- [ ] Automatic format detection (TOML/YAML/JSON)
- [ ] Configuration merging and layering support
- [ ] Custom workspace-aware serde types
- [ ] Partial configuration updates
- [ ] Atomic file operations for safety
- [ ] Comprehensive test coverage
- [ ] Excellent error messages with context

### **Future Enhancements**
- Procedural macro for auto-implementing ConfigMerge
- Configuration schema generation from Rust types  
- Hot-reloading integration with serde
- Advanced environment variable interpolation
- Configuration validation with custom serde validators

### **Breaking Changes**
None - this is purely additive functionality with feature flag.

This task makes workspace_tools the definitive choice for configuration management in Rust applications by eliminating all serde boilerplate.