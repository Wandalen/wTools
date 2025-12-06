# config_hierarchy

A robust, production-ready hierarchical configuration management system for Rust applications featuring a 6-level priority resolution system, automatic source tracking, and cross-platform file operations with atomic writes.

## Overview

`config_hierarchy` provides a flexible framework for managing application configuration across multiple sources with clear precedence rules. It eliminates configuration ambiguity by tracking the exact source of every configuration value and supports multiple storage formats and output representations.

**Key Capabilities:**
- **Priority-based resolution** across 6 configuration layers
- **Source tracking** for every configuration value
- **Atomic file operations** with cross-platform file locking
- **Multiple formats** for input (YAML/JSON/TOML) and output (table/JSON/YAML)
- **Type detection** with intelligent string-to-type conversion
- **Trait-based architecture** for application-specific customization

## Design Principles

This crate focuses exclusively on configuration management concerns:

**Core Responsibilities:**
- Hierarchical configuration resolution and merging
- Configuration persistence and retrieval
- Source tracking and validation
- Format conversion and display

**Intentional Non-Goals:**
- Application-specific configuration logic (delegated to consuming applications)
- CLI argument parsing (handled by application layer)
- External tool integration (managed by specialized crates)
- Domain-specific validation rules (implemented via traits by consumers)

## Installation

Add `config_hierarchy` to your project's `Cargo.toml`:

```toml
[dependencies]
config_hierarchy = { version = "0.2", features = ["full"] }
```

### Feature Flags

The crate provides granular feature flags for selective capability inclusion:

| Feature | Description | Includes |
|---------|-------------|----------|
| `default` | Core resolution engine | Configuration hierarchy, source tracking, type detection |
| `file_ops` | File persistence layer | YAML I/O, atomic writes, file locking, path discovery |
| `display_table` | Table output formatter | Tabular configuration display with tree formatting |
| `display_json` | JSON output formatter | JSON serialization for configuration data |
| `display_yaml` | YAML output formatter | YAML serialization for configuration data |
| `full` | All features enabled | Complete functionality for production use |

**Recommendation:** Use `full` for applications unless binary size is a critical constraint.

## Quick Start

Create a configuration manager for your application by implementing three core traits:

```rust
use config_hierarchy::{ ConfigManager, ConfigDefaults, ConfigPaths, ConfigValidator };
use std::collections::HashMap;
use serde_json::Value as JsonValue;

// Define default configuration values
struct AppDefaults;
impl ConfigDefaults for AppDefaults
{
  fn get_defaults() -> HashMap< String, JsonValue >
  {
    let mut map = HashMap::new();
    map.insert( "timeout".into(), JsonValue::Number( 30.into() ) );
    map.insert( "retries".into(), JsonValue::Number( 3.into() ) );
    map
  }

  fn get_parameter_names() -> Vec< &'static str >
  {
    vec![ "timeout", "retries" ]
  }
}

// Define configuration file paths (automatically derives all paths from app name)
struct AppPaths;
impl ConfigPaths for AppPaths
{
  fn app_name() -> &'static str { "myapp" }
}

// Define validation rules
struct AppValidator;
impl ConfigValidator for AppValidator
{
  fn validate_parameter( _name : &str, _value : &JsonValue )
    -> Result< (), config_hierarchy::ValidationError >
  {
    Ok( () )
  }

  fn validate_all( _config : &HashMap< String, ( JsonValue, config_hierarchy::ConfigSource ) > )
    -> Vec< config_hierarchy::ValidationError >
  {
    Vec::new()
  }
}

// Compose the configuration manager from the three traits
type AppConfig = ConfigManager< AppDefaults, AppPaths, AppValidator >;

fn main()
{
  // Resolve configuration from all sources
  let runtime_overrides = HashMap::new();
  let config = AppConfig::resolve_all_config( &runtime_overrides );

  // Display resolved configuration with source tracking
  for ( key, ( value, source ) ) in &config
  {
    println!( "{key}: {value:?} (from {source:?})" );
  }
}
```

**Output Example:**
```text
timeout: Number(30) (from Default)
retries: Number(3) (from Default)
```

If environment variable `MYAPP_TIMEOUT=60` is set:
```text
timeout: Number(60) (from Environment)
retries: Number(3) (from Default)
```

## Configuration Hierarchy

The crate implements a 6-level priority system where higher priority sources override lower priority ones. Configuration values are resolved by searching through layers in order until a value is found:

### Priority Levels (Highest to Lowest)

**1. Runtime Parameters**
- Explicitly provided at program execution
- Passed directly to `resolve_all_config()`
- Use case: Command-line overrides, testing, dynamic configuration

**2. Environment Variables**
- Format: `{APP_NAME}_{PARAMETER}` (uppercase with underscores)
- Example: `MYAPP_TIMEOUT=60`
- Use case: Container deployments, CI/CD pipelines, system-level configuration

**3. Local Configuration (Current Directory)**
- Temporary: `-./.{app_name}/config.yaml` (gitignored)
- Permanent: `./.{app_name}/config.yaml` (tracked in VCS)
- Temporary configs override permanent when both exist
- Use case: Project-specific settings, developer preferences

