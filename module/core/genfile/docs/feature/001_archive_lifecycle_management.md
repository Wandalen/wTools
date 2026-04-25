# Feature: Archive Lifecycle Management

### Scope

- **Purpose**: Provides commands for creating, loading, saving, and building template archives.
- **Responsibility**: Documents the four archive lifecycle commands and their behaviors.
- **In Scope**: `.archive.new`, `.archive.load`, `.archive.save`, `.archive.from_directory`.
- **Out of Scope**: File content operations within an archive (→ 002), materialization (→ 006).

### Design

Archives can be created empty with a name and optional description, loaded from JSON or YAML files (format auto-detected by extension), saved back to JSON or YAML, or built from a filesystem directory. Directory scanning supports recursive traversal with optional include/exclude glob patterns. Inline and reference content modes control whether file content is embedded or stored as external references.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/handlers/archive.rs` | Handler implementations for all four commands |
| config | `commands/archive.yaml` | Authoritative command specs for archive group |
| doc | `docs/cli/commands/archive.md` | CLI reference documentation |

### Sources

| File | Notes |
|------|-------|
| `spec.md` (deleted) | FR1 in original spec; combined source migrated to feature/. spec.md has been deleted — Sources entry retained as migration record. |
