# Feature: Write Mode Support

### Scope

- **Purpose**: Controls how generated file content is written to disk.
- **Responsibility**: Documents the `WriteMode` enum and its variants.
- **In Scope**: Rewrite mode behavior (overwrite or create); extensibility for future modes.
- **Out of Scope**: File system execution (→ 010, 011), file descriptor setup (→ 008).

### Design

`WriteMode` is an enum where the `Rewrite` variant unconditionally overwrites an existing file or creates a new one. It is stored inside `FileDescriptor` and consumed by the file system layer during generation. Additional modes (e.g., merge, skip-if-exists) may be added in future versions.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/file_descriptor.rs` | `WriteMode` enum definition |
| doc | `docs/feature/008_file_descriptor.md` | Descriptor that carries the write mode |
| doc | `docs/feature/010_file_system_trait.md` | Trait that executes the write |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | FR9 in original spec; combined source migrated to feature/ |
