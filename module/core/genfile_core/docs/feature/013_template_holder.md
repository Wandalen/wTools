# Feature: Template Holder

### Scope

- **Purpose**: Provides a low-level generic template processor parameterized over value type and renderer.
- **Responsibility**: Documents the `Template<V, R, FS>` struct and its composition model.
- **In Scope**: Combining file descriptors, parameters, values, renderer, and filesystem in one struct.
- **Out of Scope**: High-level archive API (→ `TemplateArchive`), individual components (→ 003-012).

### Design

`Template<V, R, FS>` is the low-level orchestrator. It holds a list of `FileDescriptor` instances, a `Parameters` collection, a `Values<V>` map, a renderer of type `R`, and a filesystem of type `FS`. Generic type parameters enable compile-time specialization for specific value types, renderers, and filesystems. Most consumers prefer the higher-level `TemplateArchive` API; `Template` is for cases requiring custom types.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/template.rs` | `Template` struct definition and implementation |
| doc | `docs/feature/014_template_generation.md` | Generation method on this struct |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | FR13 in original spec; combined source migrated to feature/ |
