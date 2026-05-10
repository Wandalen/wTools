# Feature: Archive Lifecycle Management

### Scope

- **Purpose**: Provides commands for creating, loading, saving, and building template archives.
- **Responsibility**: Documents the four archive lifecycle commands and their behaviors.
- **In Scope**: `.archive.new`, `.archive.load`, `.archive.save`, `.archive.from_directory`.
- **Out of Scope**: File content operations within an archive (→ 002), materialization (→ 006).

### Design

Archives can be created empty with a name and optional description, loaded from JSON or YAML files (format auto-detected by extension), saved back to JSON or YAML, or built from a filesystem directory. Directory scanning supports recursive traversal with optional include/exclude glob patterns. Inline and reference content modes control whether file content is embedded or stored as external references.

### Features

| File | Relationship |
|------|--------------|
| [`feature/002_file_content_operations.md`](002_file_content_operations.md) | File operations performed on a loaded archive |

### Docs

| File | Relationship |
|------|--------------|
| [`docs/cli/command/archive.md`](../cli/command/archive.md) | CLI reference documentation |

### Sources

| File | Relationship |
|------|--------------|
| [`src/handlers/archive.rs`](../../src/handlers/archive.rs) | Handler implementations for all four commands |

### Tests

| File | Relationship |
|------|--------------|
| [`tests/archive_commands_test.rs`](../../tests/archive_commands_test.rs) | Integration tests for archive lifecycle commands |
