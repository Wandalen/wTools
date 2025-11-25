# Specification: workspace_tools

## Overview

**workspace_tools** is a workspace-relative path resolution and configuration management utility providing reliable, context-independent path handling for Rust projects. It automatically detects workspace roots through multiple fallback strategies, offers standardized directory structures, and provides feature-gated capabilities for configuration loading, secret management, and resource discovery.

**Version:** 0.10.0
**Status:** Production-Ready
**Category:** Filesystem / Development Tools
**Dependents:** Workspace projects requiring consistent path resolution

### Scope

#### Responsibility

Provide a `Workspace` struct with methods for resolving workspace-relative paths, accessing standard directories (config, data, logs, docs, tests), loading configuration files in multiple formats (TOML/JSON/YAML), managing secrets with memory-safe handling, discovering resources via glob patterns, and validating configurations against JSON schemas.

#### In-Scope

1. **Workspace Root Resolution**
   - `workspace()` - Auto-detect workspace root
   - `Workspace::new(path)` - Explicit path creation
   - `Workspace::from_cargo_workspace()` - Cargo metadata detection
   - Multiple fallback strategies (cargo, env, git, $PRO, $HOME, cwd)
   - Path normalization (trailing `/./` removal, absolute paths)
   - `WORKSPACE_PATH` environment variable support

2. **Path Operations**
   - `root()` - Get workspace root path
   - `join(relative)` - Join paths safely
   - Path boundary checking (PathOutsideWorkspace error)
   - Consistent behavior across execution contexts

3. **Standard Directory Access**
   - `config_dir()` - ./config/ directory
   - `data_dir()` - ./data/ directory
   - `logs_dir()` - ./logs/ directory
   - `docs_dir()` - ./docs/ directory
   - `tests_dir()` - ./tests/ directory
   - Standardized project structure

4. **Configuration Loading (serde feature)**
   - `load_config(name)` - Load by name (.toml/.json/.yaml)
   - `load_config_from(path)` - Load from specific path
   - `load_config_layered([...])` - Merge multiple configs
   - `find_config(name)` - Find config with priority ordering
   - Format detection from extension
   - Serde deserialization into structs

5. **Secret Management (secrets feature)**
   - `load_secrets_from_file(file)` - Load secrets map
   - `load_secret_key(key, file)` - Load single secret
   - `secret/` directory convention
   - Supports KEY=VALUE and export KEY=VALUE formats
   - Environment variable fallback (`env_secret()`)

6. **Memory-Safe Secrets (secure feature)**
   - `load_secrets_secure(file)` - SecretString wrapped
   - `load_secret_key_secure(key, file)` - Single SecretString
   - `AsSecure` trait for type conversion
   - `SecretInjectable` trait for config injection
   - `validate_secret()` - Password strength validation
   - Zeroization on drop
   - Debug output redaction

7. **Resource Discovery (glob feature)**
   - `find_resources(pattern)` - Glob pattern matching
   - `src/**/*.rs` style patterns
   - Returns Vec<PathBuf>

8. **Configuration Validation (validation feature)**
   - `load_config_with_validation(name)` - Schema validation
   - JSON Schema support
   - Detailed validation errors
   - `schemars` integration for schema generation

9. **Testing Support (testing feature)**
   - `create_test_workspace_with_structure()` - Temp workspace
   - Automatic cleanup
   - Isolated test environments

10. **Error Handling**
    - `WorkspaceError` enum
    - Feature-gated variants
    - Display implementation with helpful messages
    - `Result<T>` type alias

#### Out-of-Scope

1. **NOT Build System**
   - No compilation
   - No dependency management
   - **Rationale:** Use cargo for builds

2. **NOT Version Control**
   - No git operations (beyond detection)
   - No commit/branch management
   - **Rationale:** Use git directly

3. **NOT Crate Management**
   - No Cargo.toml manipulation
   - No publishing
   - **Rationale:** Use cargo or crates_tools

