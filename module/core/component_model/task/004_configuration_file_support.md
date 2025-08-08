# Task 004: Configuration File Support

## 🎯 **Objective**

Integrate component model with popular configuration formats (TOML, YAML, JSON) and the `config` crate to provide seamless configuration loading with environment variable overrides and profile support.

## 📋 **Current State**

Users must manually handle configuration loading:
```rust
// Manual approach
let config_str = std::fs::read_to_string("config.toml")?;
let parsed: ConfigData = toml::from_str(&config_str)?;

let mut app_config = AppConfig::default();
app_config.assign(parsed.database.host);
app_config.assign(parsed.database.port);
// ... lots of manual mapping
```

## 🎯 **Target State**

Seamless configuration loading with component model:
```rust
#[derive(ComponentModel, Config)]
struct AppConfig {
  #[config(env = "DATABASE_HOST")]
  database_host: String,
  
  #[config(env = "DATABASE_PORT", default = "5432")]
  database_port: u16,
  
  #[config(profile = "production")]
  ssl_enabled: bool,
}

// Load from file with environment overrides
let config = AppConfig::from_config_file("app.toml")
  .with_env_overrides()
  .with_profile("production")
  .build()?;

// Or build programmatically
let config = AppConfig::default()
  .impute("localhost")       // database_host
  .impute(5432u16)          // database_port
  .impute(true)             // ssl_enabled
  .load_from_env()          // Override with env vars
  .validate()?;             // Run validation
```

## 📝 **Detailed Requirements**

### **Core Configuration API**

#### **Config Derive**
```rust
#[proc_macro_derive(Config, attributes(config))]
pub fn derive_config(input: TokenStream) -> TokenStream {
  // Generate configuration loading methods
}
```

#### **Configuration Loading Methods**
```rust
impl AppConfig {
  // File loading
  fn from_config_file<P: AsRef<Path>>(path: P) -> ConfigBuilder<Self>;
  fn from_toml<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError>;
  fn from_yaml<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError>;
  fn from_json<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError>;
  
  // Environment loading
  fn from_env() -> Result<Self, ConfigError>;
  fn from_env_with_prefix(prefix: &str) -> Result<Self, ConfigError>;
  
  // Builder pattern
  fn config() -> ConfigBuilder<Self>;
}

pub struct ConfigBuilder<T> {
  // Builder state
}

impl<T> ConfigBuilder<T> {
  fn from_file<P: AsRef<Path>>(self, path: P) -> Self;
  fn from_env(self) -> Self;
  fn with_profile(self, profile: &str) -> Self;
  fn with_overrides<F>(self, f: F) -> Self where F: Fn(&mut T);
  fn build(self) -> Result<T, ConfigError>;
}
```

### **Attribute System**

#### **Field Attributes**
```rust
#[derive(ComponentModel, Config)]
struct DatabaseConfig {
  // Environment variable mapping
  #[config(env = "DB_HOST")]
  host: String,
  
  // Default value
  #[config(default = "5432")]
  port: u16,
  
  // Profile-specific values
  #[config(profile = "production", default = "true")]
  #[config(profile = "development", default = "false")]
  ssl_required: bool,
  
  // Nested configuration
  #[config(nested)]
  connection_pool: PoolConfig,
  
  // Custom deserializer
  #[config(deserialize_with = "parse_duration")]
  timeout: Duration,
}
```

#### **Container Attributes**
```rust
#[derive(ComponentModel, Config)]
#[config(prefix = "APP")]              // Environment prefix
#[config(file = "app.toml")]           // Default config file
#[config(profiles = ["dev", "prod"])]  // Available profiles
struct AppConfig {
  // fields...
}
```

### **Integration with Popular Crates**

