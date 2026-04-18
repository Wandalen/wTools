# Feature: HTML Rendering

### Scope

- **Purpose**: Document the `html_support` opt-in feature that adds `render_html()` to `DecoratedText` and `to_css()` to `Color`.
- **Responsibility**: Covers feature flag activation, HTML output format, the raw-vs-named color boundary, and CSS mapping strategy.
- **In Scope**: Feature flag, `render_html()` output contract, `to_css()` CSS mapping table, HTML escaping behavior, the raw-color no-span design decision.
- **Out of Scope**: CSS custom property definitions for `Ansi256` (caller's stylesheet); browser compatibility of CSS keywords; non-HTML markup targets (RTF, ANSI-to-HTML parsers).

### Design

`html_support` is a compile-time opt-in that implies `enabled`. When active, two additions become available:

1. **`DecoratedText::render_html()`** — produces browser-usable HTML output.
2. **`Color::to_css()`** — maps a `Color` variant to a CSS color value string.

**Activation:**

```toml
color_tools = { workspace = true, features = [ "html_support" ] }
```

#### `render_html()` Output Contract

| Input state | Output |
|-------------|--------|
| Plain text (no color) | HTML-escaped text, no wrapper element |
| Named-color text (`with_color_named`) | `<span style="color: {css}">escaped_text</span>` |
| Raw-string color (`with_color(raw)`) | Plain escaped text — no span produced |

**HTML escaping:** `&` → `&amp;`, `<` → `&lt;`, `>` → `&gt;`. Only these three characters are escaped; no quote escaping is needed inside the span's text content.

#### The Raw-Color Boundary

`with_color(raw_str)` stores an ANSI SGR byte sequence in the `color` field. Deriving CSS from raw ANSI bytes requires an ANSI parser — `render_html()` does not include one. This is a deliberate scope boundary:

- Use `with_color_named(Color)` when HTML output is required.
- Use `with_color(raw_str)` when terminal output only is required.
- The `named_color: Option<Color>` field (active only under `html_support`) carries the semantic color set by `with_color_named` so `render_html()` can produce the CSS span without any byte parsing.

#### `Color::to_css()` Mapping

| Variant | CSS value |
|---------|-----------|
| `Black` / `BrightBlack` | `black` |
| `Red` / `BrightRed` | `red` |
| `Green` / `BrightGreen` | `green` |
| `Yellow` / `BrightYellow` | `yellow` |
| `Blue` / `BrightBlue` | `blue` |
| `Magenta` / `BrightMagenta` | `magenta` |
| `Cyan` / `BrightCyan` | `cyan` |
| `White` / `BrightWhite` | `white` |
| `Rgb(r, g, b)` | `rgb(r, g, b)` |
| `Ansi256(n)` | `var(--ansi256-n)` |

Bright variants map to the same keyword as their normal counterpart — brightness distinction requires CSS that this crate does not generate. `Ansi256` maps to a CSS custom property; callers must define `--ansi256-N` in their stylesheet if they want actual color.

#### `named_color` Field

The `named_color: Option<Color>` field is present only when `html_support` is enabled. It is populated exclusively by `with_color_named(Color)`. Direct struct construction sets it to `None`; `with_color(raw_str)` leaves it as `None`.

The field is skipped during serde serialization (`#[serde(skip)]`) to preserve JSON schema stability across feature flag changes.

### Cross-References

| Entity | File | Relationship |
|--------|------|-------------|
| feature/001 | [DecoratedText](001_decorated_text.md) | Parent feature — base type this feature extends |
| api/001 | [DecoratedText Type](../api/001_decorated_text_type.md) | `render_html()` and `named_color` in the API reference |