4. **NOT Template Engine**
   - No Mustache/Handlebars templates
   - Basic secret injection only
   - **Rationale:** Keep focused

5. **NOT Watch/Hot-Reload**
   - No file watching
   - No automatic reloading
   - **Rationale:** Use notify crate

6. **NOT Encryption**
   - No file encryption
   - Memory protection only
   - **Rationale:** Use dedicated crypto crates

7. **NOT Cloud Integration**
   - No AWS/GCP secrets
   - Local files only
   - **Rationale:** Use cloud SDKs

8. **NOT Database**
   - No structured storage
   - File-based only
   - **Rationale:** Use database crates

#### Boundaries

- **workspace_tools vs pth**: workspace_tools for workspace paths; pth for general paths
- **workspace_tools vs config crate**: workspace_tools workspace-aware; config general-purpose
- **workspace_tools vs dotenv**: workspace_tools workspace-relative; dotenv cwd-relative

## Architecture

### Dependency Structure

```
workspace_tools
├── Core Dependencies (always available)
│   ├── cargo_metadata (workspace) - Cargo workspace detection
│   └── toml (workspace, preserve_order) - TOML parsing
├── Optional Dependencies
│   ├── glob (workspace, glob feature) - Pattern matching
│   ├── tempfile (workspace, testing feature) - Temp directories
│   ├── serde (workspace, derive, serde feature) - Serialization
│   ├── serde_json (workspace, serde feature) - JSON support
│   ├── serde_yaml (workspace, serde feature) - YAML support
│   ├── jsonschema (0.20, validation feature) - JSON Schema
│   ├── schemars (0.8, validation feature) - Schema generation
│   ├── secrecy (0.8, secure feature) - Memory-safe secrets
│   └── zeroize (1.7, secure feature) - Memory zeroing
└── Dev Dependencies
    └── tempfile (workspace) - Testing
```

### Module Organization

```
workspace_tools
├── lib.rs (single-file implementation)
│   ├── WorkspaceError enum - Error types
│   ├── Result<T> type alias
│   ├── SecretInjectable trait (secure feature)
│   ├── AsSecure trait (secure feature)
│   ├── Workspace struct - Main API
│   │   ├── Creation methods (new, workspace, from_cargo_workspace)
│   │   ├── Path methods (root, join, config_dir, data_dir, etc.)
│   │   ├── Config methods (load_config, load_config_layered, etc.)
│   │   ├── Secret methods (load_secrets, load_secret_key, etc.)
│   │   ├── Discovery methods (find_resources, find_config)
│   │   └── Validation methods (load_config_with_validation)
│   └── Internal helpers
│       ├── detect_format() - File format detection
│       ├── read_file_to_string() - Consistent file reading
│       ├── parse_content() - Format-aware parsing
│       └── validate_against_schema() - Schema validation
└── testing module (testing feature)
    └── create_test_workspace_with_structure()
```

### Feature Architecture

```
default = [ enabled, serde ]

enabled (master switch)

serde (configuration loading, default)
├── dep:serde
├── dep:serde_json
└── dep:serde_yaml

glob (resource discovery)
└── dep:glob

secrets (secret file loading)
└── (no additional deps)

secure (memory-safe secrets)
├── secrets (includes secrets feature)
├── dep:secrecy
└── dep:zeroize

validation (config validation)
├── dep:jsonschema
└── dep:schemars

testing (test utilities)
└── dep:tempfile

full = all features
```

**Default Features:** `enabled`, `serde`

### Workspace Resolution Flow

```
workspace() call
  ↓
1. Try Cargo Workspace (cargo_metadata)
   └── Found? → Use workspace root, normalize path
  ↓
2. Try WORKSPACE_PATH env var
   └── Set and non-empty? → Use path, normalize
  ↓
3. Try Git Root (.git + Cargo.toml detection)
   └── Found? → Use git root
  ↓
4. Try $PRO env var (project root)
   └── Set? → Use $PRO
  ↓
5. Try $HOME directory
   └── Exists? → Use $HOME
  ↓
6. Fallback to current directory
```

