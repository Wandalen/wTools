# Feature: Config Hierarchy

### Scope

- **Purpose**: Describe layered configuration resolution from multiple prioritized sources.
- **Responsibility**: Document the trait-based configuration framework and its resolution behavior.
- **In Scope**: Application startup and per-value config lookup; resolution, persistence, path discovery, validation, type detection, and concurrency control.
- **Out of Scope**: File format (→ format/001), resolution order invariant (→ invariant/001), trait API (→ api/).

### Design

Provides a reusable, trait-based configuration framework for CLI applications that need to resolve settings from multiple sources with clear precedence rules, automatic type detection, source tracking, and validation. Users implement three traits (`ConfigDefaults`, `ConfigPaths`, `ConfigValidator`) and compose a zero-cost `ConfigManager< D, P, V >` type from them. All methods on `ConfigManager` are static — the struct holds no data beyond `PhantomData` markers.

### Behavior

#### Configuration Resolution

- Resolves a single parameter from all sources following the 6-level priority hierarchy (see invariant/001)
- Resolves all parameters into a complete `HashMap< String, ( JsonValue, ConfigSource ) >`
- Tracks source provenance for every resolved value via `ConfigSource` enum
- Handles missing files and missing values gracefully without panicking
- Supports runtime parameter overrides as highest-priority input

#### Persistence and I/O

Requires `file_ops` feature.

- Loads YAML configuration files from discovered paths
- Saves configuration with automatic metadata generation (`created_at`, `last_modified`)
- Deletes configuration files when requested
- Supports atomic read-modify-write via `atomic_config_modify()`
- Uses `fs2` file locking to prevent concurrent write corruption

#### Path Discovery

- Resolves global configuration under `$PRO/.persistent/.{app_name}/config.yaml`
- Discovers local configurations using dual-pattern support:
  - `-{app_name}/{config_filename}` — temporary, gitignored (higher priority within same directory)
  - `.{app_name}/{config_filename}` — permanent, version-controlled (lower priority within same directory)
- Walks parent directories from CWD to filesystem root, nearest ancestor first
- Directory depth takes absolute precedence over pattern type

#### Validation

- Single-parameter validation via `ConfigValidator::validate_parameter()`
- Cross-parameter validation via `ConfigValidator::validate_all()`
- Both methods called independently; all errors collected before reporting
- `NoValidator` stub available for applications not needing validation

#### Type Detection

Automatic string-to-typed-value conversion applied to all env var and file values:

- Boolean: `"true"` / `"yes"` / `"1"` / `"on"` → `Bool(true)` (case-insensitive)
- Boolean: `"false"` / `"no"` / `"0"` / `"off"` → `Bool(false)` (case-insensitive)
- Integer: `"42"`, `"-100"` → `Number`
- Float: `"3.14"`, `"1.23e-4"` → `Number`
- Fallback: all other strings → `String`

#### Concurrency Control

Requires `file_ops` feature.

- File-based advisory locking via `fs2`
- `atomic_config_modify()` provides transaction-like read-modify-write
- Safe concurrent reads from multiple processes

### Security

`ConfigPaths::app_name()` is validated at runtime before any path construction:

- Must not be empty — prevents paths like `.//config.yaml`
- Must not contain `/` or `\` — prevents directory traversal
- Must not contain `..` — prevents path traversal attacks

Path discovery functions return `Err(String)` for invalid app names. `discover_local_configs()` silently skips invalid app names to avoid breaking the discovery loop.

**Recommended values**: alphanumeric characters, hyphens, underscores (`my-app`, `my_app_123`). Unicode is supported; whitespace should be avoided (works but causes shell issues).

### Rationale

1. **Zero-cost abstractions** — `PhantomData`-based generics; `ConfigManager` has no runtime storage
2. **Trait-based customization** — Applications control all behavior via three traits; the crate provides only wiring
3. **Fail-safe defaults** — Missing files and missing keys handled without error propagation
4. **Explicit source tracking** — Every resolved value carries its provenance for debugging
5. **No mocking in tests** — All 109 tests use real file I/O via `tempfile` crate

### Algorithms

| File | Relationship |
|------|--------------|
| [algorithm/001_type_detection.md](../algorithm/001_type_detection.md) | Type detection algorithm applied during resolution |

### APIs

| File | Relationship |
|------|--------------|
| [api/001_config_paths_trait.md](../api/001_config_paths_trait.md) | Required trait for path configuration |
| [api/002_config_defaults_trait.md](../api/002_config_defaults_trait.md) | Required trait for default values |
| [api/003_config_validator_trait.md](../api/003_config_validator_trait.md) | Optional trait for validation |

### Formats

| File | Relationship |
|------|--------------|
| [format/001_config_file_format.md](../format/001_config_file_format.md) | File format used by this feature |

### Invariants

| File | Relationship |
|------|--------------|
| [invariant/001_resolution_hierarchy.md](../invariant/001_resolution_hierarchy.md) | Resolution order this feature implements |

### Sources

| File | Relationship |
|------|--------------|
| [src/lib.rs](../../src/lib.rs) | Public API surface and module entry point |
| [src/traits.rs](../../src/traits.rs) | ConfigPaths, ConfigDefaults, ConfigValidator trait definitions |
| [src/hierarchy.rs](../../src/hierarchy.rs) | Resolution hierarchy and priority ordering implementation |
| [src/manager.rs](../../src/manager.rs) | Config manager type implementation |
| [src/path_discovery.rs](../../src/path_discovery.rs) | Path discovery and dual-pattern resolution |
| [src/source.rs](../../src/source.rs) | Source provenance tracking |
| [src/file_ops.rs](../../src/file_ops.rs) | Persistence and I/O operations |
| [src/type_detection.rs](../../src/type_detection.rs) | Type detection algorithm implementation |
| [src/conversion.rs](../../src/conversion.rs) | Value conversion utilities |
| [src/error.rs](../../src/error.rs) | Error type definitions |
| [src/display/mod.rs](../../src/display/mod.rs) | Display formatting entry point |

### Tests

| File | Relationship |
|------|--------------|
| [tests/feature_tests.rs](../../tests/feature_tests.rs) | End-to-end feature integration tests |
| [tests/hierarchy_tests.rs](../../tests/hierarchy_tests.rs) | Resolution priority ordering tests |
| [tests/basic_operations_tests.rs](../../tests/basic_operations_tests.rs) | Core operation tests |
| [tests/dual_pattern_tests.rs](../../tests/dual_pattern_tests.rs) | Dual-pattern rule and depth-beats-pattern tests |
| [tests/path_standards_tests.rs](../../tests/path_standards_tests.rs) | Path discovery standards tests |
| [tests/configurability_tests.rs](../../tests/configurability_tests.rs) | Trait customization tests |
| [tests/type_detection_tests.rs](../../tests/type_detection_tests.rs) | Type detection algorithm tests |
| [tests/concurrent_access_tests.rs](../../tests/concurrent_access_tests.rs) | Concurrent access tests |
| [tests/edge_cases_tests.rs](../../tests/edge_cases_tests.rs) | Edge case coverage |
| [tests/scope_operations_tests.rs](../../tests/scope_operations_tests.rs) | Scope operations tests |
| [tests/display_tests.rs](../../tests/display_tests.rs) | Display formatting tests |