**4. Local Configuration (Parent Directories)**
- Searches ancestor directories from current to root
- Nearest configuration takes precedence
- Use case: Workspace-wide settings, monorepo configuration

**5. Global Configuration**
- Location: `$PRO/.persistent/.{app_name}/config.yaml`
- Requires `PRO` environment variable
- Use case: User-wide defaults, machine-specific settings

**6. Default Values**
- Defined in `ConfigDefaults` trait implementation
- Always available as final fallback
- Use case: Application baseline configuration

### Source Tracking

Every resolved value includes its source, enabling debugging and auditing:

```rust,no_run
# use config_hierarchy::{ ConfigManager, ConfigDefaults, ConfigPaths, ConfigValidator, ConfigSource };
# use std::collections::HashMap;
# use serde_json::Value as JsonValue;
# struct AppDefaults;
# impl ConfigDefaults for AppDefaults {
#   fn get_defaults() -> HashMap<String, JsonValue> { HashMap::new() }
#   fn get_parameter_names() -> Vec<&'static str> { vec![] }
# }
# struct AppPaths;
# impl ConfigPaths for AppPaths { fn app_name() -> &'static str { "myapp" } }
# struct AppValidator;
# impl ConfigValidator for AppValidator {
#   fn validate_parameter(_: &str, _: &JsonValue) -> Result<(), config_hierarchy::ValidationError> { Ok(()) }
#   fn validate_all(_: &HashMap<String, (JsonValue, ConfigSource)>) -> Vec<config_hierarchy::ValidationError> { Vec::new() }
# }
# type AppConfig = ConfigManager<AppDefaults, AppPaths, AppValidator>;
# let config = AppConfig::resolve_all_config( &HashMap::new() );
for ( key, ( value, source ) ) in &config
{
  match source
  {
    ConfigSource::Runtime => println!( "{key} overridden at runtime" ),
    ConfigSource::Environment => println!( "{key} from environment" ),
    ConfigSource::LocalCurrent( path ) => println!( "{key} from local config: {}", path.display() ),
    ConfigSource::LocalParent( path ) => println!( "{key} from parent config: {}", path.display() ),
    ConfigSource::Global( path ) => println!( "{key} from global config: {}", path.display() ),
    ConfigSource::Default => println!( "{key} using default" ),
  }
}
```

## Path Conventions

All configuration paths follow a consistent naming scheme derived from the application name:

### Path Patterns

| Scope | Type | Pattern | Git Tracking |
|-------|------|---------|--------------|
| Local | Temporary | `-./.{app_name}/config.yaml` | Ignored (hyphen prefix) |
| Local | Permanent | `./.{app_name}/config.yaml` | Tracked (dot prefix) |
| Global | Persistent | `$PRO/.persistent/.{app_name}/config.yaml` | N/A |

### Dual Local Configuration Pattern

The crate supports both temporary and permanent local configurations:

- **Temporary configs** (`-./`) are developer-specific and gitignored
- **Permanent configs** (`./`) are project-specific and version-controlled
- When both exist, temporary takes precedence
- Use temporary for local experiments, permanent for team defaults

### Automatic Path Derivation

All paths are automatically derived from `ConfigPaths::app_name()`:

```rust,no_run
# use config_hierarchy::ConfigPaths;
struct MyApp;

impl ConfigPaths for MyApp
{
  fn app_name() -> &'static str { "myapp" }
}

// Automatically generates:
// - Local: ./.myapp/config.yaml or -./.myapp/config.yaml
// - Global: $PRO/.persistent/.myapp/config.yaml
```

**Security Requirements:**

The `app_name()` method is validated at runtime to prevent security vulnerabilities:
- Must not be empty (prevents invalid paths like `./config.yaml`)
- Must not contain path separators `/` or `\` (prevents directory traversal)
- Must not contain parent directory references `..` (prevents path traversal attacks)

Recommended: Use alphanumeric characters, hyphens, underscores (`my-app`, `my_app123`). Unicode is supported but whitespace should be avoided.

## File Operations

### Atomic Writes with File Locking

When the `file_ops` feature is enabled, all file operations use atomic writes with cross-platform file locking:

```rust,no_run
# use config_hierarchy::atomic_config_modify;
# use std::path::Path;
# use serde_json::Value as JsonValue;
# fn example() -> Result< (), String > {
// Safely modify configuration with exclusive lock
atomic_config_modify( Path::new( "./.myapp/config.yaml" ), | config |
{
  config.insert( "timeout".into(), JsonValue::Number( 60.into() ) );
  Ok( () )
} )?;
# Ok( () )
# }
```

**Concurrency Guarantees:**
- Exclusive file locking prevents concurrent write conflicts
- Uses `fs2` crate for cross-platform advisory locks
- Works on Linux, macOS, and Windows
- Preserves metadata (created_at timestamp) during updates

### Metadata Preservation

Configuration files include automatic metadata tracking:

```yaml
metadata:
  version: "1.0"
  created_at: "2025-01-15T10:30:00Z"
  last_modified: "2025-01-15T14:45:00Z"
