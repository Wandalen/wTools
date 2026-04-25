# Feature: Config Hierarchy

### Scope

- **What**: Layered configuration resolution from multiple prioritized sources
- **Who**: CLI applications using `ConfigManager< D, P, V >`
- **When**: Application startup and per-value config lookup
- **Out of scope**: File format (→ format/001), resolution order invariant (→ invariant/001), trait API (→ api/)

### Abstract

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

### Cross-References

| Type | File | Relationship |
|------|------|-------------|
| invariant | invariant/001_resolution_hierarchy.md | resolution order this feature implements |
| api | api/001_config_paths_trait.md | required trait for path configuration |
| api | api/002_config_defaults_trait.md | required trait for default values |
| api | api/003_config_validator_trait.md | optional trait for validation |
| format | format/001_config_file_format.md | file format used by this feature |
| algorithm | algorithm/001_type_detection.md | type detection algorithm applied during resolution |
