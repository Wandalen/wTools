# Format: Config File Format

### Scope

- **Purpose**: Specify the YAML structure that configuration files must conform to.
- **Responsibility**: Documents the data model, encoding structure, and format constraints for config files on disk.
- **In Scope**: YAML file structure with metadata section (auto-managed) and parameters section (user-defined key-value pairs).
- **Out of Scope**: Resolution order (→ invariant/001), path conventions (→ api/001), type conversion (→ algorithm/001).

### Abstract

Config files use YAML with two top-level sections: `metadata` (automatically managed) and `parameters` (user-defined key-value pairs). Only YAML is supported. The I/O feature must be activated for file reading and writing.

### Data Model

The format contains two top-level sections:

#### `metadata` section

Automatically generated and managed on every save. Do not edit manually unless migrating existing files.

| Field | Type | Purpose |
|-------|------|---------|
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

A valid config file in YAML encoding:

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

### Validation

The format is not strictly validated on read beyond what the YAML parser enforces. A file missing the `metadata` section is read successfully (metadata fields default to absent). A file missing the `parameters` section returns an empty map. Unknown top-level keys are ignored.

### Version Compatibility

The format version is recorded as `"1.0"` in the metadata section. Files missing the metadata section are read successfully, providing backward compatibility for files created without this section. Files missing the parameters section return an empty map. Unknown top-level keys are ignored, providing forward compatibility for future format extensions.

### Pitfall

Only YAML format is supported. Config files in other serialization formats (JSON, TOML) placed at discovered paths are not read. The filename used for discovery must match the format's expected extension.

### APIs

| File | Relationship |
|------|--------------|
| [api/001_config_paths_trait.md](../api/001_config_paths_trait.md) | Controls the filename and path used for config files |

### Features

| File | Relationship |
|------|--------------|
| [feature/001_config_hierarchy.md](../feature/001_config_hierarchy.md) | Feature that reads and writes files in this format |

### Invariants

| File | Relationship |
|------|--------------|
| [invariant/001_resolution_hierarchy.md](../invariant/001_resolution_hierarchy.md) | Invariant governing which files are loaded at each level |
