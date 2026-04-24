# Feature: File Descriptor

### Scope

- **Purpose**: Specifies a single output file — its path, content, and write strategy.
- **Responsibility**: Documents the `FileDescriptor` type and its fields.
- **In Scope**: Target path, template/static content flag, write mode, builder construction.
- **Out of Scope**: Write execution (→ 010, 011), write modes (→ 009).

### Design

A `FileDescriptor` pairs a relative output path with content (either a template string for rendering or static content to copy verbatim) and a `WriteMode` controlling how the file is written. The `is_template` flag determines whether the renderer is invoked. Descriptors are constructed via the `former` builder pattern.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/file_descriptor.rs` | `FileDescriptor` struct and builder |
| doc | `docs/feature/009_write_mode_support.md` | `WriteMode` enum referenced by this descriptor |
| doc | `docs/feature/014_template_generation.md` | Iterates descriptors during generation |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | FR8 in original spec; combined source migrated to feature/ |