## Public API

### Workspace Struct

```rust
/// Main workspace handle for path resolution and operations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Workspace {
  root: PathBuf,
}
```

### Creation Methods

```rust
/// Auto-detect workspace root using fallback strategies
pub fn workspace() -> Result<Workspace>;

impl Workspace {
  /// Create from explicit path (normalizes automatically)
  pub fn new(root: impl AsRef<Path>) -> Self;

  /// Create from cargo workspace metadata
  pub fn from_cargo_workspace() -> Result<Self>;
}
```

### Path Methods

```rust
impl Workspace {
  /// Get workspace root path
  pub fn root(&self) -> &Path;

  /// Join relative path to workspace root
  pub fn join(&self, path: impl AsRef<Path>) -> PathBuf;

  /// Standard directory accessors
  pub fn config_dir(&self) -> PathBuf;  // ./config/
  pub fn data_dir(&self) -> PathBuf;    // ./data/
  pub fn logs_dir(&self) -> PathBuf;    // ./logs/
  pub fn docs_dir(&self) -> PathBuf;    // ./docs/
  pub fn tests_dir(&self) -> PathBuf;   // ./tests/
}
```

### Configuration Loading (serde feature)

```rust
impl Workspace {
  /// Load config by name (searches for .toml, .json, .yaml)
  pub fn load_config<T: DeserializeOwned>(&self, name: &str) -> Result<T>;

  /// Load config from specific path
  pub fn load_config_from<T: DeserializeOwned>(&self, path: impl AsRef<Path>) -> Result<T>;

  /// Load and merge multiple configs
  pub fn load_config_layered<T: DeserializeOwned>(&self, names: &[&str]) -> Result<T>;

  /// Find config file path with priority
  pub fn find_config(&self, name: &str) -> Result<PathBuf>;
}
```

### Secret Management (secrets feature)

```rust
impl Workspace {
  /// Load all secrets from file
  pub fn load_secrets_from_file(&self, file: &str) -> Result<HashMap<String, String>>;

  /// Load single secret by key
  pub fn load_secret_key(&self, key: &str, file: &str) -> Result<String>;

  /// Get secret from environment variable
  pub fn env_secret(&self, var: &str) -> Option<String>;
}
```

### Memory-Safe Secrets (secure feature)

```rust
impl Workspace {
  /// Load secrets as SecretString
  pub fn load_secrets_secure(&self, file: &str) -> Result<HashMap<String, SecretString>>;

  /// Load single secret as SecretString
  pub fn load_secret_key_secure(&self, key: &str, file: &str) -> Result<SecretString>;

  /// Validate secret strength
  pub fn validate_secret(&self, secret: &str) -> Result<()>;

  /// Load config with secret injection
  pub fn load_config_with_secrets<T: SecretInjectable>(
    &self,
    config: T,
    secrets_file: &str
  ) -> Result<T>;

  /// Load config with template-based injection
  pub fn load_config_with_secret_injection(
    &self,
    config_file: &str,
    secrets_file: &str
  ) -> Result<String>;
}

/// Trait for automatic secret injection
pub trait SecretInjectable {
  fn inject_secret(&mut self, key: &str, value: String) -> Result<()>;
  fn validate_secrets(&self) -> Result<()>;
}

/// Trait for converting to secure types
pub trait AsSecure {
  type Secure;
  fn into_secure(self) -> Self::Secure;
}
```

### Resource Discovery (glob feature)

```rust
impl Workspace {
  /// Find resources matching glob pattern
  pub fn find_resources(&self, pattern: &str) -> Result<Vec<PathBuf>>;
}
```

### Configuration Validation (validation feature)

