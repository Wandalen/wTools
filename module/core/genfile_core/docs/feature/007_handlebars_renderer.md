# Feature: Handlebars Renderer

### Scope

- **Purpose**: Provides the default template rendering implementation using the Handlebars engine.
- **Responsibility**: Documents `HandlebarsRenderer` and its rendering behavior.
- **In Scope**: Handlebars variable substitution, HTML escape disabled, supported template syntax.
- **Out of Scope**: Custom renderer implementations (→ 006), value serialization (→ 005).

### Design

`HandlebarsRenderer` wraps the Handlebars engine with HTML escaping disabled, enabling clean variable substitution in non-HTML templates (Rust code, YAML configs, etc.). It supports `{{variable}}` substitution and conditional blocks. Template compilation errors surface as typed `Error::Render` variants. The renderer holds no per-render state and can be reused across multiple render calls.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/renderer.rs` | `HandlebarsRenderer` implementation |
| doc | `docs/feature/006_template_renderer_trait.md` | Trait that this struct implements |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | FR7 in original spec; combined source migrated to feature/ |
