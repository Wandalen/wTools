# Feature: Archive Serialization

### Scope

- **Purpose**: Provides the `.pack` command to create fully portable, self-contained archive files.
- **Responsibility**: Documents the pack operation: internalizing references and writing to file.
- **In Scope**: Packing from an existing archive or directory, JSON/YAML output, portability verification.
- **Out of Scope**: Archive loading/saving without internalization (→ 001), materialization (→ 006).

### Design

The pack command internalizes all external content references (file refs, URL refs) into the archive before saving. Input can be an existing archive file or a directory (which is first scanned as a new archive). Output format is auto-detected from the file extension or specified explicitly. The result is a single portable file containing all templates, parameters, values, and metadata with no external dependencies.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/handlers/pack.rs` | Handler implementation for `.pack` command |
| config | `commands/pack.yaml` | Authoritative command spec |
| test | `tests/archive_commands_test.rs` | Integration tests covering pack command |