#### **Config Crate Integration**
```rust
impl AppConfig {
  fn from_config_crate() -> Result<Self, ConfigError> {
    let settings = config::Config::builder()
      .add_source(config::File::with_name("config"))
      .add_source(config::Environment::with_prefix("APP"))
      .build()?;
      
    Self::from_config_settings(settings)
  }
  
  fn from_config_settings(settings: config::Config) -> Result<Self, ConfigError> {
    let mut instance = Self::default();
    
    // Use component model to assign values from config
    if let Ok(host) = settings.get_string("database.host") {
      instance.assign(host);
    }
    // ... etc
    
    Ok(instance)
  }
}
```

#### **Figment Integration** (Rocket's config system)
```rust
#[cfg(feature = "figment")]
impl Configurable for AppConfig {
  fn from_figment(figment: figment::Figment) -> Result<Self, figment::Error> {
    let mut config = Self::default();
    
    // Extract values and use component assignment
    let extracted = figment.extract::<ConfigData>()?;
    config.apply_config_data(extracted);
    
    Ok(config)
  }
}
```

### **Environment Variable Support**

#### **Automatic Mapping**
```rust
// Field name to environment variable mapping
struct Config {
  database_host: String,    // -> DATABASE_HOST
  api_key: String,          // -> API_KEY  
  worker_count: usize,      // -> WORKER_COUNT
}

// With prefix
#[config(prefix = "APP")]
struct Config {
  database_host: String,    // -> APP_DATABASE_HOST
}
```

#### **Custom Environment Mapping**
```rust
#[derive(Config)]
struct Config {
  #[config(env = "DB_URL")]
  database_url: String,
  
  #[config(env = "PORT", default = "8080")]
  server_port: u16,
}
```

### **Profile Support**

#### **Profile-Specific Values**
```rust
// config.toml
[default]
debug = false
workers = 1

[development]
debug = true
workers = 1

[production]
debug = false
workers = 8
ssl_required = true

// Usage
let config = AppConfig::from_config_file("config.toml")
  .with_profile("production")
  .build()?;
```

## 🗂️ **File Changes**

### **New Files**
- `component_model_config/` - New crate for configuration support
- `component_model_config/src/lib.rs` - Main configuration API
- `component_model_config/src/config_derive.rs` - Config derive implementation
- `component_model_config/src/formats/` - Format-specific loaders (TOML, YAML, JSON)
- `component_model_config/src/env.rs` - Environment variable support
- `component_model_config/src/profiles.rs` - Profile management
- `component_model_config/src/builder.rs` - Configuration builder
- `examples/config_example.rs` - Comprehensive configuration example

### **Modified Files**
- `Cargo.toml` - Add new workspace member
- `component_model/Cargo.toml` - Add config dependency (feature-gated)
- `component_model/src/lib.rs` - Re-export config functionality

## ⚡ **Implementation Steps**

### **Phase 1: Core Configuration (Week 1)**
1. Create `component_model_config` crate
2. Implement basic file loading for TOML/JSON/YAML
3. Create `Config` derive macro with basic functionality
4. Add environment variable mapping

### **Phase 2: Advanced Features (Week 2)**
1. Implement profile support
2. Add configuration builder pattern
3. Create integration with `config` crate
4. Add validation integration

### **Phase 3: Polish & Documentation (Week 2-3)**
1. Comprehensive examples and documentation
2. Error handling improvement
3. Performance optimization
4. Integration testing with real-world configs

## 🧪 **Testing Strategy**

