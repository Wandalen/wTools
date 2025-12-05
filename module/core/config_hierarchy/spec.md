# config_hierarchy

- **Name:** config_hierarchy
- **Version:** 0.1.0
- **Purpose:** Generic hierarchical configuration management with multi-source resolution
- **Status:** Production-ready

## Goal

Provide a reusable, trait-based configuration framework for CLI applications that need to resolve settings from multiple sources with clear precedence rules, automatic type detection, source tracking, and validation.

## Responsibilities

### 1. Configuration Resolution
**Primary Concern:** Merge configuration values from all sources following priority hierarchy

- Resolve single parameter from all sources with precedence rules
- Resolve all parameters into complete configuration map
- Track source provenance for every resolved value (Runtime, Environment, Local, Global, Default)
- Handle missing/null values gracefully
- Support runtime parameter overrides

### 2. Persistence & I/O
**Primary Concern:** Store and retrieve configuration from filesystem

- Load YAML configuration files with error handling
- Save configuration with automatic metadata generation
- Delete configuration files when requested
- Preserve `created_at` timestamp across updates
- Update `last_modified` timestamp on each save
- Support atomic modifications via file locking

### 3. Path Discovery
**Primary Concern:** Locate configuration files across standard locations

- Resolve global configuration directory (`$PRO/{app}/` or OS-specific)
- Discover local configurations in current directory using dual-pattern support:
  - `-{app}/config.yaml` (temporary, higher priority within same directory)
  - `.{app}/config.yaml` (permanent, lower priority within same directory)
- Walk parent directories to find ancestor configurations (both patterns)
- Priority rule: Directory depth takes absolute precedence over pattern type
- Normalize paths across platforms
- Construct environment variable names (`{PREFIX}_{PARAM}`)

### 4. Validation
**Primary Concern:** Ensure configuration correctness

- Single-parameter validation (per-value type/range checks)
- Cross-parameter validation (relationship/dependency checks)
- Collect and report all validation errors
- Extensible validation via trait implementation
- Type-safe error reporting with parameter context

### 5. Type System
**Primary Concern:** Intelligent type conversion from strings

- Detect boolean values ("true"/"yes"/"1"/"on" → true)
- Detect integer values ("42", "-100")
- Detect floating-point values ("3.14", "1.23e-4")
- Fallback to string for unknown patterns
- Preserve type information through JSON values
- Support unicode and special characters

### 6. Concurrency Control
**Primary Concern:** Safe multi-process configuration access

- File-based advisory locking (fs2 crate)
- Atomic read-modify-write operations
- Safe concurrent reads from multiple processes
- Prevent data corruption during simultaneous writes
- Transaction-like modification support via `atomic_config_modify()`

## API

### Core Traits

Users implement these three traits to customize behavior:

```rust
pub trait ConfigDefaults
{
  fn get_defaults() -> HashMap< String, JsonValue >;
  fn get_parameter_names() -> Vec< &'static str >;
}

pub trait ConfigPaths
{
  fn app_name() -> &'static str;
  fn env_var_prefix() -> &'static str;
  fn local_config_dir_name() -> &'static str;
  fn global_config_subdirs() -> Vec< &'static str >;
}

pub trait ConfigValidator
{
  fn validate_parameter( param : &str, value : &JsonValue ) -> Result< (), ValidationError >;
  fn validate_all( config : &HashMap< String, ( JsonValue, ConfigSource ) > ) -> Vec< ValidationError >;
}
```

### Main Type

```rust
pub struct ConfigManager< D, P, V >
where
  D : ConfigDefaults,
  P : ConfigPaths,
  V : ConfigValidator;
```

### Primary Methods

**Resolution:**
- `resolve_config_value( param, runtime_params ) -> ( JsonValue, ConfigSource )`
- `resolve_all_config( runtime_params ) -> HashMap< String, ( JsonValue, ConfigSource ) >`

**File Operations:**
- `load_config_file( path ) -> Result< HashMap< String, JsonValue >, String >`
- `save_config_file( config, path ) -> Result< (), String >`
- `delete_config_file( path ) -> Result< bool, String >`

**Path Discovery:**
- `get_global_config_path() -> Result< PathBuf, String >`
- `discover_local_configs() -> Vec< PathBuf >`

**Atomic Operations:**
- `atomic_config_modify( path, modify_fn ) -> Result< (), String >`

