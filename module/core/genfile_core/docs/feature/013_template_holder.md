# Feature: Template Holder

### Scope

- **Purpose**: Provides a low-level generic template processor parameterized over value type and renderer.
- **Responsibility**: Documents the template holder struct and its composition model.
- **In Scope**: Combining file descriptors, parameters, values, renderer, and filesystem in one struct.
- **Out of Scope**: High-level archive API (→ 017), individual components (→ 003-012).

### Design

The template holder is the low-level orchestrator. It holds a list of file descriptors, a parameter collection, a value map, a renderer, and a file system. Generic type parameters enable compile-time specialization for specific value types, renderers, and file systems. Most consumers prefer the higher-level archive API; the template holder is for cases requiring custom types.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/template.rs` | Template holder struct definition and implementation |
| test | `tests/` | Template holder construction and composition tests |
| doc | `docs/feature/014_template_generation.md` | Generation operations on this struct |