```rust
impl Workspace {
  /// Load config with JSON Schema validation
  pub fn load_config_with_validation<T>(&self, name: &str) -> Result<T>
  where
    T: DeserializeOwned + JsonSchema;
}
```

### Error Types

```rust
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum WorkspaceError {
  ConfigurationError(String),
  EnvironmentVariableMissing(String),
  #[cfg(feature = "glob")]
  GlobError(String),
  IoError(String),
  PathNotFound(PathBuf),
  PathOutsideWorkspace(PathBuf),
  CargoError(String),
  TomlError(String),
  #[cfg(feature = "serde")]
  SerdeError(String),
  #[cfg(feature = "validation")]
  ValidationError(String),
  #[cfg(feature = "secure")]
  SecretValidationError(String),
  #[cfg(feature = "secure")]
  SecretInjectionError(String),
}

pub type Result<T> = core::result::Result<T, WorkspaceError>;
```

## Usage Patterns

### Pattern 1: Basic Workspace Access

```rust
use workspace_tools::workspace;

fn main() -> workspace_tools::Result<()> {
  let ws = workspace()?;
  println!("Workspace root: {}", ws.root().display());

  let config_path = ws.config_dir().join("app.toml");
  let data_path = ws.data_dir().join("cache.db");

  Ok(())
}
```

### Pattern 2: Configuration Loading

```rust
use workspace_tools::workspace;
use serde::Deserialize;

#[derive(Deserialize)]
struct AppConfig {
  name: String,
  port: u16,
}

fn main() -> workspace_tools::Result<()> {
  let ws = workspace()?;
  let config: AppConfig = ws.load_config("app")?;
  println!("App: {} on port {}", config.name, config.port);
  Ok(())
}
```

### Pattern 3: Secret Management

```rust
use workspace_tools::workspace;

fn main() -> workspace_tools::Result<()> {
  let ws = workspace()?;

  // Load all secrets
  let secrets = ws.load_secrets_from_file("-secrets.sh")?;

  // Or load specific key
  let api_key = ws.load_secret_key("API_KEY", "-secrets.sh")?;

  println!("API Key loaded");
  Ok(())
}
```

### Pattern 4: Memory-Safe Secrets

```rust
use workspace_tools::workspace;
use secrecy::ExposeSecret;

fn main() -> workspace_tools::Result<()> {
  let ws = workspace()?;

  let secrets = ws.load_secrets_secure("-secrets.sh")?;
  let api_key = secrets.get("API_KEY").unwrap();

  // Explicit exposure required
  println!("Key: {}", api_key.expose_secret());

  Ok(())
}
```

### Pattern 5: Resource Discovery

```rust
use workspace_tools::workspace;

fn main() -> workspace_tools::Result<()> {
  let ws = workspace()?;

  let rust_files = ws.find_resources("src/**/*.rs")?;
  for file in rust_files {
    println!("{}", file.display());
  }

  Ok(())
}
```

### Pattern 6: Layered Configuration

```rust
use workspace_tools::workspace;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
  database_url: String,
  log_level: String,
}

fn main() -> workspace_tools::Result<()> {
  let ws = workspace()?;

  // Merge base.toml + dev.toml (later files override)
  let config: Config = ws.load_config_layered(&["base", "dev"])?;

  Ok(())
}
```

### Pattern 7: Test Workspace

```rust
#[cfg(test)]
mod tests {
  use workspace_tools::testing::create_test_workspace_with_structure;
  use std::fs;

  #[test]
  fn test_with_isolated_workspace() {
    let (_temp_dir, ws) = create_test_workspace_with_structure();

    fs::write(
      ws.config_dir().join("test.toml"),
      "[settings]\nenabled = true"
    ).unwrap();

    // Test with isolated workspace...
  }
}
```

