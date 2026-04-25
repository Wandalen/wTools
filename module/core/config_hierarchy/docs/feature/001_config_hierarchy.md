# Feature: Config Hierarchy

### Scope

- **Purpose**: Specify the layered configuration resolution system for CLI applications.
- **Responsibility**: Documents the resolution flow, path discovery rules, persistence model, and validation integration of the configuration framework.
- **In Scope**: Multi-source resolution with priority ordering, path discovery, file persistence, type detection, validation hooks, and source tracking.
- **Out of Scope**: File format (→ format/001), resolution order invariant (→ invariant/001), trait contracts (→ api/).

### Abstract

Provides a reusable, trait-based configuration framework for CLI applications that resolve settings from multiple sources with clear precedence rules, automatic type detection, source tracking, and validation. Applications implement three configuration traits and compose a manager type from them.

### Design

#### Resolution

Resolves a single parameter or the complete parameter set from all sources following the 6-level priority hierarchy (see invariant/001). Tracks source provenance for every resolved value. Handles missing files and missing values gracefully. Supports runtime parameter overrides as the highest-priority input.

#### Path Discovery

- Global configuration resolves under `$PRO/.persistent/.{app_name}/config.yaml`
- Local configurations are discovered using dual-pattern support:
  - `-{app_name}/{config_filename}` — temporary, gitignored (higher priority within same directory)
  - `.{app_name}/{config_filename}` — permanent, version-controlled (lower priority within same directory)
- Walks parent directories from CWD to filesystem root, nearest ancestor first
- Directory depth takes absolute precedence over pattern type

#### Persistence and I/O

Requires the I/O feature to be activated.

- Loads configuration files from discovered paths
- Saves configuration with automatic metadata generation
- Supports atomic read-modify-write operations
- Uses file locking to prevent concurrent write corruption

#### Type Detection

Automatic string-to-typed-value conversion is applied to all environment variable and file values. See algorithm/001 for the complete detection algorithm.

#### Validation

Per-parameter and cross-parameter validation hooks are called after resolution. All errors are collected before reporting. Applications not requiring validation use the provided no-op implementation.

#### Security

The application name is validated before any path construction:
- Must not be empty
- Must not contain path separator characters
- Must not contain path traversal sequences

Path discovery returns an error for invalid application names.

### Key Design Decisions

- All manager methods are stateless — the manager type holds no runtime data beyond type markers
- Missing files and missing keys are handled without error propagation
- Every resolved value carries source provenance for debugging

### Cross-References

| Type | File                                  | Responsibility                                  |
|------|---------------------------------------|-------------------------------------------------|
| doc  | invariant/001_resolution_hierarchy.md | Resolution order this feature implements        |
| doc  | api/001_config_paths_trait.md         | Required trait for path configuration           |
| doc  | api/002_config_defaults_trait.md      | Required trait for default values               |
| doc  | api/003_config_validator_trait.md     | Optional trait for validation                   |
| doc  | format/001_config_file_format.md      | File format used by this feature                |
| doc  | algorithm/001_type_detection.md       | Type detection applied during resolution        |
