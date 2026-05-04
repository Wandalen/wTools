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

### Algorithms

| File | Relationship |
|------|--------------|
| [algorithm/001_type_detection.md](../algorithm/001_type_detection.md) | Type detection applied during resolution |

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
