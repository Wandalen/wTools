# Feature: Template Holder

### Scope

- **Purpose**: Provides a low-level generic template processor parameterized over value type and renderer.
- **Responsibility**: Documents the template holder struct and its composition model.
- **In Scope**: Combining file descriptors, parameters, values, renderer, and filesystem in one struct.
- **Out of Scope**: High-level archive API (→ 017), individual components (→ 003-012).

### Design

The template holder is the low-level orchestrator. It holds a list of file descriptors, a parameter collection, a value map, a renderer, and a file system. Generic type parameters enable compile-time specialization for specific value types, renderers, and file systems. Most consumers prefer the higher-level archive API; the template holder is for cases requiring custom types.

### Features

| File | Relationship |
|------|--------------|
| [`feature/014_template_generation.md`](014_template_generation.md) | Generation operations on this struct |

### Sources

| File | Relationship |
|------|--------------|
| [`src/template.rs`](../../src/template.rs) | Template holder struct definition and implementation |

### Tests

| File | Relationship |
|------|--------------|
| [`tests/inc/template_test.rs`](../../tests/inc/template_test.rs) | Template holder construction and composition tests |
