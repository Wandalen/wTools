# Task 008: Advanced Type System - Enum Support

## 🎯 **Objective**

Extend component model to support enum types with variant-specific component assignment, enabling type-safe configuration for different modes, states, and union-like data structures.

## 📋 **Current State**

Component model only works with structs:
```rust
#[derive(ComponentModel)]
struct Config {
  mode: String,  // "development" | "production" | "testing"
  database: String, // Could be different for each mode
}

// Must handle enum logic manually
let config = Config::default()
  .impute("production")
  .impute("postgres://prod-db:5432/app");

// Manual validation required
if config.mode == "production" && !config.database.starts_with("postgres://") {
  panic!("Production requires PostgreSQL");
}
```

## 🎯 **Target State**

Native enum support with variant-specific components:
```rust
#[derive(ComponentModel)]
enum DatabaseConfig {
  #[component(default)]
  Development {
    #[component(default = "localhost")]
    host: String,
    #[component(default = "5432")]
    port: u16,
  },
  
  Production {
    #[component(validate = "is_secure_connection")]
    connection_string: String,
    #[component(default = "50")]
    pool_size: usize,
  },
  
  InMemory,
}

// Type-safe variant assignment
let db_config = DatabaseConfig::Development::default()
  .impute("dev-db.local")
  .impute(5433u16);

// Or assign to existing enum
let mut config = DatabaseConfig::InMemory;
config.assign_variant(DatabaseConfig::Production {
  connection_string: "".to_string(),
  pool_size: 0,
});
config.assign("postgres://secure:pass@prod-db:5432/app");
config.assign(100usize);
```

## 📝 **Detailed Requirements**

### **Core Enum Traits**

#### **EnumAssign Trait**
```rust
pub trait EnumAssign<T, IntoT> {
  type Error;
  
  fn assign_to_variant(&mut self, component: IntoT) -> Result<(), Self::Error>;
  fn impute_to_variant(self, component: IntoT) -> Result<Self, Self::Error>
  where
    Self: Sized;
}

pub trait VariantAssign<V, T, IntoT> {
  type Error;
  
  fn assign_to_variant(&mut self, variant: V, component: IntoT) -> Result<(), Self::Error>;
  fn switch_to_variant(self, variant: V) -> Self;
}
```

#### **Variant Construction**
```rust
pub trait VariantConstructor<T> {
  fn construct_variant(components: T) -> Self;
  fn variant_name(&self) -> &'static str;
  fn variant_fields(&self) -> Vec<(&'static str, &'static str)>; // (field_name, type_name)
}
```

### **Enum Derive Implementation**

#### **Simple Enum (Unit Variants)**
```rust
#[derive(ComponentModel)]
enum LogLevel {
  Debug,
  Info, 
  Warn,
  Error,
}

// Generates string-based assignment
impl Assign<LogLevel, &str> for LogLevel {
  fn assign(&mut self, component: &str) -> Result<(), ComponentError> {
    *self = match component.to_lowercase().as_str() {
      "debug" => LogLevel::Debug,
      "info" => LogLevel::Info,
      "warn" => LogLevel::Warn,
      "error" => LogLevel::Error,
      _ => return Err(ComponentError::InvalidVariant {
        provided: component.to_string(),
        expected: vec!["debug", "info", "warn", "error"],
      }),
    };
    Ok(())
  }
}

// Usage
let mut level = LogLevel::Info;
level.assign("debug").unwrap();
assert!(matches!(level, LogLevel::Debug));
```

#### **Complex Enum (Struct Variants)**
```rust
#[derive(ComponentModel)]
enum ServerMode {
  Development {
    #[component(default = "127.0.0.1")]
    host: String,
    #[component(default = "8080")]
    port: u16,
    #[component(default = "true")]
    hot_reload: bool,
  },
  
  Production {
    #[component(validate = "is_secure_host")]
    host: String,
    #[component(validate = "is_secure_port")]
    port: u16,
    #[component(default = "100")]
    max_connections: usize,
  },
  
  Testing {
    #[component(default = "test")]
    database: String,
  },
}

// Generated variant constructors
impl ServerMode {
  pub fn development() -> Self {
    Self::Development {
      host: "127.0.0.1".to_string(),
      port: 8080,
      hot_reload: true,
    }
  }
  
  pub fn production() -> Self {
    Self::Production {
      host: "".to_string(),
      port: 0,
      max_connections: 100,
    }
  }
  
  pub fn testing() -> Self {
    Self::Testing {
      database: "test".to_string(),
    }
  }
}

// Generated component assignment
impl EnumAssign<String, &str> for ServerMode {
  type Error = ComponentError;
  
  fn assign_to_variant(&mut self, component: &str) -> Result<(), Self::Error> {
    match self {
      Self::Development { host, .. } => {
        *host = component.to_string();
        Ok(())
      },
      Self::Production { host, .. } => {
        is_secure_host(component)?;
        *host = component.to_string();
        Ok(())
      },
      Self::Testing { .. } => {
        Err(ComponentError::IncompatibleVariant {
          variant: "Testing",
          component_type: "String",
        })
      },
    }
  }
}

impl EnumAssign<u16, u16> for ServerMode {
  type Error = ComponentError;
  
  fn assign_to_variant(&mut self, component: u16) -> Result<(), Self::Error> {
    match self {
      Self::Development { port, .. } => {
        *port = component;
        Ok(())
      },
      Self::Production { port, .. } => {
        is_secure_port(component)?;
        *port = component;
        Ok(())
      },
      Self::Testing { .. } => {
        Err(ComponentError::IncompatibleVariant {
          variant: "Testing", 
          component_type: "u16",
        })
      },
    }
  }
}
```