### Pattern 8: Docker Deployment

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
ENV WORKSPACE_PATH=/app
COPY --from=builder /app/target/release/myapp .
COPY config/ ./config/
CMD ["./myapp"]
```

## Dependencies and Consumers

### Direct Dependencies

**Core (always):**
- `cargo_metadata` (workspace) - Cargo workspace detection
- `toml` (workspace, preserve_order) - TOML parsing

**Optional:**
- `glob` (workspace) - Pattern matching
- `tempfile` (workspace) - Temporary directories
- `serde` (workspace, derive) - Serialization framework
- `serde_json` (workspace) - JSON format
- `serde_yaml` (workspace) - YAML format
- `jsonschema` (0.20) - JSON Schema validation
- `schemars` (0.8) - Schema generation
- `secrecy` (0.8, serde) - Memory-safe secrets
- `zeroize` (1.7) - Memory zeroing

**Dev:**
- `tempfile` (workspace) - Testing

### Consumers (Workspace)

**Likely used by:**
- CLI applications requiring config
- Test suites needing fixtures
- Build tools
- Development utilities
- Any workspace-aware tooling

**Usage Pattern:** Applications use workspace_tools as the foundation for all workspace-relative operations, ensuring consistent behavior across development, testing, and deployment contexts.

## Design Rationale

### Why Multiple Resolution Strategies?

Fallback chain for reliability:

**Rationale:**
1. **Development**: Cargo workspace preferred
2. **CI/CD**: WORKSPACE_PATH explicit
3. **Installed Apps**: $PRO/$HOME fallback
4. **Robustness**: Always finds something

**Pattern:** Graceful degradation

### Why Standard Directories?

config/, data/, logs/, docs/, tests/:

**Rationale:**
1. **Convention**: Predictable structure
2. **Discovery**: Tools can find files
3. **Documentation**: Self-documenting layout
4. **Separation**: Clear concerns

**Pattern:** Convention over configuration

### Why Memory-Safe Secrets?

SecretString with zeroization:

**Rationale:**
1. **Security**: Prevents memory scanning
2. **Accidents**: Prevents accidental logging
3. **Explicit**: Requires expose_secret()
4. **Cleanup**: Zeroized on drop

**Pattern:** Defense in depth

### Why DRY Internal Helpers?

Refactored internal functions:

**Rationale:**
1. **Maintainability**: Single source of truth
2. **Consistency**: Same behavior everywhere
3. **Extensibility**: Easy to add formats
4. **Testing**: One place to test

**Metrics:** 127 lines eliminated, 60% complexity reduction

### Why Feature Gating?

Granular optional features:

**Rationale:**
1. **Compilation**: Only compile needed code
2. **Dependencies**: Minimize dep tree
3. **Binary Size**: Smaller for minimal use
4. **Choice**: Users pick what they need

**Pattern:** Opt-in capabilities

### Why Path Normalization?

Automatic path cleaning:

**Rationale:**
1. **Consistency**: Same path representation
2. **Comparison**: Paths compare correctly
3. **Errors**: Clear absolute paths
4. **Cross-Platform**: Works everywhere

**Example:** `/tmp/project/.` → `/tmp/project`

## Testing Strategy

### Test Coverage

**tempfile Available:**
- Isolated workspace testing
- No pollution of real workspace
- Automatic cleanup

### Test Focus

1. **Resolution**: All fallback strategies
2. **Path Operations**: join, normalization
3. **Config Loading**: All formats (TOML/JSON/YAML)
4. **Config Merging**: Layered configs
5. **Secret Loading**: KEY=VALUE parsing
6. **Secure Secrets**: SecretString wrapping
7. **Validation**: Schema validation
8. **Glob**: Pattern matching
9. **Error Handling**: All error variants
10. **Edge Cases**: Empty paths, missing files

### Known Test Limitations

1. **Cargo Detection**: Needs real Cargo.toml
2. **Env Vars**: May interfere with real env
3. **File Permissions**: Platform-dependent
4. **Secret Strength**: Validation rules may change

## Future Considerations

### Potential Enhancements

1. **Watch Mode**: File change detection
2. **Cloud Secrets**: AWS/GCP/Azure integration
3. **Encryption**: At-rest secret encryption
4. **Templates**: Mustache/Handlebars support
5. **Profiles**: dev/prod/test profiles
6. **Async**: Async file operations
7. **Remote Configs**: HTTP config loading
8. **Hot Reload**: Runtime config updates
9. **Audit Logging**: Secret access logging
10. **Multi-Workspace**: Nested workspace support

### Breaking Changes to Consider

1. **Error Type**: More structured errors
2. **Async API**: Async-first design
3. **Builder Pattern**: For configuration
4. **Trait Objects**: For extensibility
5. **Feature Reorganization**: Cleaner feature tree

### Known Limitations

1. **Single Workspace**: No nested workspace support
2. **Local Only**: No remote configs
3. **Sync I/O**: No async operations
4. **No Watch**: No file change detection
5. **No Encryption**: Memory protection only
6. **No Audit**: No secret access logging
7. **Schema Separate**: Validation schema must be provided

## Adoption Guidelines

### When to Use workspace_tools

**Good Candidates:**
- Rust workspaces with multiple crates
- Applications with configuration files
- Projects with secrets/credentials
- Tools needing consistent path resolution
- Test suites with fixtures
- Multi-environment deployments

**Poor Candidates:**
- Simple single-file projects
- Libraries without config needs
- Applications with cloud-native config
- Performance-critical path operations

### Best Practices

1. **Use workspace()**: Auto-detect over explicit
2. **Standard Dirs**: Use config_dir(), data_dir()
3. **Secure Secrets**: Use secure feature for credentials
4. **Feature Flags**: Enable only needed features
5. **Validation**: Use schema validation for safety
6. **Testing**: Use testing feature for isolation
7. **Docker**: Set WORKSPACE_PATH in container

### Integration Example

```rust
use workspace_tools::{workspace, SecretInjectable, WorkspaceError};
use serde::Deserialize;
use secrecy::ExposeSecret;