### **Unit Tests**
```rust
#[cfg(test)]
mod tests {
  use super::*;
  use std::env;
  
  #[test]
  fn test_file_loading() {
    #[derive(ComponentModel, Config, Debug, PartialEq)]
    struct TestConfig {
      name: String,
      port: u16,
    }
    
    // Create test config file
    let config_content = r#"
      name = "test-app"
      port = 8080
    "#;
    std::fs::write("test_config.toml", config_content).unwrap();
    
    let config = TestConfig::from_toml("test_config.toml").unwrap();
    assert_eq!(config.name, "test-app");
    assert_eq!(config.port, 8080);
    
    std::fs::remove_file("test_config.toml").unwrap();
  }
  
  #[test] 
  fn test_env_override() {
    #[derive(ComponentModel, Config)]
    struct TestConfig {
      #[config(env = "TEST_HOST")]
      host: String,
    }
    
    env::set_var("TEST_HOST", "override.example.com");
    
    let config = TestConfig::default()
      .load_from_env()
      .unwrap();
      
    assert_eq!(config.host, "override.example.com");
    
    env::remove_var("TEST_HOST");
  }
  
  #[test]
  fn test_profile_selection() {
    let config_content = r#"
      [default]
      debug = false
      
      [development]
      debug = true
    "#;
    std::fs::write("test_profile.toml", config_content).unwrap();
    
    #[derive(ComponentModel, Config)]
    struct TestConfig {
      debug: bool,
    }
    
    let config = TestConfig::from_config_file("test_profile.toml")
      .with_profile("development")
      .build()
      .unwrap();
      
    assert_eq!(config.debug, true);
    
    std::fs::remove_file("test_profile.toml").unwrap();
  }
}
```

### **Integration Tests**
```rust
// tests/config_integration.rs
#[test]
fn test_real_world_config() {
  let config_toml = r#"
    [database]
    host = "localhost"
    port = 5432
    
    [server]
    bind_addr = "127.0.0.1:8080"
    workers = 4
    
    [production]
    [production.database]
    host = "prod-db.example.com"
    
    [production.server]
    workers = 16
  "#;
  
  #[derive(ComponentModel, Config)]
  struct DatabaseConfig {
    host: String,
    port: u16,
  }
  
  #[derive(ComponentModel, Config)]
  struct ServerConfig {
    bind_addr: String,
    workers: usize,
  }
  
  #[derive(ComponentModel, Config)]
  struct AppConfig {
    #[config(nested)]
    database: DatabaseConfig,
    
    #[config(nested)]
    server: ServerConfig,
  }
  
  std::fs::write("app_test.toml", config_toml).unwrap();
  
  // Test default profile
  let config = AppConfig::from_toml("app_test.toml").unwrap();
  assert_eq!(config.database.host, "localhost");
  assert_eq!(config.server.workers, 4);
  
  // Test production profile
  let config = AppConfig::from_config_file("app_test.toml")
    .with_profile("production")
    .build()
    .unwrap();
    
  assert_eq!(config.database.host, "prod-db.example.com");
  assert_eq!(config.server.workers, 16);
  
  std::fs::remove_file("app_test.toml").unwrap();
}
```

## 📊 **Success Metrics**

- [ ] Support for TOML, YAML, JSON configuration formats
- [ ] Seamless environment variable integration
- [ ] Profile-based configuration
- [ ] Integration with `config` crate
- [ ] Zero-overhead when features not used
- [ ] Clear error messages for configuration issues

## 🚧 **Potential Challenges**

1. **Format Compatibility**: Different formats have different capabilities
   - **Solution**: Common denominator approach with format-specific extensions

2. **Environment Variable Mapping**: Complex nested structures
   - **Solution**: Flattened dot-notation mapping with clear documentation

3. **Profile Merging**: Complex merge semantics
   - **Solution**: Clear precedence rules and merge strategy documentation

## 🔄 **Dependencies**

- **Requires**: 
  - Task 001 (Single Derive Macro) for attribute parsing
  - Task 003 (Validation) for config validation
- **Blocks**: None  
- **Related**: Task 002 (Popular Types) benefits from config loading

## 📅 **Timeline**

- **Week 1**: Core file loading and environment variables
- **Week 2**: Profiles, builder pattern, and config crate integration
- **Week 3**: Documentation, examples, and optimization

## 💡 **Future Enhancements**

- **Hot Reload**: Watch config files for changes
- **Remote Configuration**: Load from HTTP endpoints, databases
- **Configuration Schemas**: Generate JSON schemas from structs
- **Configuration UI**: Generate web UIs for configuration editing