**Global Configuration:**
- `save_global_config( config ) -> Result< (), String >`
- `delete_global_config() -> Result< bool, String >`

## Resolution Hierarchy

Priority from highest to lowest:

1. **Runtime** — Explicit parameters passed at execution (highest priority)
2. **Environment** — Environment variables matching `{PREFIX}_{PARAM}` pattern
3. **Local (Current)** — Config files in current working directory:
   - `-{app}/config.yaml` (temporary, higher priority)
   - `.{app}/config.yaml` (permanent, lower priority)
4. **Local (Parents)** — Config files in ancestor directories (nearest first):
   - `-{app}/config.yaml` (temporary, higher priority within same directory)
   - `.{app}/config.yaml` (permanent, lower priority within same directory)
   - Directory depth trumps pattern type (current `.{app}` beats parent `-{app}`)
5. **Global** — `$PRO/{app}/config.yaml` or OS-specific config directory
6. **Defaults** — Application-defined default values (lowest priority)

Higher priority sources override lower priority sources. Source tracking records which level provided each value.

## Type Detection Rules

Automatic string-to-type conversion:

| Pattern | Type | Examples |
|---------|------|----------|
| `true`, `yes`, `1`, `on` (case-insensitive) | Boolean true | "True", "YES", "On" |
| `false`, `no`, `0`, `off` (case-insensitive) | Boolean false | "False", "NO", "Off" |
| Integer pattern | Integer | "42", "-100", "999999999" |
| Float pattern | Float | "3.14", "-2.5", "1.23e-4" |
| Everything else | String | "hello", "2025-01-19", "🔥" |

## File Format

YAML with metadata section:

```yaml
metadata:
  version: "1.0"
  created_at: "2025-01-19T10:30:00Z"
  last_modified: "2025-01-19T12:45:00Z"

parameters:
  timeout: 60
  retries: 5
  debug: true
  api_key: "sk-abc123"
```

**Metadata Fields:**
- `version` — Format version (currently "1.0")
- `created_at` — ISO 8601 timestamp (preserved across updates)
- `last_modified` — ISO 8601 timestamp (updated on each save)

**Parameters Section:**
All configuration key-value pairs stored as YAML mapping.

## Design Principles

1. **Zero-cost abstractions** — PhantomData-based generics with no runtime overhead
2. **Trait-based customization** — Applications define behavior via three simple traits
3. **Fail-safe defaults** — Missing files/values handled gracefully
4. **Explicit source tracking** — Always know where each value came from
5. **Concurrent safety** — File locking prevents race conditions
6. **No mocking in tests** — 39 tests using real file I/O with tempfile

## Dependencies

| Crate | Purpose | Features |
|-------|---------|----------|
| serde | Serialization framework | derive |
| serde_json | JSON value type | - |
| serde_yaml | YAML parsing | - |
| chrono | Timestamp generation | clock |
| fs2 | File locking | - |
| tempfile | Test isolation (dev-only) | - |

## Test Coverage

- **Basic Operations:** 8 tests (load/save/delete, metadata)
- **Hierarchy Resolution:** 9 tests (priority, source tracking)
- **Type Detection:** 9 tests (bool/int/float/string, unicode)
- **Concurrent Access:** 2 tests (locking, multi-process)
- **Edge Cases:** 11 tests (corrupted files, special chars, unicode, long values)

**Total: 39 tests, 100% passing, zero mocking**

## Module Structure

```
src/
├── lib.rs              # Public API exports
├── error.rs            # ValidationError type
├── source.rs           # ConfigSource enum
├── traits.rs           # Core trait definitions
├── type_detection.rs   # String-to-type conversion
├── conversion.rs       # YAML/JSON utilities
├── path_discovery.rs   # Path resolution logic
├── file_ops.rs         # File I/O with locking
├── hierarchy.rs        # Multi-source resolution
└── manager.rs          # ConfigManager implementation

tests/
├── basic_operations_tests.rs
├── hierarchy_tests.rs
├── type_detection_tests.rs
├── concurrent_access_tests.rs
└── edge_cases_tests.rs
```

## Usage Pattern

1. Define three trait implementations (Defaults, Paths, Validator)
2. Create type alias: `type AppConfig = ConfigManager< D, P, V >;`
3. Use static methods: `AppConfig::resolve_all_config( runtime )`

See `readme.md` for complete working example.
