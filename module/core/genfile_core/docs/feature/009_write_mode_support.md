# Feature: Write Mode Support

### Scope

- **Purpose**: Controls how generated file content is written to disk.
- **Responsibility**: Documents the write mode type and both variants: Rewrite and TomlExtend.
- **In Scope**: Rewrite mode behavior (unconditional overwrite or create); TomlExtend mode behavior (smart TOML merge preserving comments and key order).
- **Out of Scope**: File system execution (→ 010, 011), file descriptor setup (→ 008).

### Design

The write mode is a discriminated type with two variants. The `Rewrite` variant unconditionally overwrites an existing file or creates a new one. The `TomlExtend` variant performs a smart merge with an existing TOML file: it preserves comments, formatting, and key order while updating values; new keys are appended at appropriate locations. `TomlExtend` is only valid for TOML output files. The write mode is stored inside the file descriptor and consumed by the file system layer during generation.

### Features

| File | Relationship |
|------|--------------|
| [feature/008_file_descriptor.md](008_file_descriptor.md) | Descriptor that carries the write mode |
| [feature/010_file_system_trait.md](010_file_system_trait.md) | Trait that executes the write |

### Sources

| File | Relationship |
|------|--------------|
| `src/file_descriptor.rs` | Write mode type definition |

### Tests

| File | Relationship |
|------|--------------|
| `tests/inc/file_descriptor_test.rs` | Write mode variant and behavior tests |