#[derive(Deserialize)]
struct DatabaseConfig {
  host: String,
  port: u16,
  username: String,
  #[serde(skip)]
  password: String,
}

impl SecretInjectable for DatabaseConfig {
  fn inject_secret(&mut self, key: &str, value: String) -> workspace_tools::Result<()> {
    match key {
      "DB_PASSWORD" => self.password = value,
      _ => return Err(WorkspaceError::SecretInjectionError(
        format!("unknown key: {}", key)
      )),
    }
    Ok(())
  }

  fn validate_secrets(&self) -> workspace_tools::Result<()> {
    if self.password.is_empty() {
      return Err(WorkspaceError::SecretValidationError(
        "password required".to_string()
      ));
    }
    Ok(())
  }
}

fn main() -> workspace_tools::Result<()> {
  let ws = workspace()?;

  // Load base config
  let mut config: DatabaseConfig = ws.load_config("database")?;

  // Inject secrets
  config = ws.load_config_with_secrets(config, "-secrets.sh")?;

  println!("Connecting to {}:{}", config.host, config.port);
  Ok(())
}
```

## Related Crates

**Dependencies:**
- **cargo_metadata**: Cargo workspace detection (external)
- **secrecy**: Memory-safe secret handling (external)
- **schemars**: JSON Schema generation (external)

**Related:**
- **pth**: General path utilities (workspace)
- **config**: General configuration (external)
- **dotenv**: Environment file loading (external)
- **directories**: Platform-specific dirs (external)

**Alternatives:**
- **config crate**: More general, less workspace-aware
- **dotenv**: CWD-relative, simpler
- **figment**: Configuration layering (external)

## References

- [API Documentation](https://docs.rs/workspace_tools)
- [Repository](https://github.com/Wandalen/workspace_tools)
- [Cargo Environment Variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html)
- [secrecy Crate](https://docs.rs/secrecy)
- [JSON Schema](https://json-schema.org/)
- [readme.md](./readme.md)