parameters:
  timeout: 60
  retries: 3
```

## Advanced Usage

### Multiple Output Formats

Display configuration in different formats for different use cases:

```rust,no_run
# #[ cfg( all( feature = "display_table", feature = "display_json", feature = "display_yaml" ) ) ]
# {
# use config_hierarchy::{ ConfigManager, ConfigDefaults, ConfigPaths, ConfigValidator, ConfigSource };
# use config_hierarchy::display::table::format_config_table;
# use config_hierarchy::display::json::format_config_json;
# use config_hierarchy::display::yaml::format_config_yaml;
# use config_hierarchy::display::DisplayOptions;
# use std::collections::HashMap;
# use serde_json::Value as JsonValue;
# struct AppDefaults;
# impl ConfigDefaults for AppDefaults {
#   fn get_defaults() -> HashMap<String, JsonValue> { HashMap::new() }
#   fn get_parameter_names() -> Vec<&'static str> { vec![] }
# }
# struct AppPaths;
# impl ConfigPaths for AppPaths { fn app_name() -> &'static str { "myapp" } }
# struct AppValidator;
# impl ConfigValidator for AppValidator {
#   fn validate_parameter(_: &str, _: &JsonValue) -> Result<(), config_hierarchy::ValidationError> { Ok(()) }
#   fn validate_all(_: &HashMap<String, (JsonValue, ConfigSource)>) -> Vec<config_hierarchy::ValidationError> { Vec::new() }
# }
# type AppConfig = ConfigManager<AppDefaults, AppPaths, AppValidator>;
let config = AppConfig::resolve_all_config( &HashMap::new() );
let options = DisplayOptions::default();

// Table format for CLI display
println!( "{}", format_config_table::< AppDefaults, AppPaths >( &config, &[], &options ) );

// JSON for programmatic consumption
println!( "{}", format_config_json( &config, &[], &options ) );

// YAML for file export
println!( "{}", format_config_yaml( &config, &[], &options ) );
# }
```

### Type Detection

Configuration values are automatically typed from strings:

- `"123"` → `Number(123)`
- `"true"` / `"false"` → `Bool(true/false)`
- `"3.14"` → `Number(3.14)`
- `"hello"` → `String("hello")`

Override with environment variables:
```bash
MYAPP_TIMEOUT=60        # Becomes Number(60)
MYAPP_ENABLED=true      # Becomes Bool(true)
MYAPP_NAME=production   # Becomes String("production")
```

### Path Customization

The `ConfigPaths` trait provides 14 optional methods for customizing paths and naming conventions. Override any method to change defaults:

```rust
use config_hierarchy::{ ConfigPaths, EnvVarCasing };

struct CustomPaths;

impl ConfigPaths for CustomPaths
{
  fn app_name() -> &'static str { "myapp" }

  // Customize local paths
  fn local_permanent_prefix() -> &'static str { "_" }
  fn local_config_filename() -> &'static str { "settings.toml" }

  // Customize environment variables
  fn env_var_prefix() -> &'static str { "MY_CUSTOM_APP" }
  fn env_var_separator() -> &'static str { "__" }
  fn env_var_casing() -> EnvVarCasing { EnvVarCasing::LowerCase }

  // Customize global paths
  fn global_persistent_dir() -> &'static str { ".config" }
  fn global_config_filename() -> &'static str { "app.yaml" }
}
```

**Results**:
- Local config: `./_myapp/settings.toml`
- Global config: `$PRO/.config/.myapp/app.yaml`
- Environment variables: `my_custom_app__timeout`, `my_custom_app__debug`

**Available Customization Methods**:

| Category | Methods | Default |
|----------|---------|---------|
| Environment Variables | `env_var_prefix()` | `app_name().to_uppercase()` |
| | `env_var_separator()` | `"_"` |
| | `env_var_casing()` | `EnvVarCasing::UpperCase` |
| Local Paths | `local_permanent_prefix()` | `"."` |
| | `local_temporary_prefix()` | `"-"` |
| | `local_config_filename()` | `"config.yaml"` |
| Global Paths | `global_persistent_dir()` | `".persistent"` |
| | `global_config_filename()` | `"config.yaml"` |
| Environment Names | `pro_env_var()`, `home_env_var()` | `"PRO"`, `"HOME"` |
| | `xdg_config_home_var()`, `appdata_var()` | Platform-specific |
| OS-Specific Bases | `linux_config_base()` | `".config"` |
| | `macos_config_base()` | `"Library/Application Support"` |

See `ConfigPaths` trait documentation for complete details.

## Testing

Run the complete test suite:

```bash
# Unit and integration tests
cargo nextest run --all-features

# Documentation tests
cargo test --doc --all-features

# Clippy lints
cargo clippy --all-targets --all-features
```

Test coverage includes:
- Hierarchical resolution across all 6 layers
- Concurrent file access with locking
- Cross-platform path handling
- Type detection and conversion
- Display formatters

## License

MIT
