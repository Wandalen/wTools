# Feature: Write Mode Support

### Scope

- **Purpose**: Controls how generated file content is written to disk.
- **Responsibility**: Documents the write mode type and its variants.
- **In Scope**: Rewrite mode behavior (overwrite or create); extensibility for future modes.
- **Out of Scope**: File system execution (→ 010, 011), file descriptor setup (→ 008).

### Design

The write mode is a discriminated type where the rewrite variant unconditionally overwrites an existing file or creates a new one. It is stored inside the file descriptor and consumed by the file system layer during generation. Additional modes (e.g., merge, skip-if-exists) may be added in future versions.

### Features

| File | Relationship |
|------|--------------|
| [`feature/008_file_descriptor.md`](008_file_descriptor.md) | Descriptor that carries the write mode |
| [`feature/010_file_system_trait.md`](010_file_system_trait.md) | Trait that executes the write |

### Sources

| File | Relationship |
|------|--------------|
| [`src/file_descriptor.rs`](../../src/file_descriptor.rs) | Write mode type definition |

### Tests

| File | Relationship |
|------|--------------|
| [`tests/inc/file_descriptor_test.rs`](../../tests/inc/file_descriptor_test.rs) | Write mode behavior tests |
