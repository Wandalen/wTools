# Format: Config File Format

### Scope

- **What**: YAML structure for configuration files read and written by config_hierarchy
- **Who**: Users authoring config files; implementors parsing or generating them
- **When**: Writing any config file at paths discovered by path_discovery
- **Out of scope**: Resolution order (→ invariant/001), path conventions (→ api/001), type conversion (→ algorithm/001)

### Abstract

Config files use YAML with two top-level sections: `metadata` (automatically managed by the crate) and `parameters` (user-defined key-value pairs). Only YAML is supported — JSON and TOML are not valid config file formats for this crate. The `file_ops` feature must be enabled for file reading and writing.

### Structure

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

Automatically generated and managed by `save_config_file()` and `atomic_config_modify()`. Do not edit manually unless migrating existing files.

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

### Validation

The crate does not validate the file structure on read beyond what `serde_yaml` enforces. A file missing the `metadata` section is read successfully (metadata fields default to absent). A file missing the `parameters` section returns an empty map. Unknown top-level keys are ignored.

### Pitfall

**TOML and JSON are not supported as config file formats.** The `file_ops` feature uses `serde_yaml` exclusively. Placing a `config.json` or `config.toml` at a discovered path will not be read — only the filename returned by `ConfigPaths::local_config_filename()` (default `"config.yaml"`) is loaded.

### Cross-References

| Type | Target | Relationship |
|------|--------|-------------|
| doc | feature/001_config_hierarchy.md | file format used by this feature |
| doc | api/001_config_paths_trait.md | `local_config_filename()` controls the filename |
| doc | invariant/001_resolution_hierarchy.md | files at these paths feed into resolution |
