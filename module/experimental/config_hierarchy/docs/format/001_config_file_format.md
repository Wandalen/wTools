# Format: Config File Format

### Scope

- **Purpose**: Document the YAML structure for configuration files read and written by config_hierarchy.
- **Responsibility**: Define the file schema, valid field values, and validation behavior.
- **In Scope**: Writing any config file at paths discovered by path discovery; metadata and parameters sections.
- **Out of Scope**: Resolution order (→ invariant/001), path conventions (→ api/001), type conversion (→ algorithm/001).

### Abstract

Config files use YAML with two top-level sections: `metadata` (automatically managed by the crate) and `parameters` (user-defined key-value pairs). Only YAML is supported — JSON and TOML are not valid config file formats for this crate. The `file_ops` feature must be enabled for file reading and writing; YAML parsing is performed by the underlying YAML library layer.

### Data Model

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

### Fields

#### `metadata` section

Automatically generated and managed by the save and atomic-modify operations. Do not edit manually unless migrating existing files.

| Field | Type | Description |
|-------|------|-------------|
| `version` | string | Format version, always `"1.0"` |
| `created_at` | ISO 8601 string | Timestamp set on first save, preserved on all subsequent saves |
| `last_modified` | ISO 8601 string | Timestamp updated on every save |

#### `parameters` section

User-defined configuration key-value pairs. Keys are strings; values may be any YAML scalar:
- Boolean: `true`, `false`
- Integer: `60`, `-1`
- Float: `3.14`
- String: `"production"`, `hello` (quoted or unquoted)
- Null: `~` or `null`

YAML sequences and mappings are not supported as parameter values — only scalars.

### Encoding Structure

- Encoding: UTF-8, no BOM
- Line endings: LF (`\n`)
- Indentation: 2-space YAML indentation
- Top-level keys: exactly two — `metadata` and `parameters`; additional top-level keys are ignored on read
- Scalar values only in `parameters` — YAML sequences and mappings are not supported as parameter values

### Version Compatibility

- `version: "1.0"` is the only supported format version
- Files lacking a `metadata` section are read as legacy flat format — all top-level keys are treated as parameters
- Files lacking a `parameters` section return an empty parameter map
- Unknown top-level keys are silently ignored, preserving forward compatibility
- The `created_at` field is preserved verbatim on every save — the write path reads the existing value before overwriting

### Validation

The crate does not validate the file structure on read beyond what the YAML parser enforces. A file missing the `metadata` section is read successfully (metadata fields default to absent). A file missing the `parameters` section returns an empty map. Unknown top-level keys are ignored.

### Pitfall

**TOML and JSON are not supported as config file formats.** The `file_ops` feature uses YAML exclusively. Placing a `config.json` or `config.toml` at a discovered path will not be read — only the filename returned by `ConfigPaths::local_config_filename()` (default `"config.yaml"`) is loaded.

### APIs

| File | Relationship |
|------|--------------|
| [api/001_config_paths_trait.md](../api/001_config_paths_trait.md) | local_config_filename() controls the filename |

### Features

| File | Relationship |
|------|--------------|
| [feature/001_config_hierarchy.md](../feature/001_config_hierarchy.md) | Feature that uses this file format |

### Invariants

| File | Relationship |
|------|--------------|
| [invariant/001_resolution_hierarchy.md](../invariant/001_resolution_hierarchy.md) | Files at these paths feed into resolution |
| [invariant/002_file_persistence_contracts.md](../invariant/002_file_persistence_contracts.md) | Contracts governing write operations on this format |

### Sources

| File | Relationship |
|------|--------------|
| [src/file_ops.rs](../../src/file_ops.rs) | YAML read/write, metadata handling, atomic save |
| [src/conversion.rs](../../src/conversion.rs) | YAML-to-JSON and JSON-to-YAML conversion |

### Tests

| File | Relationship |
|------|--------------|
| [tests/basic_operations_tests.rs](../../tests/basic_operations_tests.rs) | File load/save round-trip tests |
| [tests/edge_cases_tests.rs](../../tests/edge_cases_tests.rs) | Legacy format, metadata preservation, missing-section cases |
