# Feature: File Descriptor

### Scope

- **Purpose**: Specifies a single output file — its path, content, and write strategy.
- **Responsibility**: Documents the file descriptor type and its fields.
- **In Scope**: Target path, template/static content flag, write mode, builder construction.
- **Out of Scope**: Write execution (→ 010, 011), write modes (→ 009).

### Design

A file descriptor pairs a relative output path with content (either a template string for rendering or static content to copy verbatim) and a write mode controlling how the file is written. The template flag determines whether the renderer is invoked. Descriptors are constructed via the former builder pattern.

### Features

| File | Relationship |
|------|--------------|
| [feature/009_write_mode_support.md](009_write_mode_support.md) | Write mode type referenced by this descriptor |
| [feature/014_template_generation.md](014_template_generation.md) | Iterates descriptors during generation |

### Sources

| File | Relationship |
|------|--------------|
| `src/file_descriptor.rs` | File descriptor struct and builder |

### Tests

| File | Relationship |
|------|--------------|
| `tests/inc/file_descriptor_test.rs` | File descriptor construction and field tests |
