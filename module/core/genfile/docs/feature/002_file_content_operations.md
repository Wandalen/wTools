# Feature: File Content Operations

### Scope

- **Purpose**: Provides commands to manage template files inside a loaded archive.
- **Responsibility**: Documents the four file operation commands.
- **In Scope**: `.file.add`, `.file.remove`, `.file.list`, `.file.show`.
- **Out of Scope**: Archive lifecycle (→ 001), parameter management (→ 003).

### Design

Files can be added as text templates or binary content, either inline or from an external source path. Removal is by archive-internal path. Listing supports verbosity levels to show file metadata (size, type). Content display shows text files directly and indicates binary files by type. Binary files are safely embedded within the archive serialization format.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/handlers/file.rs` | Handler implementations for file commands |
| config | `commands/file.yaml` | Authoritative command specs for file group |
| test | `tests/file_commands_test.rs` | Integration tests for file operation commands |