### **Variant Switching and Migration**

#### **Safe Variant Switching**
```rust
impl ServerMode {
  pub fn switch_to_development(self) -> Self {
    match self {
      Self::Development { .. } => self, // Already correct variant
      Self::Production { host, .. } => {
        // Migrate from production to development
        Self::Development {
          host: if host.is_empty() { "127.0.0.1".to_string() } else { host },
          port: 8080,
          hot_reload: true,
        }
      },
      Self::Testing { .. } => {
        // Default development config
        Self::development()
      },
    }
  }
  
  pub fn try_switch_to_production(self) -> Result<Self, ValidationError> {
    match self {
      Self::Production { .. } => Ok(self),
      Self::Development { host, port, .. } => {
        // Validate before switching
        is_secure_host(&host)?;
        is_secure_port(port)?;
        
        Ok(Self::Production {
          host,
          port,
          max_connections: 100,
        })
      },
      Self::Testing { .. } => {
        Err(ValidationError::InvalidTransition {
          from: "Testing",
          to: "Production",
          reason: "Cannot migrate test config to production".to_string(),
        })
      },
    }
  }
}
```

### **Pattern Matching Integration**

#### **Component Query by Variant**
```rust
impl ServerMode {
  pub fn get_host(&self) -> Option<&str> {
    match self {
      Self::Development { host, .. } | Self::Production { host, .. } => Some(host),
      Self::Testing { .. } => None,
    }
  }
  
  pub fn get_port(&self) -> Option<u16> {
    match self {
      Self::Development { port, .. } | Self::Production { port, .. } => Some(*port),
      Self::Testing { .. } => None,
    }
  }
  
  pub fn supports_component<T: ComponentType>(&self) -> bool {
    match (T::type_name(), self.variant_name()) {
      ("String", "Development") => true,
      ("String", "Production") => true, 
      ("u16", "Development") => true,
      ("u16", "Production") => true,
      ("bool", "Development") => true,
      ("usize", "Production") => true,
      ("String", "Testing") => true, // database field
      _ => false,
    }
  }
}
```

### **Advanced Enum Patterns**

#### **Nested Enums**
```rust
#[derive(ComponentModel)]
enum DatabaseType {
  Postgres {
    #[component(nested)]
    connection: PostgresConfig,
  },
  Mysql {
    #[component(nested)]
    connection: MysqlConfig,
  },
  Sqlite {
    #[component(validate = "file_exists")]
    file_path: PathBuf,
  },
}

#[derive(ComponentModel)]
struct PostgresConfig {
  host: String,
  port: u16,
  sslmode: String,
}
```

#### **Generic Enum Support**
```rust
#[derive(ComponentModel)]
enum Result<T, E> {
  Ok(T),
  Err(E),
}

#[derive(ComponentModel)]
enum Option<T> {
  Some(T),
  None,
}

// Usage with component assignment
let mut result: Result<String, String> = Result::Ok("".to_string());
result.assign_to_variant("success_value".to_string()); // Assigns to Ok variant

let mut option: Option<i32> = Option::None;
option.assign_to_variant(42); // Changes to Some(42)
```

### **Union-Type Support**

#### **Either Pattern**
```rust
#[derive(ComponentModel)]
enum Either<L, R> {
  Left(L),
  Right(R),
}

impl<L, R, T> Assign<Either<L, R>, T> for Either<L, R>
where
  T: TryInto<L> + TryInto<R>,
{
  fn assign(&mut self, component: T) {
    // Try left first, then right
    if let Ok(left_val) = component.try_into() {
      *self = Either::Left(left_val);
    } else if let Ok(right_val) = component.try_into() {
      *self = Either::Right(right_val);
    }
    // Could implement priority or explicit variant selection
  }
}
```

## 🗂️ **File Changes**

### **New Files**
- `component_model_meta/src/enum_derive.rs` - Enum derive implementation
- `component_model_types/src/enum_traits.rs` - Enum-specific traits
- `component_model_types/src/variant.rs` - Variant handling utilities
- `component_model_types/src/pattern_match.rs` - Pattern matching helpers
- `examples/enum_config_example.rs` - Comprehensive enum examples
- `examples/state_machine_example.rs` - State machine with enums

### **Modified Files**
- `component_model_meta/src/lib.rs` - Export enum derive
- `component_model_types/src/lib.rs` - Export enum traits
- `component_model/src/lib.rs` - Re-export enum functionality

## ⚡ **Implementation Steps**

