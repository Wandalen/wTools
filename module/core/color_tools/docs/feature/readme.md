# Feature Doc Entity

### Scope

- **Purpose**: Document features of `color_tools` — what the crate does and why each feature exists.
- **Responsibility**: Collect one doc instance per user-visible feature; each instance owns scope, design, and cross-references.
- **In Scope**: Feature specifications — scope, design decisions, constraints, and integration points.
- **Out of Scope**: Behavioral contracts (→ `invariant/`); API signatures (→ `api/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [DecoratedText](001_decorated_text.md) | Typed text wrapper with optional ANSI color prefix | ✅ |
| 002 | [serde Support](002_serde_support.md) | Opt-in Serialize/Deserialize for DecoratedText | ✅ |
| 003 | [HTML Rendering](003_html_rendering.md) | Opt-in render_html() and Color::to_css() via html_support | ✅ |
