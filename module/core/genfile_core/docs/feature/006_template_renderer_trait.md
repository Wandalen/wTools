# Feature: Template Renderer Trait

### Scope

- **Purpose**: Decouples template rendering from any specific engine via a pluggable trait.
- **Responsibility**: Documents the `TemplateRenderer` trait and its contract.
- **In Scope**: Trait definition, render method signature (conceptually), extensibility.
- **Out of Scope**: Handlebars implementation (→ 007), value serialization (→ 005).

### Design

The `TemplateRenderer` trait defines a `render` method accepting a template string and a serializable value map, returning a rendered string or an error. This abstraction allows consumers to swap rendering engines (e.g., Tera, Minijinja, custom) without changing calling code. The default implementation uses Handlebars; custom renderers implement the trait directly.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/renderer.rs` | `TemplateRenderer` trait definition |
| doc | `docs/feature/007_handlebars_renderer.md` | Default implementation of this trait |
| doc | `docs/feature/014_template_generation.md` | Consumes a renderer during generation |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | FR6 in original spec; combined source migrated to feature/ |
