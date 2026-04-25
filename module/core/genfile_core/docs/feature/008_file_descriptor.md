# Feature: File Descriptor

### Scope

- **Purpose**: Specifies a single output file — its path, content, and write strategy.
- **Responsibility**: Documents the file descriptor type and its fields.
- **In Scope**: Target path, template/static content flag, write mode, builder construction.
- **Out of Scope**: Write execution (→ 010, 011), write modes (→ 009).

### Design

A file descriptor pairs a relative output path with content (either a template string for rendering or static content to copy verbatim) and a write mode controlling how the file is written. The template flag determines whether the renderer is invoked. Descriptors are constructed via the former builder pattern.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/file_descriptor.rs` | File descriptor struct and builder |
| test | `tests/` | File descriptor construction and field tests |
| doc | `docs/feature/009_write_mode_support.md` | Write mode type referenced by this descriptor |
| doc | `docs/feature/014_template_generation.md` | Iterates descriptors during generation |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | FR8 in original spec; combined source migrated to feature/. spec.md has been deleted — Sources entry retained as migration record. |