### **Phase 1: Basic Enum Support (Week 1)**
1. Implement simple enum derive (unit variants only)
2. Add string-based variant assignment
3. Create basic error types for enum operations
4. Unit tests for simple enums

### **Phase 2: Struct Variants (Week 2)**
1. Add support for struct-like enum variants
2. Implement field-level component assignment within variants
3. Add variant switching and migration
4. Validation integration for enum fields

### **Phase 3: Advanced Features (Week 2-3)**
1. Generic enum support
2. Nested enums and complex patterns
3. Pattern matching helpers and utilities
4. Performance optimization and comprehensive testing

## 🧪 **Testing Strategy**

### **Unit Tests**
```rust
#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_simple_enum_assignment() {
    #[derive(ComponentModel, PartialEq, Debug)]
    enum Color {
      Red,
      Green,
      Blue,
    }
    
    let mut color = Color::Red;
    color.assign("green").unwrap();
    assert_eq!(color, Color::Green);
    
    assert!(color.assign("purple").is_err());
  }
  
  #[test]
  fn test_struct_variant_assignment() {
    #[derive(ComponentModel)]
    enum ServerConfig {
      Development { host: String, port: u16 },
      Production { host: String, port: u16, ssl: bool },
    }
    
    let mut config = ServerConfig::Development {
      host: "localhost".to_string(),
      port: 8080,
    };
    
    config.assign_to_variant("api.example.com").unwrap();
    config.assign_to_variant(3000u16).unwrap();
    
    match config {
      ServerConfig::Development { host, port } => {
        assert_eq!(host, "api.example.com");
        assert_eq!(port, 3000);
      },
      _ => panic!("Wrong variant"),
    }
  }
  
  #[test]
  fn test_variant_switching() {
    #[derive(ComponentModel)]
    enum Mode {
      Dev { debug: bool },
      Prod { optimized: bool },
    }
    
    let dev_mode = Mode::Dev { debug: true };
    let prod_mode = dev_mode.switch_to_variant(Mode::Prod { optimized: false });
    
    match prod_mode {
      Mode::Prod { optimized } => assert!(!optimized),
      _ => panic!("Failed to switch variant"),
    }
  }
}
```

### **Integration Tests**
```rust
// tests/enum_integration.rs
#[test]
fn test_complex_enum_config() {
  #[derive(ComponentModel)]
  enum AppEnvironment {
    Development {
      #[component(default = "localhost")]
      db_host: String,
      #[component(default = "3000")]
      port: u16,
      #[component(default = "true")]
      hot_reload: bool,
    },
    
    Production {
      #[component(validate = "is_production_db")]
      db_connection_string: String,
      #[component(validate = "is_https_port")]
      port: u16,
      #[component(default = "1000")]
      max_connections: usize,
    },
  }
  
  // Test development configuration
  let mut dev_config = AppEnvironment::Development {
    db_host: "".to_string(),
    port: 0,
    hot_reload: false,
  };
  
  dev_config.assign_to_variant("dev-db.local").unwrap();
  dev_config.assign_to_variant(4000u16).unwrap();
  dev_config.assign_to_variant(true).unwrap();
  
  // Test migration to production
  let prod_config = dev_config.try_switch_to_production().unwrap();
  
  match prod_config {
    AppEnvironment::Production { port, max_connections, .. } => {
      assert_eq!(port, 443); // Should validate and use HTTPS port
      assert_eq!(max_connections, 1000);
    },
    _ => panic!("Migration failed"),
  }
}
```

## 📊 **Success Metrics**

- [ ] Support for unit, tuple, and struct enum variants  
- [ ] Type-safe component assignment within variants
- [ ] Variant switching with validation and migration
- [ ] Generic enum support (Option<T>, Result<T,E>, Either<L,R>)
- [ ] Clear error messages for invalid variant operations
- [ ] Zero runtime overhead vs manual enum handling

## 🚧 **Potential Challenges**

1. **Type Complexity**: Generic enums with complex constraints
   - **Solution**: Careful trait bounds and incremental implementation

2. **Pattern Matching**: Generating efficient match statements
   - **Solution**: Optimize generated code and benchmark performance

3. **Variant Migration**: Complex data transformations between variants
   - **Solution**: User-defined migration functions and validation

4. **Error Handling**: Clear errors for variant-specific operations
   - **Solution**: Structured error types with context information

## 🔄 **Dependencies**

- **Requires**: 
  - Task 001 (Single Derive Macro) for attribute parsing
  - Task 003 (Validation) for variant validation
- **Blocks**: None
- **Related**: All configuration tasks benefit from enum support

## 📅 **Timeline**

- **Week 1**: Simple enum support (unit variants)
- **Week 2**: Struct variants and field assignment
- **Week 2-3**: Advanced features, generics, and optimization

## 💡 **Future Enhancements**

- **State Machines**: First-class state machine support with transitions
- **Pattern Matching Macros**: Advanced pattern matching helpers
- **Serialization**: Seamless serde integration for enum variants
- **GraphQL Integration**: Generate GraphQL union types from enums
- **Database Mapping**: Map enum variants to database columns/tables