# Feature: Handlebars Renderer

### Scope

- **Purpose**: Provides the default template rendering implementation using the Handlebars engine.
- **Responsibility**: Documents the handlebars renderer and its rendering behavior.
- **In Scope**: Handlebars variable substitution, HTML escape disabled, supported template syntax.
- **Out of Scope**: Custom renderer implementations (→ 006), value serialization (→ 005).

### Design

The handlebars renderer wraps the Handlebars engine with HTML escaping disabled, enabling clean variable substitution in non-HTML templates (Rust code, YAML configs, etc.). It supports `{{variable}}` substitution and conditional blocks. Template compilation errors surface as typed render error variants. The renderer holds no per-render state and can be reused across multiple render calls.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/renderer.rs` | Handlebars renderer implementation |
| test | `tests/` | Handlebars rendering and template substitution tests |
| doc | `docs/feature/006_template_renderer_trait.md` | Trait that this renderer implements |
| doc | `docs/invariant/001_rendering_performance.md` | Performance bound that applies to this renderer |
